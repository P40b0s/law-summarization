mod logger;
mod database;
mod recognition;
mod publication_service;
mod configuration;
mod ai_service;
mod scheduler;

use std::{sync::Arc, time::Duration};

pub use database::{DbCommand, Document, start_database_service};
pub use configuration::CoreConfiguration;
use tokio::sync::oneshot;
use publication_client::ReqwestPublicationApiClient;
pub use publication_service::PublicationService;
pub use ai_service::{AiService, ChatCompletionRequest, MessageWithContent};
pub use scheduler::Scheduler;
use tokio::time::Instant;
use tracing::{error, info};
use utilites::Date;

use crate::scheduler::ShedulerNew;
//pub use ai_service::{AiService, ChatCompletionRequest, MessageWithContent};


pub async fn run_service(config: Arc<CoreConfiguration>,) -> anyhow::Result<()>
{
    let (db_sender, db_receiver) = tokio::sync::mpsc::channel(16);
    let _db_service = start_database_service(db_receiver).await;
    let client  = ReqwestPublicationApiClient::new();
    let publication_service = PublicationService::new(client, config.clone());
    let ai_service = ai_service::AiService::new("qwen3.5".to_owned(), config.clone());
    loop 
    {
        let date = Date::now();
        let result = publication_service.search_documents(&date).await.unwrap();
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
                        let png = publication_service.get_png(&card.id, page).await.unwrap();
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
                        eo_number: card.eo_number,
                        complex_name: card.complex_name,
                        summary: Some(summarization),
                        respond: db_result_sender 
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
    async fn test_service()
    {
        logger::init();
        let config = Arc::new(CoreConfiguration::default());
        super::run_service(config).await.unwrap();
    }
    
    
    
    
    
    
    
    
    
    
    
    
    
    
    
    
    
    
    
    
    
    
    
    
    
    
    
    
    
    
    
    
    
    #[tokio::test]
    async fn test_recognition()
    {
        crate::logger::init();
        let mock_client = ReqwestPublicationApiClient::new();
        let config = ArcSwap::new(Arc::new(CoreConfiguration::default()));
        let service = PublicationService::new(mock_client, config.load().clone());
        let date = Date::parse("12-05-2026").unwrap();
        let result = service.search_documents(&date).await.unwrap();
        info!("Fetched {} documents for date {}", result.len(), date);
        let ai_service = ai_service::AiService::new("qwen3.5".to_owned(), config.load().clone());
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