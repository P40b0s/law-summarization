use std::sync::Arc;

use bytes::Bytes;
use publication_client::{PublicationApiClient, PublicationDocumentCard, ReqwestPublicationApiClient};
use utilites::Date;

use crate::configuration::CoreConfiguration;

pub struct PublicationService<T: PublicationApiClient>
{
    client: T,
    configuration: Arc<CoreConfiguration>
}
impl<T: PublicationApiClient> PublicationService<T>
{
    pub fn new(client: T, configuration: Arc<CoreConfiguration>) -> Self
    {
        Self
        {
            client,
            configuration
        }
    }
    pub fn create_reqwest_service(configuration: Arc<CoreConfiguration>) -> PublicationService<ReqwestPublicationApiClient> 
    {
        let client = ReqwestPublicationApiClient::new(configuration.publication_api_url.clone(), configuration.publication_base_url.clone());
        PublicationService::new(client, configuration)
    }


    pub async fn fetch_document(&self, eo_number: &str) -> anyhow::Result<PublicationDocumentCard>
    {
        let document = self.client.get_document_by_eo_number(eo_number).await?;
        Ok(document)
    }

    pub async fn get_png(&self, doc_id: &str, number: u32) -> anyhow::Result<Bytes>
    {
        self.client.get_image_by_id(doc_id, number).await
    }
    pub async fn search_documents(&self, date: &Date) -> anyhow::Result<Vec<PublicationDocumentCard>>
    {
        let mut documents = Vec::new();
        for ds in &self.configuration.document_signatories
        {
            let mut docs = self.client.search_documents(date, ds, None).await.unwrap_or_else(|e| {
                tracing::error!("Failed to search documents for date {} and signatory {}: {}", date, ds, e);
                vec![]
            });
            documents.append(&mut docs);
        }
        Ok(documents)
    }
}


mod tests
{
    use super::*;
    use crate::configuration::CoreConfiguration;
    use crate::publication_service::PublicationService;
    use arc_swap::ArcSwap;
use publication_client::{ReqwestPublicationApiClient, PublicationDocumentCard};
use tracing::info;
    use std::sync::Arc;

    #[tokio::test]
    async fn test_fetch_document() 
    {
        crate::logger::init();
        let mock_client = ReqwestPublicationApiClient::default();
        let config = ArcSwap::new(Arc::new(CoreConfiguration::default()));
        let service = PublicationService::new(mock_client, config.load().clone());
        let date = Date::parse("22-05-2026").unwrap();
        let result = service.search_documents(&date).await.unwrap();
        info!("Fetched {} documents for date {}", result.len(), date);
    }

     #[tokio::test]
    async fn test_get_png() 
    {
        crate::logger::init();
        let mock_client = ReqwestPublicationApiClient::default();
        let config = ArcSwap::new(Arc::new(CoreConfiguration::default()));
        let service = PublicationService::new(mock_client, config.load().clone());
        let date = Date::parse("22-05-2026").unwrap();
        let result = service.search_documents(&date).await.unwrap();
        info!("Fetched {} documents for date {}", result.len(), date);
        if let Some(doc) = result.first() 
        {
            for page in 1..=doc.pages_count
            {
                let png = service.get_png(&doc.id, page).await.unwrap();
                std::fs::write(format!("document_{}_page_{}.png", doc.id, page), &png).unwrap();
                info!("Fetched PNG for document {} page {}: {} bytes", doc.id, page, png.len());
            }
        }
    }
}