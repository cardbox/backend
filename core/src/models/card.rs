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

#[cfg(feature = "testing")]
impl Card {
    pub fn create_random() -> Self {
        use chrono::TimeZone;
        use rand::distributions::Alphanumeric;
        use rand::{thread_rng, Rng};
        use serde_json::json;

        let date = Utc.timestamp(thread_rng().gen_range(1431648000..1500000000), 0);

        fn random_string(length: usize) -> String {
            thread_rng()
                .sample_iter(&Alphanumeric)
                .take(length)
                .map(char::from)
                .collect()
        }

        Self {
            id: Uuid::new_v4(),
            author_id: Uuid::new_v4(),
            title: random_string(12),
            created_at: date,
            updated_at: date,
            contents: json!([]),
            tags: vec![],
        }
    }
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
