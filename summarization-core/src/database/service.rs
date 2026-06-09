use anyhow::anyhow;
use tokio::sync::mpsc;
use tracing::{error, info};

use super::commands::DbCommand;
use super::documents::{DocumentsTable, DocumentsDbo};

pub async fn start_database_service(receiver: mpsc::Receiver<DbCommand>)
{
    tokio::spawn(async move 
    {
        match database_service(receiver).await
        {
            Ok(_) => (),
            Err(e) => 
            {
                error!("Database service crashed: {}", e);
                //panic!("Database service crashed: {}", e);
            }
        }
    });
}

async fn database_service(mut receiver: mpsc::Receiver<DbCommand>) -> anyhow::Result<()>
{
    let documents = DocumentsTable::new_default("law_summaries").await?;
    while let Some(command) = receiver.recv().await 
    {
        match command 
        {
            DbCommand::InsertDocument { doc_id, eo_number, publication_date, complex_name, summary, respond } => 
            {
                // Логика сохранения документа в БД
                info!("Saving document for EO number: {}", eo_number);
                let doc = DocumentsDbo {
                    doc_id,
                    eo_number: eo_number,
                    summarization_text: summary.clone(),
                    complex_name,
                    checked_time: None,
                    unloaded: false,
                    publication_date,
                };
                let result = documents.insert(&doc).await
                    .inspect_err(|e| error!("Failed to insert document: {}", e));
                respond.send(result).unwrap_or_else(|e| error!("Failed to send response for InsertDocument: {:?}", e));
            }
            DbCommand::UpdateDocument { doc_id, eo_number, publication_date, summary, respond, complex_name } => 
            {
                // Логика получения резюме из БД
                info!("Update document: {}", eo_number);
                let doc = DocumentsDbo {
                    doc_id,
                    eo_number: eo_number,
                    summarization_text: summary.clone(),
                    complex_name,
                    checked_time: None,
                    unloaded: false,
                    publication_date,
                };

                let result = documents.update(&doc).await
                    .inspect_err(|e| error!("Failed to update document: {}", e));
                respond.send(result).unwrap_or_else(|e| error!("Failed to send response for UpdateDocument: {:?}", e));
            }
            DbCommand::DeleteDocument { eo_number, respond } =>
            {
                // Логика удаления документа из БД
                info!("Delete document: {}", eo_number);
                let result = documents.delete(&eo_number).await
                    .inspect_err(|e| error!("Failed to send response for DeleteDocument: {}", e));
                respond.send(result).unwrap_or_else(|e| error!("Failed to send response for DeleteDocument: {:?}", e));
            }
            DbCommand::GetDocument { eo_number, respond } =>
            {
                // Логика получения документа из БД
                info!("Get document: {}", eo_number);
                let result = documents.get_by_id(&eo_number).await
                    .inspect_err(|e| error!("Failed to send response for GetDocument: {}", e))
                    .and_then(|r| Ok(r.map(|d| d.into())));
                respond.send(result).unwrap_or_else(|e| error!("Failed to send response for GetDocument: {:?}", e));
            }
            DbCommand::GetDocuments { publication_date, respond } =>
            {
                // Логика получения документов из БД
                info!("Get documents for date: {}", publication_date);
                let result = documents.get_by_publication_date(&publication_date).await
                    .inspect_err(|e| error!("Failed to send response for GetDocuments: {}", e))
                    .and_then(|r| Ok(r.into_iter().map(|d| d.into()).collect()));
                respond.send(result).unwrap_or_else(|e| error!("Failed to send response for GetDocuments: {:?}", e));
            }
            DbCommand::GetAllDocuments { respond } =>
            {
                // Логика получения всех документов из БД
                info!("Get all documents");
                let result = documents.get_all().await
                    .inspect_err(|e| error!("Failed to send response for GetAllDocuments: {}", e))
                    .and_then(|r| Ok(r.into_iter().map(|d| d.into()).collect()));
                respond.send(result).unwrap_or_else(|e| error!("Failed to send response for GetAllDocuments: {:?}", e));
            }
            DbCommand::GetUnloadedDocuments { respond } => {
                info!("Get unloaded documents");
                let result = documents.get_unloaded().await
                    .inspect_err(|e| error!("Failed to send response for GetUnloadedDocuments: {}", e))
                    .and_then(|r| Ok(r.into_iter().map(|d| d.into()).collect()));
                respond.send(result).unwrap_or_else(|e| error!("Failed to send response for GetUnloadedDocuments: {:?}", e));
            }
            DbCommand::SetDocumentUnloaded { eo_number, unloaded, respond } => {
                info!("Set unloaded={} for {}", unloaded, eo_number);
                let result = documents.set_unloaded(&eo_number, unloaded).await
                    .inspect_err(|e| error!("Failed to set unloaded: {}", e));
                respond.send(result).unwrap_or_else(|e| error!("Failed to send response for SetDocumentUnloaded: {:?}", e));
            }
            DbCommand::SetDocumentCheckedTime { eo_number, checked_time, respond } => {
                info!("Set checked_time for {}", eo_number);
                let result = documents.set_checked_time(&eo_number, checked_time).await
                    .inspect_err(|e| error!("Failed to set checked_time: {}", e));
                respond.send(result).unwrap_or_else(|e| error!("Failed to send response for SetDocumentCheckedTime: {:?}", e));
            },
        }
    }
    Err(anyhow!("Database service stopped unexpectedly"))
}
