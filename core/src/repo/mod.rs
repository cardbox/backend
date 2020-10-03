mod session_token;
mod user;

pub use session_token::*;
pub use user::*;

#[derive(PartialEq, Debug, Clone, Eq)]
pub struct UnexpectedError;

pub type RepoResult<T> = Result<T, UnexpectedError>;
