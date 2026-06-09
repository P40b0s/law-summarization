use std::sync::Arc;

use async_trait::async_trait;
use messages::{actor::Actor, address::Address, context::Context, handler::Notifiable};
use rinf::{DartSignal, RustSignal, RustSignalBinary, debug_print};
use tokio::task::JoinSet;
use tracing::error;

use crate::{client::ApiClient, configuration::Configuration, signals::{CalendarRequest, CalendarResponse, DocumentPublicationDateRequest, ErrorSignal, PageRequest, PageResponse}};
/// Actor definition that will hold state in real apps.
pub struct ServicesActor 
{
  client: ApiClient,
  _owned_tasks: JoinSet<()>
}

impl Actor for ServicesActor {}

impl ServicesActor 
{
  pub fn new(self_addr: Address<Self>, conf: Arc<Configuration>) -> Self 
  {
    let mut owned_tasks = JoinSet::new();
    owned_tasks.spawn(Self::listen_to_pages_request(self_addr.clone()));
    owned_tasks.spawn(Self::listen_to_calendar_request(self_addr.clone()));
    owned_tasks.spawn(Self::listen_to_documents_by_date_request(self_addr.clone()));
    Self 
    { 
        client: ApiClient::new(conf),
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
}

#[async_trait]
impl Notifiable<PageRequest> for ServicesActor 
{
  async fn notify(&mut self, msg: PageRequest, _: &Context<Self>) 
  {
    debug_print!("request page № {} for  {}", msg.page_number, msg.id);
    let page = self.client.get_page(&msg).await;
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
    let calendar = self.client.get_calendar(&msg).await;
    if let Err(e) = calendar
    {
        error!("{:?}", e);
        let signal: ErrorSignal = e.into();
        signal.send_signal_to_dart();
    }
    else 
    {
        let calendar = calendar.unwrap();
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
    let documents = self.client.get_documents_by_publication_date(&msg).await;
    if let Err(e) = documents
    {
        error!("{:?}", e);
        let signal: ErrorSignal = e.into();
        signal.send_signal_to_dart();
    }
    else 
    {
        let documents = documents.unwrap();
        documents.send_signal_to_dart();   
    }
  }
}