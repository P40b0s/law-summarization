use std::fmt::Display;

use axum::{http::{HeaderMap, StatusCode}, response::{IntoResponse, Response}};
use serde::Serialize;
use thiserror::Error;
use tracing::error;
use utilites::Date;

#[derive(Error, Debug)]
pub enum AppError 
{
    #[error(transparent)]
    DeserializeError(#[from] serde_json::Error),
    #[error(transparent)]
    IoError(#[from] std::io::Error),
    #[error(transparent)]
    UtilitesError(#[from] utilites::error::Error),
    #[error("Ошибка, путь не найден")]
    PathNotFound,
    #[error("Внутренняя ошибка: {0}")]
    InternalError(String),
    #[error(transparent)]
    AnyhowError(#[from] anyhow::Error),
}

impl serde::Serialize for AppError 
{
	fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
	where
	S: serde::ser::Serializer,
	{
		serializer.serialize_str(self.to_string().as_ref())
	}
}

impl IntoResponse for AppError
{
    fn into_response(self) -> axum::response::Response 
    {
        let message = self.to_string();
        match self
        {
            AppError::IoError(e) =>
            {
                let body = e.to_string();
                error!("{:?}", &body);
                ServerErrorResponse::new(StatusCode::BAD_REQUEST, body).into_response()
            },
            _ => 
            {
                let body = self.to_string();
                error!("{:?}", &body);
                ServerErrorResponse::new(StatusCode::BAD_REQUEST, body).into_response()
            }
        }
    }
}

#[derive(Serialize, Debug, Clone)]
pub struct ServerErrorResponse
{
    err: String
}
impl ServerErrorResponse
{
    pub fn new(status_code: StatusCode, error: String) -> impl IntoResponse
    {
        let e = Self {err: error};
        (status_code, serde_json::to_string(&e).unwrap())
    }
}