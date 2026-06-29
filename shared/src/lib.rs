use std::{collections::HashMap, str::FromStr};
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
#[derive(Debug, Clone, Copy, Serialize, Deserialize)]
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


#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum SseMessage
{
    DocsProgressInfo
    {
        count: i32,
        progress: i32,
    },
    PagesProgressInfo
    {
        count: i32,
        progress: i32
    },
    Health,
    CalendarUpdate
    {
        date: String,
        state: DateState
    }
}


#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum SseMessageType
{
    Info
}
impl AsRef<str> for SseMessageType
{
    fn as_ref(&self) -> &str 
    {
        match self 
        {
            Self::Info => "info"    
        }
    }
}

impl FromStr for SseMessageType
{
    type Err = String;
    fn from_str(s: &str) -> Result<Self, Self::Err> 
    {
        match s
        {
            "info" => Ok(SseMessageType::Info),
            _ => Err(format!("Неизвестное значение: {}", s))
        }
    }
}