use chrono::{DateTime, Utc};
use sqlx_core::types::Json;
use uuid::Uuid;

#[derive(Debug, Clone)]
pub struct Card {
    pub id: Uuid,
    pub author_id: Uuid,
    pub title: String,
    pub created_at: DateTime<Utc>,
    pub updated_at: DateTime<Utc>,
    pub contents: serde_json::Value,
    pub tags: Vec<String>,
}

#[derive(Debug)]
pub struct CardCreate<'a> {
    pub author_id: Uuid,
    pub title: String,
    pub contents: Json<&'a serde_json::value::RawValue>,
    pub tags: Vec<String>,
}

#[derive(Debug)]
pub struct CardUpdate<'a> {
    pub id: Uuid,
    pub title: Option<String>,
    pub contents: Option<Json<&'a serde_json::value::RawValue>>,
    pub tags: Option<Vec<String>>,
}
