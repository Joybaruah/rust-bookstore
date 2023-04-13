use crate::{error::Error::*, handler::BookRequest, Book, Result};

use chrono::prelude::*;
use futures::StreamExt;

use mongodb::bson::{doc, document::Document, oid::ObjectId, Bson};
use mongodb::{options::ClientOptions, Client, Collection};

const DB_NAME: &str = "rust-bookstore";
const COLL: &str = "books";

const ID: &str = "_id";
const NAME: &str = "name";
const AUTHOR: &str = "author";
const NUM_PAGES: &str = "num_pages";
const CREATED_AT: &str = "created_at";
const TAGS: &str = "tags";

#[derive(Clone, Debug)]
pub struct DB{
    pub client: Client,   
}

impl DB {
    pub async fn init() -> Result<Self>{
        let mut client_options = ClientOptions::parse("mongodb://127.0.0.1:27017").await?;
        client_options.app_name = Some("rust-bookstore".to_string());

        Ok(Self { client: Client::with_options(client_options)?, })
    }
}