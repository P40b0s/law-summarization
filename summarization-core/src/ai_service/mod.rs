use std::sync::Arc;

use anyhow::{anyhow, Context, Result};
use serde::{Deserialize, Serialize};
use reqwest::Client;
use tracing::error;

use crate::configuration::CoreConfiguration;



/// Контент с текстом
#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct TextContent {
    pub r#type: String, // "text"
    pub text: String,
}

/// Контент с изображением
#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct ImageUrlContent {
    pub r#type: String, // "image_url"
    pub image_url: ImageUrl,
}

#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct ImageUrl {
    pub url: String, // data:image/png;base64,...
}

/// Сообщение в API запросе
#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct Message {
    pub role: String, // "user", "assistant", etc
    #[serde(skip_serializing_if = "Option::is_none")]
    pub content: Option<String>, // для простого текста
}

/// Сообщение с контентом (текст + изображения)
#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct MessageWithContent {
    pub role: String,
    pub content: Vec<serde_json::Value>, // может быть текст или image_url
}

/// Запрос к OpenAI-подобному API
#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct ChatCompletionRequest {
    pub model: String,
    pub messages: Vec<MessageWithContent>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub temperature: Option<f32>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub max_tokens: Option<u32>,
}

/// Ответ от API
#[derive(Debug, Serialize, Deserialize)]
pub struct ChatCompletionResponse {
    pub id: String,
    pub object: String,
    pub created: u64,
    pub model: String,
    pub choices: Vec<Choice>,
    pub usage: Usage,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct Choice {
    pub index: u32,
    pub message: Message,
    pub finish_reason: String,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct Usage {
    pub prompt_tokens: u32,
    pub completion_tokens: u32,
    pub total_tokens: u32,
}

/// AI сервис для отправки запросов к OpenAI-подобному API
#[derive(Clone)]
pub struct AiService {
    client: Client,
    model: String,
    configuration: Arc<CoreConfiguration>,
}

impl AiService {
    /// Создать новый сервис
    pub fn new(model: String, configuration: Arc<CoreConfiguration>) -> Self {
        Self {
            client: Client::new(),
            model,
            configuration,
        }
    }

    /// Распознать текст с изображения
    /// 
    /// # Arguments
    /// * `image_base64` - изображение в формате data:image/png;base64,...
    /// * `prompt` - текст с инструкциями для распознавания
    /// * `temperature` - параметр температуры (по умолчанию 0.0)
    pub async fn recognize_image(
        &self,
        image_base64: &str,
        prompt: &str,
        temperature: Option<f32>,
    ) -> Result<String> {
        let text_content = serde_json::json!({
            "type": "text",
            "text": prompt
        });

        let image_content = serde_json::json!({
            "type": "image_url",
            "image_url": {
                "url": image_base64
            }
        });

        let message = MessageWithContent {
            role: "user".to_string(),
            content: vec![text_content, image_content],
        };

        let request = ChatCompletionRequest {
            model: self.model.clone(),
            messages: vec![message],
            temperature: temperature.or(Some(0.0)),
            max_tokens: Some(2048),
        };

        let url = format!("{}/v1/chat/completions", self.configuration.ai_service_url);
        let response = self.client
            .post(&url)
            .json(&request)
            .send()
            .await
            .context("Failed to send request to AI API")?;

        let status = response.status();
        if !status.is_success() {
            let error_text = response.text().await.unwrap_or_default();
            return Err(anyhow!("AI API returned {}: {}", status, error_text));
        }

        let response_data: ChatCompletionResponse = response
            .json()
            .await
            .context("Failed to parse AI API response")?;

        if response_data.choices.is_empty() {
            return Err(anyhow!("No choices in AI API response"));
        }

        let content = response_data.choices[0]
            .message
            .content
            .clone()
            .ok_or_else(|| anyhow!("No content in AI response"))?;

        Ok(content)
    }

    /// Отправить общий запрос к API
    pub async fn chat(
        &self,
        messages: Vec<MessageWithContent>,
        temperature: Option<f32>,
    ) -> Result<String> {
        let request = ChatCompletionRequest {
            model: self.model.clone(),
            messages,
            temperature: temperature.or(Some(0.0)),
            max_tokens: None,
        };

        let url = format!("{}/v1/chat/completions", self.configuration.ai_service_url);
        let response = self.client
            .post(&url)
            .json(&request)
            .send()
            .await
            .context("Failed to send request to AI API")?;

        let status = response.status();
        if !status.is_success() {
            let error_text = response.text().await.unwrap_or_default();
            return Err(anyhow!("AI API returned {}: {}", status, error_text));
        }

        let response_data: ChatCompletionResponse = response
            .json()
            .await
            .context("Failed to parse AI API response")?;

        if response_data.choices.is_empty() {
            return Err(anyhow!("No choices in AI API response"));
        }

        let content = response_data.choices[0]
            .message
            .content
            .clone()
            .ok_or_else(|| anyhow!("No content in AI response"))?;

        Ok(content)
    }

     /// Проверка статуса
    pub async fn status(
        &self,
    ) -> Result<SlotsStatus> {

        let url = format!("{}/slots", self.configuration.ai_service_url);
        let mut response: Vec<SlotsStatus> = self.client
            .get(&url)
            .send()
            .await?
            .json()
            .await
            .inspect_err(|e| error!("Ошибка запроса статуса llama! {}", e))
            .context("Failed to send request to AI API")?;
        response.pop().ok_or(anyhow!("Ошибка, массив статусов не заполнен"))
    }
}


#[derive(Debug, Deserialize)]
pub struct SlotsStatus
{
    pub id: i32,
    pub n_ctx: i32,
    pub speculative: bool,
    pub is_processing: bool
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_request_serialization() {
        let text_content = serde_json::json!({
            "type": "text",
            "text": "Hello"
        });

        let message = MessageWithContent {
            role: "user".to_string(),
            content: vec![text_content],
        };

        let request = ChatCompletionRequest {
            model: "qwen3.5".to_string(),
            messages: vec![message],
            temperature: Some(0.0),
            max_tokens: None,
        };

        let json = serde_json::to_string(&request).unwrap();
        assert!(json.contains("qwen3.5"));
        assert!(json.contains("user"));
    }
}
