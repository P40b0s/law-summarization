use tokio::sync::oneshot;
use utilites::Date;

use crate::database::documents;

pub enum DbCommand 
{
    // Document commands
    InsertDocument 
    {
        doc_id: String,
        publication_date: utilites::Date,
        eo_number: String,
        complex_name: String,
        summary: Option<String>,
        pages_count: i32,
        respond: oneshot::Sender<anyhow::Result<()>>,
    },
    UpdateDocument 
    {
        doc_id: String,
        summary: Option<String>,
        unloaded: bool,
        checked_time: Option<Date>,
        respond: oneshot::Sender<anyhow::Result<()>>,
    },
    DeleteDocument 
    {
        eo_number: String,
        respond: oneshot::Sender<anyhow::Result<()>>,
    },
    GetDocument 
    {
        eo_number: String,
        respond: oneshot::Sender<anyhow::Result<Option<super::Document>>>,
    },
    GetDocuments
    {
        publication_date: Date,
        respond: oneshot::Sender<anyhow::Result<Vec<super::Document>>>,
    },
    GetAllDocuments 
    {
        respond: oneshot::Sender<anyhow::Result<Vec<super::Document>>>,
    },
    GetUnloadedDocuments 
    {
        respond: oneshot::Sender<anyhow::Result<Vec<super::Document>>>,
    },
    SetDocumentUnloaded 
    {
        eo_number: String,
        unloaded: bool,
        respond: oneshot::Sender<anyhow::Result<()>>,
    },
    SetDocumentCheckedTime 
    {
        eo_number: String,
        checked_time: Option<utilites::Date>,
        respond: oneshot::Sender<anyhow::Result<()>>,
    },
    GetCalendarState 
    {
        date_from: Date,
        date_to: Date,
        respond: oneshot::Sender<anyhow::Result<Vec<documents::DocumentStatsRow>>>,
    },
}
