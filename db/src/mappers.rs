use crate::sql_state::SqlState;
use cardbox_core::app::CardSaveError;
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

pub fn sqlx_error_to_card_save_error(error: sqlx::Error) -> CardSaveError {
    use sqlx::Error as SqlxError;

    if let SqlxError::Database(ref e) = error {
        let pg_err = e.downcast_ref::<PgDatabaseError>();
        if pg_err.code() == SqlState::UNIQUE_VIOLATION.code() {
            return CardSaveError::AlreadySaved;
        }
    }

    CardSaveError::Unexpected(error.into())
}
