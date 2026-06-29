use std::{collections::HashMap, sync::Arc, time::Duration};

use reqwest::{Response, header::CONTENT_TYPE};
use reqwest_middleware::{ClientBuilder, ClientWithMiddleware};
use reqwest_retry::{RetryTransientMiddleware, policies::ExponentialBackoff};
use anyhow::Result;
use futures_util::StreamExt;
use rinf::RustSignal;
use serde::{Deserialize, de::DeserializeOwned};
use shared::{CalendarRequest, DocumentPublicationDateRequest, DocumentPublicationDateResponse, PageRequest, SseMessage};
use tokio::{sync::watch, time::{Instant, sleep}};
use tokio_util::{codec::{FramedRead, LinesCodec}, io::StreamReader};
use tracing::error;

use crate::{configuration::Configuration, signals::{ServiceDocumentsProgress, ServiceHealth, ServicePagesProgress}};
#[derive(Deserialize, Debug, Clone)]
pub struct ServerErrorResponse
{
    err: String
}
pub struct ApiClient
{
    client: ClientWithMiddleware,
    url: String,
    health_notifier: watch::Sender<Instant>,
}
impl ApiClient
{
    const URL: &str = "";
    pub fn new(conf: Arc<Configuration>) -> Self
    {
        let _ = conf;
        let (health_notifier, health_receiver) = watch::channel(Instant::now());
        Self::health_handler(health_receiver);
        let retry_policy = ExponentialBackoff::builder().build_with_max_retries(8);
        let client: ClientWithMiddleware = ClientBuilder::new(reqwest::Client::new())
            .with(RetryTransientMiddleware::new_with_policy(retry_policy))
                .build();
        Self
        {
            url: ["http://", &conf.service_addresse, ":", conf.service_port.to_string().as_str(), "/api/v1/"].concat(),
            client: client,
            health_notifier
        }
    }

    pub fn health_handler(mut receiver: tokio::sync::watch::Receiver<Instant>)
    {
        tokio::spawn(async move 
        {
            loop 
            {
                // Вычисляем, сколько времени прошло с последнего пинга
                let last_ping = *receiver.borrow();
                let elapsed = last_ping.elapsed();

                if elapsed >= Duration::from_secs(10) 
                {
                    // Таймаут превышен! Отправляем false в Dart
                    ServiceHealth { alive: false }.send_signal_to_dart();
                    
                    // Чтобы не спамить в Dart каждые миллисекунды после таймаута, 
                    // ждем фиксированные 10 секунд до следующей проверки (или пока не придет новый пинг)
                    tokio::select! 
                    {
                        _ = tokio::time::sleep(Duration::from_secs(10)) => {}
                        _ = receiver.changed() => {}
                    }
                } 
                else 
                {
                    // Если пинг был недавно, спим ровно столько, сколько осталось до лимита в 10 сек
                    let time_left = Duration::from_secs(10) - elapsed;
                    tokio::select! 
                    {
                        _ = tokio::time::sleep(time_left) => {}
                        // Если в процессе сна пришел новый пинг — мгновенно просыпаемся и пересчитываем
                        _ = receiver.changed() => {}
                    }
                }
            }
        });
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

    pub async fn get_calendar(&self, req: &CalendarRequest) -> Result<shared::CalendarResponse>
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

    pub async fn update_document(&self, req: &shared::UpdateDocumentRequest) -> Result<shared::CalendarResponse>
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

    pub async fn events_handler(&self) -> Result<()>
    {
        let url = [&self.url, "events"].concat();
        let retry_interval = Duration::from_secs(5); 
        loop 
        {
            let response_result = self.client
            .get(&url)
            .header("Accept", "text/event-stream")
            .header("Cache-Control", "no-cache")
            .send()
            .await;

            let response = match response_result 
            {
                Ok(res) => res,
                Err(e) => 
                {
                    error!("Ошибка сети при подключении к SSE: {}. Повтор через {:?}", e, retry_interval);
                    sleep(retry_interval).await;
                    continue; // Уходим на следующую попытку
                }
            };
            let status = response.status();
            if !status.is_success() 
            {
                // Пытаемся прочитать ошибку от сервера, но не даем ей уронить функцию
                if let Ok(error_response) = response.json::<ServerErrorResponse>().await 
                {
                    error!("Сервер вернул ошибку: {}. Повтор через {:?}", error_response.err, retry_interval);
                } 
                else 
                {
                    error!("Сервер вернул статус {}, но не удалось распарсить ответ. Повтор через {:?}", status, retry_interval);
                }
                sleep(retry_interval).await;
                continue;
            }

            if let Err(e) = self.process_sse_stream(response).await 
            {
                error!("Поток SSE оборвался с ошибкой: {}. Переподключение через {:?}", e, retry_interval);
            } 
            else 
            {
                error!("Поток SSE был закрыт сервером. Переподключение через {:?}", retry_interval);
            }
            sleep(retry_interval).await;
        }
      

        // if !response.status().is_success()
        // {
        //     let error_response: ServerErrorResponse = response.json().await?;
        //     return Err(anyhow::anyhow!("{}", error_response.err));
        // }
        // // Обрабатываем SSE поток
        // self.process_sse_stream(response).await?;
    }

    async fn process_sse_stream(&self, response: Response) -> Result<()> 
    {
        let byte_stream = response.bytes_stream().map(|item| 
        {
            item.map_err(|e| std::io::Error::new(std::io::ErrorKind::Other, e))
        });
        let async_reader = StreamReader::new(byte_stream);
        let mut lines_stream = FramedRead::new(async_reader, LinesCodec::new());

        let mut current_event = String::new();
        let mut current_data = String::new();
        let mut current_id = String::new();

        while let Some(line_result) = lines_stream.next().await 
        {
            let line = line_result?;
            let trimmed = line.trim();

            if trimmed.is_empty() 
            {
                // Пустая строка — событие завершено, выводим данные
                if !current_data.is_empty() 
                {
                    if let Ok(msg) = serde_json::from_str::<SseMessage>(&current_data)
                    {
                        self.process_sse_message(msg).await;
                    }
                    else 
                    {
                        error!("Ошибка парсинга JSON сообщения от сервера {}", current_data);    
                    }
                    current_event.clear();
                    current_data.clear();
                    current_id.clear();
                }
                continue;
            }

            if let Some((field, value)) = trimmed.split_once(':') 
            {
                let value = value.trim();
                match field {
                    "data" => {
                        if !current_data.is_empty() 
                        {
                            current_data.push('\n');
                        }
                        current_data.push_str(value);
                    }
                    "event" => 
                    {
                        current_event.clear();
                        current_event.push_str(value);
                    }
                    "id" => 
                    {
                        current_id.clear();
                        current_id.push_str(value);
                    }
                    _ => {} // Игнорируем retry и комментарии
                }
            }
        }
        Ok(())
    }

    async fn process_sse_message(&self, sse_message: SseMessage)
    {
        match sse_message
        {
            SseMessage::DocsProgressInfo {count, progress} => 
            {
                ServiceDocumentsProgress
                {
                    count,
                    progress
                }.send_signal_to_dart();
            },
            SseMessage::PagesProgressInfo { count, progress } =>
            {
                ServicePagesProgress
                {
                    count,
                    progress
                }.send_signal_to_dart();
            },
            SseMessage::Health =>
            {
                let _ = self.health_notifier.send(Instant::now());
                ServiceHealth
                {
                    alive: true
                }.send_signal_to_dart();
            },
            SseMessage::CalendarUpdate { date, state } =>
            {
                let mut hm = HashMap::with_capacity(1);
                hm.insert(date, state.into());
                crate::signals::CalendarResponse
                {
                    dates: hm
                }.send_signal_to_dart();
            }
        }
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