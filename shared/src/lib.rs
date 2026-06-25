use std::collections::HashMap;
use serde::{Deserialize, Serialize};
use utilites::Date;

#[derive(Deserialize, Serialize)]
pub struct PageRequest
{
    pub id: String,
    pub page_number: i32
}
#[derive(Serialize, Deserialize)]
pub struct PageResponse
{
    pub page: Vec<u8>,
    pub page_number: i32
}

#[derive(Deserialize, Serialize)]
pub struct CalendarRequest
{
    pub from: Date,
}
#[derive(Serialize, Deserialize)]
pub struct CalendarResponse
{
    pub dates: HashMap<String, DateState>
}
#[derive(Serialize, Deserialize)]
pub struct DateState
{
    pub checked: i32,
    pub unloaded: i32,
    pub count: i32
}

#[derive(Deserialize, Serialize)]
pub struct DocumentPublicationDateRequest
{
    pub publication_date: Date,
}
#[derive(Serialize, Deserialize)]
pub struct DocumentPublicationDateResponse
{
    pub documents: Vec<Document>,
    pub selected_date: Date,
}

#[derive(Debug, Deserialize, Serialize)]
pub struct UpdateDocumentRequest
{
    pub document: Document,
}


#[derive(Debug, Serialize, Deserialize)]
pub struct Document
{
    pub doc_id: String,
    pub eo_number: String,
    pub complex_name: String,
    pub summarization_text: Option<String>,
    pub publication_date: utilites::Date,
    pub checked_time: Option<utilites::Date>,
    pub unloaded: bool,
    pub pages_count: i32,
}