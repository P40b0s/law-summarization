use std::collections::HashMap;
use rinf::{DartSignal, RustSignal, RustSignalBinary, SignalPiece};
use serde::{Deserialize, Serialize};
use utilites::Date;


#[derive(Deserialize, Serialize, Clone, DartSignal)]
pub struct PageRequest 
{
    pub id: String,
    pub page_number: i32
}
impl Into<shared::PageRequest> for PageRequest
{
    fn into(self) -> shared::PageRequest 
    {
        shared::PageRequest 
        { 
            id: self.id,
            page_number: self.page_number
        }
    }
}

#[derive(Serialize, RustSignalBinary)]
pub struct PageResponse
{
    pub page_number: i32,
}


#[derive(Deserialize, Serialize, Clone, DartSignal)]
pub struct CalendarRequest
{
    pub from: String,
}
impl Into<shared::CalendarRequest> for CalendarRequest
{
    fn into(self) -> shared::CalendarRequest 
    {
        shared::CalendarRequest 
        {
            from: self.from.parse().unwrap_or(Date::now())
        }
    }
}
#[derive(Serialize, Deserialize, RustSignal)]
pub struct CalendarResponse
{
    pub dates: HashMap<String, DateState>
}

impl Into<CalendarResponse> for shared::CalendarResponse
{
    fn into(self) -> CalendarResponse 
    {
        CalendarResponse 
        { 
            dates: self.dates.into_iter().map(|d| (d.0, d.1.into())).collect()
        }
    }
}

#[derive(Serialize, Deserialize, SignalPiece)]
pub struct DateState
{
    pub checked: i32,
    pub unloaded: i32,
    pub count: i32
}
impl Into<DateState> for shared::DateState
{
    fn into(self) -> DateState 
    {
        DateState 
        { 
            checked: self.checked,
            unloaded: self.unloaded,
            count: self.count 
        }
    }
}


#[derive(Deserialize, Serialize, DartSignal)]
pub struct DocumentPublicationDateRequest
{
    pub publication_date: String,
}
impl Into<shared::DocumentPublicationDateRequest> for DocumentPublicationDateRequest
{
    fn into(self) -> shared::DocumentPublicationDateRequest 
    {
        shared::DocumentPublicationDateRequest
        {
            publication_date: Date::parse(self.publication_date).unwrap_or(Date::now())
        }
    }
}
#[derive(Serialize, Deserialize, RustSignal)]
pub struct DocumentPublicationDateResponse
{
    pub documents: Vec<Document>,
    pub selected_date: String,
}
impl Into<DocumentPublicationDateResponse> for shared::DocumentPublicationDateResponse
{
    fn into(self) -> DocumentPublicationDateResponse 
    {
        DocumentPublicationDateResponse
        {
            documents: self.documents.into_iter().map(|d| d.into()).collect(),
            selected_date: self.selected_date.format(utilites::DateFormat::SerializeDate)
        }
    }
}

#[derive(Deserialize, Serialize, DartSignal)]
pub struct UpdateDocumentRequest
{
    pub document: Document,
}

impl Into<shared::UpdateDocumentRequest> for UpdateDocumentRequest
{
    fn into(self) -> shared::UpdateDocumentRequest 
    {
        shared::UpdateDocumentRequest { document: self.document.into() }
    }
}

#[derive(Serialize, Deserialize, SignalPiece)]
pub struct Document
{
    pub eo_number: String,
    pub publication_date: String,
    pub doc_id: String,
    pub summarization_text: Option<String>,
    pub complex_name: String,
    pub checked_time: Option<String>,
    pub unloaded: bool,
    pub pages_count: i32
}

impl Into<Document> for shared::Document
{
    fn into(self) -> Document 
    {
        Document 
        { 
            eo_number: self.eo_number,
            publication_date: self.publication_date.format(utilites::DateFormat::SerializeDate),
            doc_id: self.doc_id,
            summarization_text: self.summarization_text,
            complex_name: self.complex_name,
            checked_time: self.checked_time.map(|t| t.format(utilites::DateFormat::Serialize)),
            unloaded: self.unloaded,
            pages_count: self.pages_count
        }
    }
}
impl Into<shared::Document> for Document
{
    fn into(self) -> shared::Document 
    {
        shared::Document 
        { 
            eo_number: self.eo_number,
            publication_date: self.publication_date.parse().unwrap_or(Date::now()),
            doc_id: self.doc_id,
            summarization_text: self.summarization_text,
            complex_name: self.complex_name,
            checked_time: self.checked_time.map(|t| t.parse().unwrap_or(Date::now())),
            unloaded: self.unloaded,
            pages_count: self.pages_count
        }
    }
}

#[derive(Serialize, RustSignal)]
pub struct ServiceDocumentsProgress
{
    pub count: i32,
    pub progress: i32,
}

#[derive(Serialize, RustSignal)]
pub struct ServicePagesProgress
{
    pub count: i32,
    pub progress: i32,
}

#[derive(Serialize, RustSignal)]
pub struct ServiceHealth
{
    pub alive: bool,
    pub busy: bool
}




#[derive(Serialize, RustSignal)]
pub struct ErrorSignal
{
    pub error: String,
    pub severity: ErrorSeverity
}

#[derive(Serialize, Deserialize, SignalPiece)]
pub enum ErrorSeverity
{
    Error,
    Warning,
    Info,
    Success
}

impl Into<ErrorSignal> for anyhow::Error
{
    fn into(self) -> ErrorSignal 
    {
        ErrorSignal 
        {
            error: self.to_string(),
            severity: ErrorSeverity::Error
        }
    }
}

