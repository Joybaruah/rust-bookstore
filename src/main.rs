use chrono::prelude::*;
use db::DB;
use serde::{Deserialize, Serialize};
use std::convert::Infallible;

#[derive(Serialize, Deserialize, Debug)]
pub struct Book {
    pub id: String,
    pub name: String,
    pub author: String,
    pub pages: usize,
    pub created_at: DateTime<Utc>,
    pub tags: Vec<String>,
}

#[tokio::main]
async fn main() -> Result<()> {

}