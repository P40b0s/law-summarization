use std::sync::Arc;

use reqwest::{Response, Url, header::CONTENT_TYPE};
use reqwest_middleware::{ClientBuilder, ClientWithMiddleware};
use reqwest_retry::{RetryTransientMiddleware, policies::ExponentialBackoff};
use anyhow::Result;
use serde::{Deserialize, Serialize, de::DeserializeOwned};
use shared::{CalendarRequest, CalendarResponse, DocumentPublicationDateRequest, DocumentPublicationDateResponse, PageRequest};

use crate::{configuration::Configuration};
#[derive(Deserialize, Debug, Clone)]
pub struct ServerErrorResponse
{
    err: String
}
pub struct ApiClient
{
    client: ClientWithMiddleware,
    url: String
}
impl ApiClient
{
    const URL: &str = "";
    pub fn new(conf: Arc<Configuration>) -> Self
    {
        let _ = conf;
        let retry_policy = ExponentialBackoff::builder().build_with_max_retries(8);
        let client: ClientWithMiddleware = ClientBuilder::new(reqwest::Client::new())
            .with(RetryTransientMiddleware::new_with_policy(retry_policy))
                .build();
        Self
        {
            url: ["http://", &conf.service_addresse, ":", conf.service_port.to_string().as_str(), "/api/v1/"].concat(),
            client: client,
        }
    }
    pub async fn get_page(&self, req: &PageRequest) -> Result<shared::PageResponse>
    {
        let url = [&self.url, "pages"].concat();
        let body = serde_json::to_string(req)?;
        let response = self.client.post(url)
        .body(body)
        .header(CONTENT_TYPE, "application/json")
        .send()
        .await?;
        Self::check_response(response).await
    }

    pub async fn get_calendar(&self, req: &CalendarRequest) -> Result<CalendarResponse>
    {
        let url = [&self.url, "calendar"].concat();
        let body = serde_json::to_string(req)?;
        let response = self.client.post(url)
        .body(body)
        .header(CONTENT_TYPE, "application/json")
        .send()
        .await?;
        Self::check_response(response).await
    }

    pub async fn update_document(&self, req: &shared::UpdateDocumentRequest) -> Result<CalendarResponse>
    {
        let url = [&self.url, "documents/update"].concat();
        let body = serde_json::to_string(req)?;
        let response = self.client.post(url)
        .body(body)
        .header(CONTENT_TYPE, "application/json")
        .send()
        .await?;
        Self::check_response(response).await
        
    }

    async fn check_response<T: DeserializeOwned>(response: Response) -> Result<T>
    {
        if response.status().is_success()
        {
            Ok(response.json().await?)
        }
        else
        {
            let error_response: ServerErrorResponse = response.json().await?;
            Err(anyhow::anyhow!("{}", error_response.err))
        }
    }
    pub async fn get_documents_by_publication_date(&self, req: &DocumentPublicationDateRequest) -> Result<DocumentPublicationDateResponse>
    {
        let url = [&self.url, "documents/publication_date"].concat();
        let body = serde_json::to_string(req)?;
        let response = self.client.post(url)
        .body(body)
        .header(CONTENT_TYPE, "application/json")
        .send()
        .await?;
        Self::check_response(response).await
        
     }
}

#[derive(Deserialize)]
pub struct  ApiPageResponse
{
    pub page: Vec<u8>,
    pub page_number: i32,
}


#[cfg(test)]
mod tests
{
    use reqwest::header::CONTENT_TYPE;
use reqwest_middleware::{ClientBuilder, ClientWithMiddleware};
use reqwest_retry::{RetryTransientMiddleware, policies::ExponentialBackoff};

use crate::signals::{PageRequest, PageResponse};

    #[tokio::test]
    pub async fn test_get_page()
    {
        let url = "http://127.0.0.1:8081/api/v1/pages";
        let req = PageRequest
        {
            id: "5133ba0c-1d95-42e5-822f-c10c691b467d".to_owned(),
            page_number: 1
        };
        let retry_policy = ExponentialBackoff::builder().build_with_max_retries(8);
        let client: ClientWithMiddleware = ClientBuilder::new(reqwest::Client::new())
            .with(RetryTransientMiddleware::new_with_policy(retry_policy))
                .build();
        let body = serde_json::to_string(&req).unwrap();
        let result: super::ApiPageResponse = client.post(url)
        .body(body)
        .header(CONTENT_TYPE, "application/json")
        .send()
        .await.unwrap()
        .json()
        .await.unwrap();
        println!("{}", result.page_number);
    }
}