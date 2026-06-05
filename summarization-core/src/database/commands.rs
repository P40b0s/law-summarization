use tokio::sync::oneshot;

pub enum DbCommand 
{
    // Document commands
    InsertDocument 
    {
        doc_id: String,
        eo_number: String,
        complex_name: String,
        summary: Option<String>,
        respond: oneshot::Sender<anyhow::Result<()>>,
    },
    UpdateDocument 
    {
        doc_id: String,
        eo_number: String,
        complex_name: String,
        summary: Option<String>,
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
}
