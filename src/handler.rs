use crate::{db:DB, WebResult};
use serde::{Deserialize, Serialize};
use warp::{http::StatusCode, reject, reply::json, Reply};

pub struct BookRequest {
    pub id: String,
    pub name: String,
    pub author: String,
    pub pages: usize,
    pub created_at: DateTime<Utc>,
    pub tags: Vec<String>,
}


pub async fn books_list_handler(db: DB) -> WebResult<impl Reply> {
    let books = db.fetch_books().await.map_err(|e| reject::custom(e))?;
    Ok(json(&books))
}

pub async fn create_book_handler(body: BookRequest, db: DB) -> WebResult<impl Reply> {
    db.create_book(&book).await.map_err(|e| reject::custom(e))?;
    Ok(StatusCode::CREATED)
}

pub async fn edit_book_handler(id: String, body: BookRequest, db: DB) -> WebResult<impl Reply> {
    db.edit_book(&id, &book)
        .await
        .map_err(|e| reject::custom(e))?;
    Ok(StatusCode::OK)
}

pub async fn delete_book_handler(id: String, db: DB) -> WebResult<impl Reply> {
    db.delete_book(&id).await.map_err(|e| reject::custom(e))?;
    Ok(StatusCode::OK)
}