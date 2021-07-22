use cardbox_core::models;
use chrono::{DateTime, Utc};
use uuid::Uuid;

#[derive(Debug, sqlx::FromRow, sqlx::Type)]
pub(crate) struct Card {
    pub(crate) id: Uuid,
    pub(crate) author_id: Uuid,
    pub(crate) title: String,
    pub(crate) created_at: DateTime<Utc>,
    pub(crate) updated_at: DateTime<Utc>,
    pub(crate) contents: serde_json::Value,
    pub(crate) tags: Option<Vec<String>>,
}

impl From<Card> for models::Card {
    fn from(card: Card) -> Self {
        Self {
            id: card.id,
            author_id: card.author_id,
            title: card.title,
            created_at: card.created_at,
            updated_at: card.updated_at,
            contents: card.contents,
            tags: card.tags.unwrap_or_else(Vec::new),
        }
    }
}

impl From<models::Card> for Card {
    fn from(card: models::Card) -> Self {
        let tags = if card.tags.is_empty() {
            None
        } else {
            Some(card.tags)
        };

        Self {
            id: card.id,
            author_id: card.author_id,
            title: card.title,
            created_at: card.created_at,
            updated_at: card.updated_at,
            contents: card.contents,
            tags,
        }
    }
}
