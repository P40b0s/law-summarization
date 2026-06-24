use summarization_core::{DbCommand, PublicationService, SummarizationService};
use std::sync::Arc;

use publication_client::ReqwestPublicationApiClient;

use crate::configuration::Configuration;

#[derive(Clone)]
pub struct AppState 
{
    pub configuration: Arc<Configuration>,
    //pub publication_service: Arc<PublicationService<ReqwestPublicationApiClient>>,
    //pub ai_service: Arc<summarization_core::AiService>,
    //pub db_tx: tokio::sync::mpsc::Sender<DbCommand>,
    pub summarization_service: Arc<SummarizationService>
}
