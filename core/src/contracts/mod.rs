pub mod generator;
pub mod repo;

pub use generator::*;
pub use repo::*;

pub trait Repository: UserRepo + SessionTokenRepo + CardRepo + BoxRepo + Send + Sync {}

impl<T> Repository for T where T: UserRepo + SessionTokenRepo + CardRepo + BoxRepo + Send + Sync {}
