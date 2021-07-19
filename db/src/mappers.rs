use crate::sql_state::SqlState;
use cardbox_core::contracts::repo::UserCreateError;
use sqlx::postgres::PgDatabaseError;

pub fn sqlx_error_to_user_create_error(error: sqlx::Error) -> UserCreateError {
    use sqlx::Error as SqlxError;

    if let SqlxError::Database(ref e) = error {
        let pg_err = e.downcast_ref::<PgDatabaseError>();
        if pg_err.code() == SqlState::UNIQUE_VIOLATION.code() {
            return UserCreateError::UserAlreadyExists;
        }
    }

    UserCreateError::UnexpectedFailure(error.into())
}
