/// TODO: how to guarantee model validity
#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub struct User {
    pub id: uuid::Uuid,
    pub accesso_id: uuid::Uuid,
    pub first_name: String,
    pub last_name: String,
    // pub registered_at: chrono::NaiveDateTime,
}

impl User {
    pub fn new(accesso_id: uuid::Uuid) -> Self {
        Self {
            id: uuid::Uuid::new_v4(),
            accesso_id,
            first_name: String::new(),
            last_name: String::new(),
        }
    }

    pub fn id(&self) -> uuid::Uuid {
        self.id
    }

    pub fn accesso_id(&self) -> uuid::Uuid {
        self.accesso_id
    }

    pub fn first_name(&self) -> String {
        self.first_name.clone()
    }

    pub fn last_name(&self) -> String {
        self.last_name.clone()
    }

    pub(crate) fn set_first_name(&mut self, first_name: String) -> &mut Self {
        self.first_name = first_name;
        self
    }

    pub(crate) fn set_last_name(&mut self, last_name: String) -> &mut Self {
        self.last_name = last_name;
        self
    }
}
