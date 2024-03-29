use crate::app::UserInfo;
use crate::models::Social;

/// TODO: how to guarantee model validity
#[derive(Debug, Clone)]
pub struct User {
    pub id: uuid::Uuid,
    pub accesso_id: uuid::Uuid,
    pub first_name: String,
    pub last_name: String,
    pub username: Option<String>,
    pub bio: Option<String>,
    pub avatar: Option<String>,
    pub work: Option<String>, // pub registered_at: chrono::NaiveDateTime,
    pub socials: Option<Vec<Social>>,
}

#[derive(Debug, Clone)]
pub struct SessionUser {
    pub avatar: Option<String>,
    pub id: uuid::Uuid,
    pub accesso_id: uuid::Uuid,
    pub first_name: String,
    pub last_name: String,
    pub username: Option<String>,
    pub expired: bool,
}

#[derive(Debug)]
pub struct UserCreate {
    pub accesso_id: uuid::Uuid,
    pub first_name: String,
    pub last_name: String,
}

impl From<UserInfo> for UserCreate {
    #[inline]
    fn from(i: UserInfo) -> Self {
        Self {
            accesso_id: i.accesso_id,
            first_name: i.first_name,
            last_name: i.last_name,
        }
    }
}

impl User {
    pub fn new(accesso_id: uuid::Uuid) -> Self {
        Self {
            id: uuid::Uuid::new_v4(),
            accesso_id,
            first_name: String::new(),
            last_name: String::new(),
            username: None,
            bio: None,
            avatar: None,
            work: None,
            socials: None,
        }
    }
}

#[cfg(feature = "testing")]
impl User {
    pub fn create_random() -> Self {
        use rand::distributions::Alphanumeric;
        use rand::{thread_rng, Rng};

        fn random_string(length: usize) -> String {
            thread_rng()
                .sample_iter(&Alphanumeric)
                .take(length)
                .map(char::from)
                .collect()
        }

        Self {
            id: uuid::Uuid::new_v4(),
            accesso_id: uuid::Uuid::new_v4(),
            first_name: random_string(12),
            last_name: random_string(12),
            username: None,
            bio: None,
            avatar: None,
            work: None,
            socials: None,
        }
    }
}
