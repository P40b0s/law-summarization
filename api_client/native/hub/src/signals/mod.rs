use std::collections::HashMap;

use rinf::{DartSignal, RustSignal, RustSignalBinary, SignalPiece};
use serde::{Deserialize, Serialize};
use utilites::Date;

/// To send data from Dart to Rust, use `DartSignal`.
#[derive(Deserialize, DartSignal)]
pub struct SmallText {
    pub text: String,
}

/// To send data from Rust to Dart, use `RustSignal`.
#[derive(Serialize, RustSignal)]
pub struct SmallNumber {
    pub number: i32,
}

/// A signal can be nested inside another signal.
#[derive(Serialize, RustSignal)]
pub struct BigBool {
    pub member: bool,
    pub nested: SmallBool,
}

/// To nest a signal inside other signal, use `SignalPiece`.
#[derive(Serialize, SignalPiece)]
pub struct SmallBool(pub bool);


#[derive(Deserialize, Serialize, Clone, DartSignal)]
pub struct PageRequest 
{
    pub id: String,
    pub page_number: i32
}

#[derive(Serialize, RustSignalBinary)]
pub struct  PageResponse
{
    pub page_number: i32,
}

#[derive(Deserialize, Serialize, Clone, DartSignal)]
pub struct CalendarRequest
{
    pub from: String,
}
#[derive(Serialize, Deserialize, RustSignal)]
pub struct CalendarResponse
{
    pub dates: HashMap<String, DateState>
}
#[derive(Serialize, Deserialize, SignalPiece)]
pub struct DateState
{
    pub ready: i32,
    pub unready: i32
}

#[derive(Deserialize, Serialize, DartSignal)]
pub struct DocumentPublicationDateRequest
{
    pub publication_date: String,
}
#[derive(Serialize, Deserialize, RustSignal)]
pub struct DocumentPublicationDateResponse
{
    pub documents: Vec<Document>,
    pub selected_date: String,
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
}

#[derive(Serialize, RustSignal)]
pub struct ErrorSignal
{
    pub error: String,
}

impl Into<ErrorSignal> for anyhow::Error
{
    fn into(self) -> ErrorSignal 
    {
        ErrorSignal 
        {
            error: self.to_string()
        }
    }
}

