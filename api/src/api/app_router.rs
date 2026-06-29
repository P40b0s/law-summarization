use std::{collections::HashMap, convert::Infallible, net::SocketAddr, sync::Arc, time::Duration};
use axum::{Extension, Json, Router, body::Body, extract::{ConnectInfo, DefaultBodyLimit, Path, State}, http::{HeaderValue, StatusCode, header}, response::{IntoResponse, Response, Sse, sse::Event}, routing::{get, post}};
use futures::{Stream, stream};
use serde::Serialize;
use shared::{CalendarRequest, CalendarResponse, DateState, DocumentPublicationDateRequest, DocumentPublicationDateResponse, PageRequest, PageResponse, SseMessage, SseMessageType, UpdateDocumentRequest};
use tokio::{fs::create_dir_all, io::AsyncWriteExt};
use tokio_stream::StreamExt;
use tower_http::trace::{DefaultMakeSpan, TraceLayer};
use utilites::Date;
use crate::{error::AppError, state::AppState};

//use crate::{Error, api::{CollectionAddRequest, CollectionUpdateRequest, DocumentRequest, EmbeddingRequest, GenerationRequest, types::QdrantContext}, state::AppState};
use super::layers::{cors_layer};



 //.route(&super::with_api_version(super::ApiVersion::V1,"/models/{dep_id}"), 
pub fn app_router(app_state: Arc<AppState>) -> Router
{   
    Router::new()    
    //  .route(&super::with_api_version(super::ApiVersion::V1,"/documents"), 
    //      get(get_db_documents))

     .route(&super::with_api_version(super::ApiVersion::V1,"/pages"), 
         post(get_page))

    .route(&super::with_api_version(super::ApiVersion::V1,"/calendar"), 
         post(get_calendar))

    .route(&super::with_api_version(super::ApiVersion::V1,"/documents/publication_date"), 
         post(get_documents_by_publication_date))

    .route(&super::with_api_version(super::ApiVersion::V1,"/documents/update"), 
         post(update_document))

    .route(&super::with_api_version(super::ApiVersion::V1,"/events"), 
         get(sse_handler))

    //модели  
    //     .route(&super::with_api_version(super::ApiVersion::V1,"/models/load_generation_model"), 
    //     get(load_generation_model))

    //     .route(&super::with_api_version(super::ApiVersion::V1,"/models/unload_generation_model"), 
    //     get(unload_generation_model))

    //     .route(&super::with_api_version(super::ApiVersion::V1,"/models/load_embedding_model"), 
    //     get(load_embedding_model))

    //     .route(&super::with_api_version(super::ApiVersion::V1,"/models/unload_embedding_model"), 
    //     get(unload_embedding_model))

    //     .route(&super::with_api_version(super::ApiVersion::V1,"/models/get_models_state"), 
    //     get(get_models_state))
    // //документы
    //     .route(&super::with_api_version(super::ApiVersion::V1,"/documents/request_document"), 
    //     post(request_document))

    //     .route(&super::with_api_version(super::ApiVersion::V1,"/documents/{offset}"), 
    //     get(get_documents))

    //     .route(&super::with_api_version(super::ApiVersion::V1,"/documents/embedding_document"), 
    //     post(embedding_document))

    //     .route(&super::with_api_version(super::ApiVersion::V1,"/health_check"), 
    //     get(health_check))

    //     .route(&super::with_api_version(super::ApiVersion::V1,"/query/generator"), 
    //     post(generation_request))

    //     .route(&super::with_api_version(super::ApiVersion::V1,"/documents/count"), 
    //     get(documents_count))

    //     .route(&super::with_api_version(super::ApiVersion::V1,"/documents/delete_document/{hash}"), 
    //     get(delete_document))

    //     .route(&super::with_api_version(super::ApiVersion::V1,"/collections"), 
    //     get(get_collections))

    //     .route(&super::with_api_version(super::ApiVersion::V1,"/collections/add"), 
    //     post(add_collection))

    //     .route(&super::with_api_version(super::ApiVersion::V1,"/collections/delete/{id}"), 
    //     get(delete_collection))

    //     .route(&super::with_api_version(super::ApiVersion::V1,"/collections/update"), 
    //     post(update_collection))
          
    //     .route(&super::with_api_version(super::ApiVersion::V1,"/query/results"), 
    //     post(search_results))
    
        .with_state(app_state.clone())
        .layer(cors_layer(app_state))
        .layer(TraceLayer::new_for_http().make_span_with(DefaultMakeSpan::default().include_headers(true)))
}


// pub async fn get_db_documents(
//     ConnectInfo(_): ConnectInfo<SocketAddr>,
//     State(app_state): State<Arc<AppState>>,)
// -> Result<Response<Body>, AppError>
// {
//     let response = tokio::sync::oneshot::channel();
//     let _ = app_state.db_tx.send(summarization_core::DbCommand::GetAllDocuments { respond: response.0 }).await
//         .map_err(|e| AppError::InternalError(format!("Failed to send command to database service: {}", e)))?;
//     match response.1.await    
//     {
//         Ok(result) =>
//         {
//             match result
//             {
//                 Ok(docs) =>
//                 {
//                     Ok((
//                             StatusCode::OK,
//                             Json(docs)
//                         ).into_response())
//                 }
//                 Err(e) =>
//                 {
//                     return Err(AppError::InternalError(format!("Failed to get documents from database service: {}", e)));
//                 }
//              }
//         }
//         Err(e) =>
//         {
//             return Err(AppError::InternalError(format!("Failed to receive response from database service: {}", e)));
//         }
     
//     }
// }

pub async fn get_calendar(
    ConnectInfo(_): ConnectInfo<SocketAddr>,
    State(app_state): State<Arc<AppState>>,
    Json(req): Json<CalendarRequest>)
-> Result<Response<Body>, AppError>
{
    let state = app_state.summarization_service.get_calendar_state(req.from, Date::now()).await?;
    Ok((
        StatusCode::OK,
        Json(state)
    ).into_response())
}


pub async fn get_documents_by_publication_date(
    ConnectInfo(_): ConnectInfo<SocketAddr>,
    State(app_state): State<Arc<AppState>>,
    Json(req): Json<DocumentPublicationDateRequest>)
-> Result<Response<Body>, AppError>
{
    let result = app_state.summarization_service.get_documents_by_publication_date(req.publication_date.clone()).await?;
    Ok((
        StatusCode::OK,
        Json(DocumentPublicationDateResponse { documents: result, selected_date: req.publication_date })
    ).into_response())
}

pub async fn update_document(
    ConnectInfo(_): ConnectInfo<SocketAddr>,
    State(app_state): State<Arc<AppState>>,
    Json(req): Json<UpdateDocumentRequest>)
-> Result<Response<Body>, AppError>
{
    let result = app_state.summarization_service.update_document(req.document).await?;
    Ok((
        StatusCode::OK,
        Json(result)
    ).into_response())
}

pub async fn get_page(
    ConnectInfo(_): ConnectInfo<SocketAddr>,
    State(app_state): State<Arc<AppState>>,
    Json(req): Json<PageRequest>)
-> Result<Response<Body>, AppError>
{
    let page = app_state.summarization_service.publication_service.get_png(&req.id, req.page_number as u32).await?;
    let page = PageResponse
    {
        page: page.to_vec(),
        page_number: req.page_number
    };
    Ok((
        StatusCode::OK,
        Json(page),
    ).into_response())
}



async fn sse_handler(
     State(app_state): State<Arc<AppState>>,
) -> Sse<impl Stream<Item = Result<Event, Infallible>>> 
{
    let rx = app_state.summarization_service.subscribe_events();
    let stream = tokio_stream::wrappers::BroadcastStream::new(rx)
        .filter_map(|result|
        {
            match result 
            {
                Ok(sse_message) => 
                {
                    Some(Ok(Event::default()
                                .event(SseMessageType::Info)
                                .data(serde_json::to_string(&sse_message).unwrap())
                                .id(chrono::Utc::now().timestamp().to_string())))
                }
                Err(_) => None,
            }
        });
    Sse::new(stream)
}
// pub async fn delete_document(
//     ConnectInfo(_): ConnectInfo<SocketAddr>,
//     State(app_state): State<Arc<AppState>>,
//     Path(hash): Path<String>)
// -> Result<Response<Body>, Error>
// {
//     let service = app_state.get_services();
//     let _ = service.documents_service.delete_document(&hash).await?;
//     Ok((
//         StatusCode::OK,
//     ).into_response())
// }

// pub async fn documents_count(
//     ConnectInfo(_): ConnectInfo<SocketAddr>,
//     State(app_state): State<Arc<AppState>>,)
// -> Result<Response<Body>, Error>
// {
//     let service = app_state.get_services();
//     let count = service.database_service.get_documents_count().await?;
//     Ok((
//         StatusCode::OK,
//         Json(count),
//     ).into_response())
// }

// pub async fn unload_generation_model(
//     ConnectInfo(_): ConnectInfo<SocketAddr>,
//     State(app_state): State<Arc<AppState>>,)
// -> Result<Response<Body>, Error>
// {
//     let service = app_state.get_services();
//     let models_state = service.documents_service.unload_generator_model().await?;
//     Ok((
//         StatusCode::OK,
//         Json(models_state),
//     ).into_response())
// }

// pub async fn load_embedding_model(
//     ConnectInfo(_): ConnectInfo<SocketAddr>,
//     State(app_state): State<Arc<AppState>>,)
// -> Result<Response<Body>, Error>
// {
//     let service = app_state.get_services();
//     let models_state = service.documents_service.load_embedding_model().await?;
//     Ok((
//         StatusCode::OK,
//         Json(models_state),
//     ).into_response())
// }

// pub async fn unload_embedding_model(
//     ConnectInfo(_): ConnectInfo<SocketAddr>,
//     State(app_state): State<Arc<AppState>>,)
// -> Result<Response<Body>, Error>
// {
//     let service = app_state.get_services();
//     let models_state = service.documents_service.unload_embedding_model().await?;
//     Ok((
//         StatusCode::OK,
//         Json(models_state),
//     ).into_response())
// }

// pub async fn get_models_state(
//     ConnectInfo(_): ConnectInfo<SocketAddr>,
//     State(app_state): State<Arc<AppState>>,)
// -> Result<Response<Body>, Error>
// {
//     let service = app_state.get_services();
//     let models_state = service.rag_service.models_state().await;
//     Ok((
//         StatusCode::OK,
//         Json(models_state),
//     ).into_response())
// }

// pub async fn request_document(
//     ConnectInfo(_): ConnectInfo<SocketAddr>,
//     State(app_state): State<Arc<AppState>>,
//     Json(req): Json<DocumentRequest>)
// -> Result<Response<Body>, Error>
// {
//     let service = app_state.get_services();
//     let doc = service.documents_service.get_document_and_add_to_db(req.sign_date, &req.number, req.collection_id).await?;
//     Ok((
//         StatusCode::OK,
//         Json(doc),
//     ).into_response())
// }
// pub async fn health_check(
//     ConnectInfo(_): ConnectInfo<SocketAddr>,
//     State(_): State<Arc<AppState>>,)
// -> Result<Response<Body>, Error>
// {
//     Ok((
//         StatusCode::OK,
//     ).into_response())
// }

// pub async fn get_documents(
//     ConnectInfo(_): ConnectInfo<SocketAddr>,
//     State(app_state): State<Arc<AppState>>,
//     Path(offset): Path<u32>)
// -> Result<Response<Body>, Error>
// {
//     let service = app_state.get_services();
//     let docs = service.documents_service.get_documents_list(offset, 30).await?;
//     Ok((
//         StatusCode::OK,
//         Json(docs),
//     ).into_response())
// }
// pub async fn embedding_document(
//     ConnectInfo(_): ConnectInfo<SocketAddr>,
//     State(app_state): State<Arc<AppState>>,
//      Json(req): Json<EmbeddingRequest>)
// -> Result<Response<Body>, Error>
// {
//     let service = app_state.get_services();
//     let _ = service.documents_service.embedding_document_from_sqlite(&req.hash, req.collection_id).await?;
//     Ok((
//         StatusCode::OK,
//     ).into_response())
// }

// pub async fn generation_request(
//     ConnectInfo(_): ConnectInfo<SocketAddr>,
//     //TODO сделать экстракотры проверки pow и capcha и в том числе добавить сами эти сервисы
//     State(app_state): State<Arc<AppState>>,
//     Json(query): Json<GenerationRequest>)
// -> Result<Response<Body>, Error>
// {
//     let service = app_state.get_services();
//     let search_result = service.documents_service.search_context(&query.query, query.per_collection_limit, query.final_limit).await?;
//     let _ = service.documents_service.generate_result(&query.query, search_result).await?;
//     Ok((
//         StatusCode::OK,
//     ).into_response())
// }


// pub async fn search_results(
//     ConnectInfo(_): ConnectInfo<SocketAddr>,
//     State(app_state): State<Arc<AppState>>,
//     Json(query): Json<GenerationRequest>)
// -> Result<Response<Body>, Error>
// {
//     let service = app_state.get_services();
//     let search_result = service.documents_service.search_context(&query.query, query.per_collection_limit, query.final_limit).await?;
//     let search_result: Vec<QdrantContext> = search_result.into_iter().map(|v|
//     {
//         v.into()
//     }).collect();
//     Ok((
//         StatusCode::OK,
//         Json(search_result),
//     ).into_response())
// }


// pub async fn get_collections(
//     ConnectInfo(_): ConnectInfo<SocketAddr>,
//     State(app_state): State<Arc<AppState>>)
// -> Result<Response<Body>, Error>
// {
//     let service = app_state.get_services();
//     let collections = service.database_service.get_collections().await?;
//     Ok((
//         StatusCode::OK,
//         Json(collections),
//     ).into_response())
// }

// pub async fn add_collection(
//     ConnectInfo(_): ConnectInfo<SocketAddr>,
//     State(app_state): State<Arc<AppState>>,
//     Json(req): Json<CollectionAddRequest>)
// -> Result<Response<Body>, Error>
// {
//     let service = app_state.get_services();
//     let collection = service.database_service.add_collection(&req.name, &req.description).await?;
//     Ok((
//         StatusCode::OK,
//         Json(collection),
//     ).into_response())
// }

// pub async fn update_collection(
//     ConnectInfo(_): ConnectInfo<SocketAddr>,
//     State(app_state): State<Arc<AppState>>,
//     Json(req): Json<CollectionUpdateRequest>)
// -> Result<Response<Body>, Error>
// {
//     let service = app_state.get_services();
//     let _ = service.database_service.update_collection(req.id, &req.description).await?;
//     Ok((
//         StatusCode::OK,
//     ).into_response())
// }

// pub async fn delete_collection(
//     ConnectInfo(_): ConnectInfo<SocketAddr>,
//     State(app_state): State<Arc<AppState>>,
//     Path(id): Path<uuid::Uuid>)
// -> Result<Response<Body>, Error>
// {
//     let service = app_state.get_services();
//     let _ = service.database_service.delete_collection(id).await?;
//     Ok((
//         StatusCode::OK,
//     ).into_response())
// }