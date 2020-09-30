#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub struct User {
    pub(crate) id: uuid::Uuid,
    pub(crate) first_name: String,
    pub(crate) last_name: String,
}

impl User {
    pub fn id(&self) -> uuid::Uuid {
        self.id
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
