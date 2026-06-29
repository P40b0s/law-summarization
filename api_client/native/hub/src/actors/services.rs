use std::sync::Arc;

use async_trait::async_trait;
use messages::{actor::Actor, address::Address, context::Context, handler::Notifiable};
use rinf::{DartSignal, RustSignal, RustSignalBinary, debug_print};
use tokio::{spawn, task::JoinSet};
use tracing::error;
use crate::{client::ApiClient, configuration::Configuration, signals::{CalendarRequest, CalendarResponse, DocumentPublicationDateRequest, DocumentPublicationDateResponse, ErrorSignal, PageRequest, PageResponse, UpdateDocumentRequest}};
/// Actor definition that will hold state in real apps.
pub struct ServicesActor 
{
  client: Arc<ApiClient>,
  _owned_tasks: JoinSet<()>
}

impl Actor for ServicesActor {}

impl ServicesActor 
{
  pub fn new(self_addr: Address<Self>, conf: Arc<Configuration>) -> Self 
  {
    let client = Arc::new(ApiClient::new(conf));
    let mut owned_tasks = JoinSet::new();
    owned_tasks.spawn(Self::listen_to_pages_request(self_addr.clone()));
    owned_tasks.spawn(Self::listen_to_calendar_request(self_addr.clone()));
    owned_tasks.spawn(Self::listen_to_documents_by_date_request(self_addr.clone()));
    owned_tasks.spawn(Self::listen_to_document_update(self_addr.clone()));
    let events_client = Arc::clone(&client);
    spawn(async move {events_client.events_handler().await});
    Self 
    { 
      client,
      _owned_tasks: owned_tasks
    }
  }

  async fn listen_to_pages_request(mut self_addr: Address<Self>) 
  {
    let receiver = PageRequest::get_dart_signal_receiver();
    while let Some(signal_pack) = receiver.recv().await 
    {
      let message = signal_pack.message;
      let _ = self_addr.notify(message).await;
    }
  }

  async fn listen_to_calendar_request(mut self_addr: Address<Self>) 
  {
    let receiver = CalendarRequest::get_dart_signal_receiver();
    while let Some(signal_pack) = receiver.recv().await 
    {
      let message = signal_pack.message;
      let _ = self_addr.notify(message).await;
    }
  }

  async fn listen_to_documents_by_date_request(mut self_addr: Address<Self>) 
  {
    let receiver = DocumentPublicationDateRequest::get_dart_signal_receiver();
    while let Some(signal_pack) = receiver.recv().await 
    {
      let message = signal_pack.message;
      let _ = self_addr.notify(message).await;
    }
  }
  async fn listen_to_document_update(mut self_addr: Address<Self>) 
  {
    let receiver = UpdateDocumentRequest::get_dart_signal_receiver();
    while let Some(signal_pack) = receiver.recv().await 
    {
      let message = signal_pack.message;
      let _ = self_addr.notify(message).await;
    }
  }
}

#[async_trait]
impl Notifiable<PageRequest> for ServicesActor 
{
  async fn notify(&mut self, msg: PageRequest, _: &Context<Self>) 
  {
    debug_print!("request page № {} for  {}", msg.page_number, msg.id);
    let req = msg.into();
    let page = self.client.get_page(&req).await;
    if let Err(e) = page
    {
        error!("{:?}", e);
        let signal: ErrorSignal = e.into();
        signal.send_signal_to_dart();
    }
    else 
    {
        let page = page.unwrap();
        let flutter = PageResponse
        {
            page_number: page.page_number
        };
        flutter.send_signal_to_dart(page.page);   
    }
  }
}

#[async_trait]
impl Notifiable<CalendarRequest> for ServicesActor 
{
  async fn notify(&mut self, msg: CalendarRequest, _: &Context<Self>) 
  {
    debug_print!("request calendar from date {}", msg.from);
    let req = msg.into();
    let calendar = self.client.get_calendar(&req).await;
    if let Err(e) = calendar
    {
      error!("{:?}", e);
      let signal: ErrorSignal = e.into();
      signal.send_signal_to_dart();
    }
    else 
    {
      let calendar: CalendarResponse = calendar.unwrap().into();
      calendar.send_signal_to_dart();   
    }
  }
}

#[async_trait]
impl Notifiable<DocumentPublicationDateRequest> for ServicesActor 
{
  async fn notify(&mut self, msg: DocumentPublicationDateRequest, _: &Context<Self>) 
  {
    debug_print!("request documents for publication date {}", msg.publication_date);
    let req = msg.into();
    let documents = self.client.get_documents_by_publication_date(&req).await;
    if let Err(e) = documents
    {
      error!("{:?}", e);
      let signal: ErrorSignal = e.into();
      signal.send_signal_to_dart();
    }
    else 
    {
      let documents: DocumentPublicationDateResponse = documents.unwrap().into();
      documents.send_signal_to_dart();   
    }
  }
}


#[async_trait]
impl Notifiable<UpdateDocumentRequest> for ServicesActor 
{
  async fn notify(&mut self, msg: UpdateDocumentRequest, _: &Context<Self>) 
  {
    debug_print!("request update document {}", msg.document.doc_id);
    let req = msg.into();
    let documents = self.client.update_document(&req).await;
    if let Err(e) = documents
    {
      error!("{:?}", e);
      let signal: ErrorSignal = e.into();
      signal.send_signal_to_dart();
    }
    else 
    {
      let calendar: CalendarResponse = documents.unwrap().into();
      calendar.send_signal_to_dart();  
    }
  }
}