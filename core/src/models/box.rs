use uuid::Uuid;

#[derive(Debug)]
pub struct Box {
    pub id: Uuid,
    pub user_id: Uuid,
    pub _type: BoxType,
}

#[derive(Debug)]
pub enum BoxType {
    User,
}
