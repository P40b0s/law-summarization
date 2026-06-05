mod logger;
mod error;
mod state;

use arc_swap::ArcSwap;
use axum::{extract::Path, http::StatusCode, response::IntoResponse, routing::{get, post}, Extension, Json, Router};
use publication_client::{ExtendedPublicationDocumentCard, PublicationApiClient, ReqwestPublicationApiClient};
use serde::{Deserialize, Serialize};
use sqlx::{sqlite::SqlitePoolOptions, FromRow, SqlitePool};
use summarization_core::{DbCommand, PublicationService};
use std::{net::SocketAddr, sync::Arc};
use tokio::sync::{mpsc, oneshot};
use tracing::{debug, error, info};
mod api;
use state::AppState;
use api::router;
mod configuration;
use crate::configuration::Configuration;


#[tokio::main]
async fn main() -> anyhow::Result<()> 
{
    // tracing_subscriber::fmt()
    //     .with_env_filter("summarization_core=info,api=info,tower_http=warn")
    //     .init();
    logger::init();
    let conf = Configuration::new()?;
    let config = ArcSwap::new(Arc::new(conf));
    //TODO разобраться потом с динамическим изменением конфигурации
    let core_config = Arc::new(config.load().core_configuration.clone());
    let publication_service = PublicationService::<ReqwestPublicationApiClient>::create_reqwest_service(core_config.clone());
    let publication_service = Arc::new(publication_service);
    let (db_tx, db_rx) = tokio::sync::mpsc::channel(16);
    tokio::spawn(summarization_core::start_database_service( db_rx));
    let ai_service = Arc::new(summarization_core::AiService::new("qwen3.5".to_owned(), core_config.clone()));
    // tokio::spawn(
    // async move {
    //         summarization_core::run_service(core_config)
    //             .await.inspect_err(|e| error!("Error {:?}", e))
    // });
    let app_state = AppState {
        configuration: config.load().clone(),
        publication_service: publication_service,
        ai_service: ai_service,
        db_tx,
    };

    // let app = Router::new()
    //     .route("/", get(root_handler))
    //     .route("/summarize", post(summarize_handler))
    //     .route("/summary/:eo_number", get(get_summary_handler))
    //     .layer(Extension(Arc::new(app_state.clone())));
    let addr = SocketAddr::from(([0, 0, 0, 0], config.load().server_port));
    debug!("Апи сервера доступно на {}", &addr);
    let listener = tokio::net::TcpListener::bind(addr).await.unwrap();
    axum::serve(listener, router(Arc::new(app_state)).into_make_service_with_connect_info::<SocketAddr>()).await.unwrap();
    Ok(())
}

// async fn init_db(pool: &SqlitePool) -> anyhow::Result<()> {
//     sqlx::query(
//         r#"CREATE TABLE IF NOT EXISTS summaries (
//             eo_number TEXT PRIMARY KEY,
//             summary TEXT NOT NULL,
//             created_at TEXT NOT NULL DEFAULT (strftime('%Y-%m-%dT%H:%M:%fZ','now'))
//         );"#,
//     )
//     .execute(pool)
//     .await?;

//     Ok(())
// }

// async fn root_handler() -> impl IntoResponse {
//     (StatusCode::OK, Json(serde_json::json!({ "message": "Law summarization service is running" })))
// }

// async fn summarize_handler(
//     Extension(state): Extension<Arc<AppState>>,
//     Json(request): Json<SummarizeRequest>,
// ) -> Result<Json<SummarizeResponse>, (StatusCode, Json<ApiError>)> {
//     let request_eo = request.eo_number.trim().to_owned();
//     if request_eo.is_empty() {
//         return Err((
//             StatusCode::BAD_REQUEST,
//             Json(ApiError {
//                 message: "eo_number cannot be empty".to_owned(),
//             }),
//         ));
//     }

//     if let Some(record) = query_summary(&state.db_tx, &request_eo).await.map_err(api_error)? {
//         return Ok(Json(SummarizeResponse {
//             eo_number: record.eo_number,
//             summary: record.summary,
//             cached: true,
//         }));
//     }

//     let document = fetch_document(&state.publication_tx, &request_eo)
//         .await
//         .map_err(api_error)?;

//     let summary = generate_summary(&document);
//     save_summary(&state.db_tx, &request_eo, &summary)
//         .await
//         .map_err(api_error)?;

//     Ok(Json(SummarizeResponse {
//         eo_number: request_eo,
//         summary,
//         cached: false,
//     }))
// }

// async fn get_summary_handler(
//     Extension(state): Extension<Arc<AppState>>,
//     Path(eo_number): Path<String>,
// ) -> Result<Json<SummarizeResponse>, (StatusCode, Json<ApiError>)> {
//     let eo_number = eo_number.trim().to_owned();
//     if eo_number.is_empty() {
//         return Err((
//             StatusCode::BAD_REQUEST,
//             Json(ApiError {
//                 message: "eo_number path parameter cannot be empty".to_owned(),
//             }),
//         ));
//     }

//     let record = query_summary(&state.db_tx, &eo_number).await.map_err(api_error)?;
//     match record {
//         Some(record) => Ok(Json(SummarizeResponse {
//             eo_number: record.eo_number,
//             summary: record.summary,
//             cached: true,
//         })),
//         None => Err((
//             StatusCode::NOT_FOUND,
//             Json(ApiError {
//                 message: format!("Summary not found for eo_number: {}", eo_number),
//             }),
//         )),
//     }
// }

// async fn query_summary(
//     db_tx: &DbTx,
//     eo_number: &str,
// ) -> anyhow::Result<Option<SummaryRecord>> {
//     let (respond_tx, respond_rx) = oneshot::channel();
//     db_tx
//         .send(DbCommand::FetchSummary {
//             eo_number: eo_number.to_owned(),
//             respond: respond_tx,
//         })
//         .await?;

//     respond_rx.await?.map_err(|err| err.context("database response failed"))
// }

// async fn save_summary(db_tx: &DbTx, eo_number: &str, summary: &str) -> anyhow::Result<()> {
//     let (respond_tx, respond_rx) = oneshot::channel();
//     db_tx
//         .send(DbCommand::SaveSummary {
//             eo_number: eo_number.to_owned(),
//             summary: summary.to_owned(),
//             respond: respond_tx,
//         })
//         .await?;

//     respond_rx.await?.map_err(|err| err.context("database response failed"))
// }

// async fn fetch_document(
//     publication_tx: &PublicationTx,
//     eo_number: &str,
// ) -> anyhow::Result<ExtendedPublicationDocumentCard> {
//     let (respond_tx, respond_rx) = oneshot::channel();
//     publication_tx
//         .send(PublicationCommand::FetchExtendedCard {
//             eo_number: eo_number.to_owned(),
//             respond: respond_tx,
//         })
//         .await?;

//     respond_rx.await?.map_err(|err| err.context("publication service response failed"))
// }

// fn generate_summary(document: &ExtendedPublicationDocumentCard) -> String {
//     let mut summary = format!(
//         "{} — {}",
//         document.title.clone().trim(),
//         document.complex_name.clone().trim()
//     );

//     if summary.len() > 400 {
//         summary.truncate(400);
//         summary.push_str("...");
//     }

//     format!(
//         "Document {eo} summary: {summary}",
//         eo = document.eo_number,
//         summary = summary
//     )
// }

// fn api_error(error: anyhow::Error) -> (StatusCode, Json<ApiError>) {
//     error!(%error, "request failed");
//     (
//         StatusCode::INTERNAL_SERVER_ERROR,
//         Json(ApiError {
//             message: format!("Internal server error: {}", error),
//         }),
//     )
// }

// async fn publication_service<C>(
//     client: C,
//     mut receiver: mpsc::Receiver<PublicationCommand>,
// ) where
//     C: PublicationApiClient + Send + Sync + 'static,
// {
//     while let Some(command) = receiver.recv().await {
//         match command {
//             PublicationCommand::FetchExtendedCard { eo_number, respond } => {
//                 let result = client.get_document_extended_card(&eo_number).await;
//                 let _ = respond.send(result);
//             }
//         }
//     }
// }

// async fn db_service(pool: SqlitePool, mut receiver: mpsc::Receiver<DbCommand>) {
//     while let Some(command) = receiver.recv().await {
//         match command {
//             DbCommand::FetchSummary { eo_number, respond } => {
//                 let result = sqlx::query_as::<_, SummaryRecord>(
//                     "SELECT eo_number, summary, created_at FROM summaries WHERE eo_number = ?",
//                 )
//                 .bind(&eo_number)
//                 .fetch_optional(&pool)
//                 .await
//                 .map_err(anyhow::Error::from);
//                 let _ = respond.send(result);
//             }
//             DbCommand::SaveSummary {
//                 eo_number,
//                 summary,
//                 respond,
//             } => {
//                 let result = sqlx::query(
//                     "INSERT OR REPLACE INTO summaries (eo_number, summary) VALUES (?, ?)",
//                 )
//                 .bind(&eo_number)
//                 .bind(&summary)
//                 .execute(&pool)
//                 .await
//                 .map(|_| ())
//                 .map_err(anyhow::Error::from);
//                 let _ = respond.send(result);
//             }
//         }
//     }
// }
// mod configuration;
