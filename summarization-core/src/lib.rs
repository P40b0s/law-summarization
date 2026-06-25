mod logger;
mod database;
mod recognition;
mod publication_service;
mod configuration;
mod ai_service;
mod scheduler;

use std::{collections::HashMap, sync::Arc, time::Duration};

use anyhow::anyhow;
use chrono::Days;
pub use database::{DbCommand, start_database_service};
pub use shared::Document;
pub use configuration::CoreConfiguration;
use tokio::sync::oneshot;
use publication_client::{PublicationDocumentCard, ReqwestPublicationApiClient};
pub use publication_service::PublicationService;
pub use ai_service::{AiService, ChatCompletionRequest, MessageWithContent};
pub use scheduler::Scheduler;
use tokio::time::Instant;
use tracing::{error, info};
use utilites::Date;

use crate::scheduler::ShedulerNew;

pub struct SummarizationService
{
    pub publication_service: PublicationService<ReqwestPublicationApiClient>,
    databse_service: tokio::sync::mpsc::Sender<DbCommand>,
    ai_service: AiService,
    configuration: Arc<CoreConfiguration>

}

impl SummarizationService
{
    pub fn new(config: Arc<CoreConfiguration>, model: &str) -> Self
    {
        let client  = ReqwestPublicationApiClient::new();
        let publication_service = PublicationService::new(client, config.clone());
        let ai_service = ai_service::AiService::new(model.to_owned(), config.clone());
        let (db_sender, db_receiver) = tokio::sync::mpsc::channel(16);
        start_database_service(db_receiver);
        Self
        {
            publication_service,
            databse_service: db_sender,
            ai_service,
            configuration: config
        }
    }

    pub async fn start_service(&self)
    {
        let mut interval = tokio::time::interval(Duration::from_mins(self.configuration.check_period_min as u64));
        interval.set_missed_tick_behavior(tokio::time::MissedTickBehavior::Skip);
        let _ = self.processig_documents_for_dates_range(self.configuration.retro_days_period).await
            .inspect_err(|e| error!("{}", e));
        loop 
        {
            let date = Date::now();
            let documents = self.get_documents_for_date(date).await
                .inspect_err(|e| error!("{e}"))
                .unwrap_or(Vec::new());
            for doc in documents
            {
                let _ = self.process_document(doc).await
                    .inspect_err(|e| error!("{e}"));
            }
            interval.tick().await;
        }

    }
    async fn processig_documents_for_dates_range(&self, retro_checked_days: usize) -> anyhow::Result<()>
    {
        let today = chrono::Local::now().date_naive();
        let start_date = chrono::Local::now().checked_sub_days(Days::new(retro_checked_days as u64)).unwrap().date_naive();
        let days_count = (today - start_date).num_days() as usize + 1;
        let mut dates = Vec::with_capacity(days_count);
        let mut current = start_date;
        for _ in 0..days_count 
        {
            dates.push(Date::parse(current.format("%Y-%m-%d").to_string()).unwrap());
            current += chrono::Duration::days(1);
        };
        info!("Проверка данных за последние {} дней", days_count);
        for date in dates
        {
            let documents = self.get_documents_for_date(date).await?;
            for doc in documents
            {
                let _ = self.process_document(doc).await?;
            }
        };
        Ok(())
    }

    async fn get_documents_for_date(&self, date: Date) -> anyhow::Result<Vec<PublicationDocumentCard>>
    {
        let result = self.publication_service.search_documents(&date).await?;
        let mut for_processing = Vec::with_capacity(result.len());
        for card in result
        {
            let (db_result_sender, db_result_receiver) = oneshot::channel();
            let _ = self.databse_service.send(DbCommand::GetDocument { eo_number: card.eo_number.clone(), respond: db_result_sender }).await?;
            match db_result_receiver.await?
            {
                Ok(Some(doc)) => 
                {
                    // document already exists in DB, skip
                    info!("Document {} already exists in DB, skipping", doc.eo_number);
                }
                Ok(None) => 
                {
                    // document not in DB, process it
                    //info!("Document {} add to processing query", card.eo_number);
                    for_processing.push(card);
                }
                Err(e) => 
                {
                    error!("Failed to query DB for document {}: {}", card.eo_number, e);
                }
            }
        }
        Ok(for_processing)
    }

    async fn process_document(&self, card: PublicationDocumentCard) -> anyhow::Result<()>
    {
        info!("Processing new document {}", card.eo_number);
        let mut texts = String::new();
        for page in 1..=card.pages_count
        {
            let png = self.publication_service.get_png(&card.id, page).await?;
            let image = recognition::bytes_to_base64url(&png);
            let text = self.ai_service.recognize_image(&image, "Распознай весь текст с этого изображения. Запиши его в markdown формате, используй заголовки списки и таблицы, вместо '\n' используй кодировку юникода U+000A, без комментариев, закончи, когда текст на изображении кончится", Some(0.0)).await.unwrap();
            texts.push_str(&text);
            info!("Fetched PNG for document {} page {}: {} bytes", card.id, page, png.len());
        }
        let command = format!("Составь краткое содержание этого документа в 2-6 предложениях, начинай в подобном формате (например если федеральный закон): \"Федеральный закон № 133-ФЗ от 25 мая 2026 года... и дальше содержание документа\", не дополняй кем подписан документ и где, так же не уточняй кем принят или одобрен, нужно только краткое содержание текста. Делай на основе следующего текста: {}", texts);
        let message = MessageWithContent
        {
            role: "user".to_owned(),
            content: vec![serde_json::json!({
                "type": "text",
                "text": command
            })]
        };
        let summarization = self.ai_service.chat(vec![message], Some(0.0)).await?;
        let (db_result_sender, db_result_receiver) = oneshot::channel();
        self.databse_service.send(DbCommand::InsertDocument 
        { 
            doc_id: card.id.clone(),
            publication_date: card.publish_date_short,
            eo_number: card.eo_number,
            complex_name: card.complex_name,
            summary: Some(summarization),
            respond: db_result_sender,
            pages_count: card.pages_count as i32
        }).await?;
        match db_result_receiver.await?
        {
            Ok(_) => 
            {
                info!("Document {} saved to DB", card.id);
               Ok(())
            },
            Err(e) => 
            {
                error!("Failed to save document {} to DB: {}", card.id, e);
                Err( anyhow!("Ошибка `{}` при добавлении документа {} в базу данных!", e, card.id))
            }
        }
    }
    pub async fn get_calendar_state(&self, date_from: Date, date_to: Date) -> anyhow::Result<shared::CalendarResponse>
    {
        let (db_result_sender, db_result_receiver) = oneshot::channel();
        self.databse_service.send(DbCommand::GetCalendarState { date_from, date_to, respond: db_result_sender }).await?;
        match db_result_receiver.await?
        {
            Ok(result) => 
            {
                let result: HashMap<String, shared::DateState> = result.into_iter().map(|s| 
                    (s.publication_date, 
                        shared::DateState 
                        { 
                            checked: s.checked_count as i32,
                            unloaded: s.unloaded_count as i32,
                            count: s.total_count as i32
                        }
                    )
                ).collect();
               Ok(shared::CalendarResponse { dates: result })
            },
            Err(e) => 
            {
                error!("Ошибка `{}` при получении статуса календаря", e);
                Err( anyhow!("Ошибка `{}` при получении статуса календаря", e))
            }
        }
    }
    pub async fn get_documents_by_publication_date(&self, date: Date) -> anyhow::Result<Vec<shared::Document>>
    {
        let (db_result_sender, db_result_receiver) = oneshot::channel();
        self.databse_service.send(DbCommand::GetDocuments { publication_date: date, respond: db_result_sender }).await?;
        match db_result_receiver.await?
        {
            Ok(result) => 
            {
               
               Ok(result)
            },
            Err(e) => 
            {
                error!("Ошибка `{}` при получении списка документов", e);
                Err( anyhow!("Ошибка `{}` при получении списка документов", e))
            }
        }
    }

    pub async fn update_document(&self, doc: Document) -> anyhow::Result<shared::CalendarResponse>
    {
        let (db_result_sender, db_result_receiver) = oneshot::channel();
        self.databse_service.send(DbCommand::UpdateDocument { doc_id: doc.doc_id.clone(), summary: doc.summarization_text, checked_time: doc.checked_time, unloaded: doc.unloaded, respond: db_result_sender }).await?;
        match db_result_receiver.await?
        {
            Ok(_) => 
            {
               let result = self.get_calendar_state(doc.publication_date.clone(), doc.publication_date.clone()).await;
               result
            },
            Err(e) => 
            {
                error!("Ошибка `{}` при обновлении документа {}", e, doc.doc_id);
                Err( anyhow!("Ошибка `{}` при обновлении документа {}", e, doc.doc_id))
            }
        }
    }

}


pub async fn run_service(config: Arc<CoreConfiguration>, db_sender: tokio::sync::mpsc::Sender<DbCommand>) -> anyhow::Result<()>
{
    //let (db_sender, db_receiver) = tokio::sync::mpsc::channel(16);
    //let _db_service = start_database_service(db_receiver).await;
    let client  = ReqwestPublicationApiClient::new();
    let publication_service = PublicationService::new(client, config.clone());
    let ai_service = ai_service::AiService::new("qwen3.6".to_owned(), config.clone());
    loop 
    {
        let date = Date::now();
        let result = publication_service.search_documents(&date).await?;
        for card in result
        {
            let (db_result_sender, db_result_receiver) = oneshot::channel();
            let _db_doc = db_sender.send(DbCommand::GetDocument { eo_number: card.eo_number.clone(), respond: db_result_sender }).await?;
            match db_result_receiver.await?
            {
                Ok(Some(doc)) => 
                {
                    // document already exists in DB, skip
                    info!("Document {} already exists in DB, skipping", doc.eo_number);
                }
                Ok(None) => 
                {
                    // document not in DB, process it
                    info!("Processing new document {}", card.eo_number);
                    let mut texts = String::new();
                    for page in 1..=card.pages_count
                    {
                        let png = publication_service.get_png(&card.id, page).await?;
                        let image = recognition::bytes_to_base64url(&png);
                        let text = ai_service.recognize_image(&image, "Распознай весь текст с этого изображения. Запиши его в markdown формате, используй заголовки списки и таблицы, вместо '\n' используй кодировку юникода U+000A, без комментариев, закончи, когда текст на изображении кончится", Some(0.0)).await.unwrap();
                        texts.push_str(&text);
                        info!("Fetched PNG for document {} page {}: {} bytes", card.id, page, png.len());
                    }
                    let command = format!("Составь краткое содержание этого документа в 2-6 предложениях, начинай в подобном формате (например если федеральный закон): \"Федеральный закон № 133-ФЗ от 25 мая 2026 года... и дальше содержание документа\", не дополняй кем подписан документ и где, так же не уточняй кем принят или одобрен, нужно только краткое содержание текста. Делай на основе следующего текста: {}", texts);
                    let message = MessageWithContent
                    {
                        role: "user".to_owned(),
                        content: vec![serde_json::json!({
                            "type": "text",
                            "text": command
                        })]
                    };
                    let summarization = ai_service.chat(vec![message], Some(0.0)).await.unwrap();
                    let (db_result_sender, db_result_receiver) = oneshot::channel();
                    db_sender.send(DbCommand::InsertDocument 
                    { 
                        doc_id: card.id.clone(),
                        publication_date: card.publish_date_short,
                        eo_number: card.eo_number,
                        complex_name: card.complex_name,
                        summary: Some(summarization),
                        respond: db_result_sender,
                        pages_count: card.pages_count as i32
                    }).await?;
                    match db_result_receiver.await?
                    {
                        Ok(_) => info!("Document {} saved to DB", card.id),
                        Err(e) => error!("Failed to save document {} to DB: {}", card.id, e),
                    }
                }
                Err(e) => 
                {
                    error!("Failed to query DB for document {}: {}", card.eo_number, e);
                }
            }
            
        }
        ShedulerNew::start(Duration::from_mins(config.check_period_min as u64)).await?;    
    }
}


mod tests
{
    use super::*;
    use crate::{ai_service::MessageWithContent, configuration::CoreConfiguration};
    use crate::publication_service::PublicationService;
    use arc_swap::ArcSwap;
    use publication_client::{ReqwestPublicationApiClient, PublicationDocumentCard};
    use tracing::info;
use utilites::Date;
    use std::sync::Arc;



    
    
    
    
    
    
    
    
    
    
    
    
    
    
    
    
    
    
    
    
    
    
    
    
    
    
    
    
    
    
    
    
    
    
    #[tokio::test]
    async fn test_recognition()
    {
        crate::logger::init();
        let mock_client = ReqwestPublicationApiClient::new();
        let config = ArcSwap::new(Arc::new(CoreConfiguration::default()));
        let service = PublicationService::new(mock_client, config.load().clone());
        let date = Date::parse("24-06-2026").unwrap();
        let result = service.search_documents(&date).await.unwrap();
        info!("Fetched {} documents for date {}", result.len(), date);
        let ai_service = ai_service::AiService::new("qwen3.6".to_owned(), config.load().clone());
        for doc in result
        {
            let mut texts = String::new();
            for page in 1..=doc.pages_count
            {
                let png = service.get_png(&doc.id, page).await.unwrap();
                let image = recognition::bytes_to_base64url(&png);
                let text = ai_service.recognize_image(&image, "Распознай весь текст с этого изображения. Запиши его в markdown формате, используй заголовки списки и таблицы, вместо '\n' используй кодировку юникода U+000A, без комментариев, закончи, когда текст на изображении кончится", Some(0.0)).await.unwrap();
                std::fs::write(format!("document_{}_page_{}.png", doc.number, page), &png).unwrap();
                std::fs::write(format!("document_{}_page_{}.md", doc.number, page), &text).unwrap();
                texts.push_str(&text);
                info!("Fetched PNG for document {} page {}: {} bytes", doc.number, page, png.len());
            }
            let command = format!("Составь краткое содержание этого документа в 2-6 предложениях, начинай в подобном формате (например если федеральный закон): \"Федеральный закон № 133-ФЗ от 25 мая 2026 года... и дальше содержание документа\", не дополняй кем подписан документ и где, так же не уточняй кем принят или одобрен, нужно только краткое содержание текста. Делай на основе следующего текста: {}", texts);
            let message = MessageWithContent
            {
                role: "user".to_owned(),
                content: vec![serde_json::json!({
                    "type": "text",
                    "text": command
                })]
            };
            let summarization = ai_service.chat(vec![message], Some(0.0)).await.unwrap();
            texts.clear();
            std::fs::write(format!("document_{}_summarization.md", doc.id), &summarization).unwrap();
        }

    }

}