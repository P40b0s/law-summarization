
use std::sync::Arc;
use serde::{Deserialize, Serialize};
use sqlx::{Row, SqlitePool, prelude::FromRow, sqlite::SqliteRow};
use tracing::{error, warn};
use anyhow::{Context, Result};
use utilites::Date;

#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct DocumentsDbo
{
    pub eo_number: String,
    pub publication_date: Date,
    pub doc_id: String,
    pub summarization_text: Option<String>,
    pub complex_name: String,
    pub checked_time: Option<Date>,
    pub unloaded: bool,
    pub pages_count: i32,
}

impl FromRow<'_, SqliteRow> for DocumentsDbo 
{
    fn from_row(row: &SqliteRow) -> sqlx::Result<Self> 
    {
        let eo_number: String =  row.try_get("eo_number")?;
        let doc_id: String = row.try_get("doc_id")?;
        let summarization_text: Option<String> =  row.try_get("summarization_text")?;
        let complex_name: String = row.try_get("complex_name")?;
        let publication_date: String = row.try_get("publication_date")?;
        let publication_date = Date::parse(publication_date).context("Failed to parse publication date").unwrap();
        let checked_time: Option<String> = row.try_get("checked_time")?;
        let checked_time = checked_time.and_then(|ct| Date::parse(ct));
        let unloaded: bool = row.try_get("unloaded")?;
        let pages_count: i32 = row.try_get("pages_count")?;
        let obj = DocumentsDbo
        {
            eo_number,
            doc_id,
            summarization_text,
            complex_name,
            checked_time,
            unloaded,
            publication_date,
            pages_count,
        };
        Ok(obj)
    }
}

pub struct DocumentsTable
{
    connection: Arc<SqlitePool>,
}

impl DocumentsTable
{
     fn create_code() -> &'static str 
    {
        "BEGIN;
        CREATE TABLE IF NOT EXISTS documents (
        eo_number TEXT NOT NULL,
        doc_id TEXT NOT NULL,
        summarization_text TEXT,
        complex_name TEXT NOT NULL,
        checked_time TEXT,
        unloaded BOOLEAN NOT NULL DEFAULT 0,
        publication_date TEXT NOT NULL,
        pages_count INTEGER NOT NULL DEFAULT 0,
        PRIMARY KEY(eo_number)
        );
        COMMIT;"
    }

    pub async fn new(pool: Arc<SqlitePool>) -> Result<Self>
    {
        let r1 = sqlx::query(Self::create_code()).execute(&*pool).await;
        if r1.is_err()
        {
            error!("{}", r1.as_ref().err().unwrap());
            let _ = r1?;
        };
        Ok(Self
        {
            connection: pool,
        })
    }
    pub async fn new_default(db_name: &str) -> Result<Self>
    {
        let pool = super::connection::new_connection(db_name).await?;
        let table = Self::new(Arc::new(pool)).await?;
        Ok(table)
    }

    /// Добавить новый документ в БД
    pub async fn insert(&self, doc: &DocumentsDbo) -> Result<()>
    {
        sqlx::query(
            "INSERT INTO documents (eo_number, doc_id, summarization_text, complex_name, checked_time, unloaded, publication_date, pages_count) 
             VALUES (?, ?, ?, ?, ?, ?, ?, ?)"
        )
        .bind(&doc.eo_number)
        .bind(&doc.doc_id)
        .bind(&doc.summarization_text)
        .bind(&doc.complex_name)
        .bind(doc.checked_time.as_ref().map(|d| d.to_string()))
        .bind(doc.unloaded)
        .bind(doc.publication_date.format(utilites::DateFormat::SerializeDate))
        .bind(doc.pages_count)
        .execute(&*self.connection)
        .await
        .context("Failed to insert document")?;
        Ok(())
    }

    /// Получить документ по eo_number
    pub async fn get_by_id(&self, eo_number: &str) -> Result<Option<DocumentsDbo>>
    {
        let result = sqlx::query_as::<_, DocumentsDbo>(
            "SELECT eo_number, doc_id, summarization_text, complex_name, checked_time, unloaded, publication_date, pages_count FROM documents WHERE eo_number = ?"
        )
        .bind(eo_number)
        .fetch_optional(&*self.connection)
        .await
        .context("Failed to fetch document")?;
        Ok(result)
    }

    /// Получить документы по дате публикации
    pub async fn get_by_publication_date(&self, publication_date: &utilites::Date) -> Result<Vec<DocumentsDbo>>
    {
        let results = sqlx::query_as::<_, DocumentsDbo>(
            "SELECT eo_number, doc_id, summarization_text, complex_name, checked_time, unloaded, publication_date, pages_count FROM documents WHERE publication_date = ?"
        )
        .bind(publication_date.format(utilites::DateFormat::SerializeDate))
        .fetch_all(&*self.connection)
        .await
        .context("Failed to fetch documents")?;
        Ok(results)
    }

    /// Обновить документ
    pub async fn update(&self, doc: &DocumentsDbo) -> Result<()>
    {
        let rows_affected = sqlx::query(
            "UPDATE documents SET doc_id = ?, summarization_text = ?, complex_name = ?, checked_time = ?, unloaded = ?, publication_date = ? WHERE eo_number = ?"
        )
        .bind(&doc.doc_id)
        .bind(&doc.summarization_text)
        .bind(&doc.complex_name)
        .bind(doc.checked_time.as_ref().map(|d| d.to_string()))
        .bind(doc.unloaded)
        .bind(doc.publication_date.format(utilites::DateFormat::SerializeDate))
        .bind(&doc.eo_number)
        .execute(&*self.connection)
        .await
        .context("Failed to update document")?
        .rows_affected();

        if rows_affected == 0 {
            warn!("No document found with eo_number: {}", doc.eo_number);
        }
        Ok(())
    }

    /// Удалить документ по eo_number
    pub async fn delete(&self, eo_number: &str) -> Result<()>
    {
        let rows_affected = sqlx::query("DELETE FROM documents WHERE eo_number = ?")
        .bind(eo_number)
        .execute(&*self.connection)
        .await
        .context("Failed to delete document")?
        .rows_affected();

        if rows_affected == 0 {
            warn!("No document found with eo_number: {}", eo_number);
        }
        Ok(())
    }

    /// Получить все документы
    pub async fn get_all(&self) -> Result<Vec<DocumentsDbo>>
    {
        let results = sqlx::query_as::<_, DocumentsDbo>(
            "SELECT eo_number, doc_id, summarization_text, complex_name, checked_time, unloaded, publication_date, pages_count FROM documents"
        )
        .fetch_all(&*self.connection)
        .await
        .context("Failed to fetch all documents")?;
        Ok(results)
    }

    /// Установить флаг `unloaded` для документа
    pub async fn set_unloaded(&self, eo_number: &str, unloaded: bool) -> Result<()> {
        let rows_affected = sqlx::query("UPDATE documents SET unloaded = ? WHERE eo_number = ?")
            .bind(unloaded)
            .bind(eo_number)
            .execute(&*self.connection)
            .await
            .context("Failed to set unloaded flag")?
            .rows_affected();
        if rows_affected == 0 {
            warn!("No document found with eo_number: {}", eo_number);
        }
        Ok(())
    }

    pub async fn set_summary(&self, eo_number: &str, summary: String) -> Result<()> {
        let rows_affected = sqlx::query("UPDATE documents SET summarization_text = ? WHERE eo_number = ?")
            .bind(summary)
            .bind(eo_number)
            .execute(&*self.connection)
            .await
            .context("Failed to set summary")?
            .rows_affected();
        if rows_affected == 0 {
            warn!("No document found with eo_number: {}", eo_number);
        }
        Ok(())
    }

    /// Получить все документы с `unloaded = true`
    pub async fn get_unloaded(&self) -> Result<Vec<DocumentsDbo>> {
        let results = sqlx::query_as::<_, DocumentsDbo>(
            "SELECT eo_number, doc_id, summarization_text, complex_name, checked_time, unloaded, publication_date, pages_count FROM documents WHERE unloaded = 1"
        )
        .fetch_all(&*self.connection)
        .await
        .context("Failed to fetch unloaded documents")?;
        Ok(results)
    }

    /// Обновить поле `checked_time` (может быть `None` для очистки)
    pub async fn set_checked_time(&self, eo_number: &str, checked_time: Option<Date>) -> Result<()> {
        let rows_affected = sqlx::query("UPDATE documents SET checked_time = ? WHERE eo_number = ?")
            .bind(checked_time.as_ref().map(|d| d.to_string()))
            .bind(eo_number)
            .execute(&*self.connection)
            .await
            .context("Failed to set checked_time")?
            .rows_affected();
        if rows_affected == 0 {
            warn!("No document found with eo_number: {}", eo_number);
        }
        Ok(())
    }
}