use chrono::{DateTime, Utc};
use uuid::Uuid;

pub struct Card {
    pub id: Uuid,
    pub user_id: Uuid,
    pub title: String,
    pub created_at: DateTime<Utc>,
    pub updated_at: DateTime<Utc>,
    pub contents: serde_json::Value,
    pub tags: Vec<String>,
}

pub struct CardCreate {
    pub id: Uuid,
    pub user_id: Uuid,
    pub title: String,
    pub contents: Option<serde_json::Value>,
    pub tags: Vec<String>,
}
