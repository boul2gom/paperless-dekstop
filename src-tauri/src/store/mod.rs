pub mod local;

use std::sync::Arc;
use paperless_rs::endpoint::documents::Document;

pub type MemoryCache = moka::sync::Cache<String, Cached>;

#[derive(Debug, Clone, PartialEq, PartialOrd)]
pub enum Cached {
    Document(Arc<Document>),
    Ids(Vec<u64>),

    Search(Vec<Arc<Document>>),
    Version(String),
}

#[derive(Debug)]
pub enum Error {
    NotFound,
    ExpiredCache,

    IO(String),
    Serialize(String),
    Deserialize(String),
}