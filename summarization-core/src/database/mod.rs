mod connection;
mod documents;
mod commands;
mod service;
pub use commands::DbCommand;
use documents::DocumentsDbo;
use serde::{Deserialize, Serialize};
pub use service::start_database_service;

#[derive(Debug, Serialize, Deserialize)]
pub struct Document
{
    pub doc_id: String,
    pub eo_number: String,
    pub complex_name: String,
    pub summarization_text: Option<String>,
}
impl From<DocumentsDbo> for Document
{
    fn from(dbo: DocumentsDbo) -> Self 
    {
        Self
        {
            doc_id: dbo.doc_id,
            eo_number: dbo.eo_number,
            complex_name: dbo.complex_name,
            summarization_text: dbo.summarization_text,
        }
    }
}




