mod card;
mod session_token;
mod user;

pub use card::*;
pub use session_token::*;
pub use user::*;

#[cfg(feature = "testing")]
pub struct MockDb {
    pub users: MockUserRepo,
    pub session_tokens: MockSessionTokenRepo,
    pub cards: MockCardRepo,
}

#[cfg(feature = "testing")]
impl Default for MockDb {
    fn default() -> Self {
        Self::new()
    }
}

#[cfg(feature = "testing")]
impl MockDb {
    pub fn new() -> Self {
        Self {
            users: MockUserRepo::new(),
            session_tokens: MockSessionTokenRepo::new(),
            cards: MockCardRepo::new(),
        }
    }
}

#[derive(Debug, thiserror::Error)]
pub enum UnexpectedDatabaseError {
    #[error("Unexpected database error: {0}")]
    SqlxError(#[from] sqlx_core::error::Error),
}

pub type RepoResult<T> = Result<T, UnexpectedDatabaseError>;
