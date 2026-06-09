use std::sync::Arc;

use reqwest::{Url, header::CONTENT_TYPE};
use reqwest_middleware::{ClientBuilder, ClientWithMiddleware};
use reqwest_retry::{RetryTransientMiddleware, policies::ExponentialBackoff};
use anyhow::Result;
use serde::Deserialize;

use crate::{configuration::Configuration, signals::{CalendarRequest, CalendarResponse, DocumentPublicationDateRequest, DocumentPublicationDateResponse, PageRequest, PageResponse}};

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
    pub async fn get_page(&self, req: &PageRequest) -> Result<ApiPageResponse>
    {
        let url = [&self.url, "pages"].concat();
        let body = serde_json::to_string(req)?;
        let result = self.client.post(url)
        .body(body)
        .header(CONTENT_TYPE, "application/json")
        .send()
        .await?
        .json()
        .await?;
        Ok(result)
    }

    pub async fn get_calendar(&self, req: &CalendarRequest) -> Result<CalendarResponse>
    {
        let url = [&self.url, "calendar"].concat();
        let body = serde_json::to_string(req)?;
        let result = self.client.post(url)
        .body(body)
        .header(CONTENT_TYPE, "application/json")
        .send()
        .await?
        .json()
        .await?;
        Ok(result)
    }
    pub async fn get_documents_by_publication_date(&self, req: &DocumentPublicationDateRequest) -> Result<DocumentPublicationDateResponse>
    {
        let url = [&self.url, "documents/publication_date"].concat();
        let body = serde_json::to_string(req)?;
        let result = self.client.post(url)
        .body(body)
        .header(CONTENT_TYPE, "application/json")
        .send()
        .await?
        .json()
        .await?;
        Ok(result)
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