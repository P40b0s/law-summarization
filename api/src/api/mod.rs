mod layers;
mod router;
mod static_router;
mod app_router;
use std::collections::HashMap;

use app_router::app_router;
pub use layers::cors_layer;
pub use router::{ApiVersion, with_api_version, router};
use serde::{Deserialize, Serialize};
use summarization_core::Document;
use utilites::Date;

//use static_router::static_router;

#[derive(Deserialize)]
pub struct PageRequest
{
    id: String,
    page_number: i32
}
#[derive(Serialize)]
pub struct PageResponse
{
    page: Vec<u8>,
    page_number: i32
}

#[derive(Deserialize)]
pub struct CalendarRequest
{
    from: Date,
}
#[derive(Serialize)]
pub struct CalendarResponse
{
    dates: HashMap<String, DateState>
}
#[derive(Serialize)]
pub struct DateState
{
    ready: i32,
    unready: i32
}

#[derive(Deserialize)]
pub struct DocumentPublicationDateRequest
{
    publication_date: Date,
}
#[derive(Serialize)]
pub struct DocumentPublicationDateResponse
{
    documents: Vec<Document>,
    selected_date: Date,
}