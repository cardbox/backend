use uuid::Uuid;

#[derive(Debug, Clone)]
pub struct Social {
    pub id: Uuid,
    pub user_id: Uuid,
    pub name: String,
    pub link: String,
}
