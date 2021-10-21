use cardbox_core::models;
use uuid::Uuid;

#[derive(Debug, sqlx::FromRow)]
pub(crate) struct Box {
    pub(crate) id: Uuid,
    pub(crate) user_id: Uuid,
    #[sqlx(rename = "type")]
    pub(crate) r#type: BoxType,
    pub(crate) default: bool,
}

#[derive(Debug, sqlx::Type)]
#[sqlx(type_name = "box_type", rename_all = "snake_case")]
pub(crate) enum BoxType {
    User,
}

impl From<Box> for models::Box {
    fn from(b: Box) -> Self {
        Self {
            id: b.id,
            user_id: b.user_id,
            _type: b.r#type.into(),
            default: b.default,
        }
    }
}

impl From<BoxType> for models::BoxType {
    fn from(t: BoxType) -> Self {
        match t {
            BoxType::User => models::BoxType::User,
        }
    }
}
