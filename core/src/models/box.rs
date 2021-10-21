use uuid::Uuid;

#[derive(Debug, Clone)]
pub struct Box {
    pub id: Uuid,
    pub user_id: Uuid,
    pub _type: BoxType,
    pub default: bool,
}

#[derive(Debug, Clone)]
pub enum BoxType {
    User,
}

#[cfg(feature = "testing")]
impl Box {
    pub fn create_random() -> Self {
        Self {
            _type: BoxType::User,
            user_id: Uuid::new_v4(),
            id: Uuid::new_v4(),
            default: false,
        }
    }
}
