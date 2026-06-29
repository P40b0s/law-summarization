use summarization_core::{DbCommand, PublicationService, SummarizationService};
use std::sync::Arc;

use publication_client::ReqwestPublicationApiClient;

use crate::{configuration::Configuration};

#[derive(Clone)]
pub struct AppState 
{
    pub configuration: Arc<Configuration>,
    pub summarization_service: Arc<SummarizationService>,
}
