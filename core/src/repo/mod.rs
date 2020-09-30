mod user;

pub use user::*;

#[derive(PartialEq, Debug, Clone, Eq)]
pub struct UnexpectedError;

pub type RepoResult<T> = Result<T, UnexpectedError>;
