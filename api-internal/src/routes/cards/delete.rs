use crate::generated::{
    components::{
        request_bodies::CardsDeleteRequestBody,
        responses::{CardsDeleteError, CardsDeleteFailed, CardsDeleteSuccess},
    },
    paths::cards_delete::{Error, Response},
};
use actix_web::web::{self, Data};
use cardbox_core::app::extractors::SessionToken;
use cardbox_core::app::{self, Cards};

pub async fn route(
    app: Data<cardbox_app::App>,
    body: web::Json<CardsDeleteRequestBody>,
    session_token: SessionToken,
) -> Result<Response, Error> {
    let body = body.into_inner();

    let card_id = app
        .card_delete(body.card_id, session_token.into_inner())
        .await
        .map_err(map_card_delete_error)?;

    Ok(Response::Ok(CardsDeleteSuccess { card_id }))
}

fn map_card_delete_error(error: app::CardDeleteError) -> Error {
    use app::CardDeleteError::*;

    match error {
        Unexpected(e) => Error::InternalServerError(e),
        NoAccess => CardsDeleteFailed {
            error: CardsDeleteError::NoAccess,
        }
        .into(),
        TokenExpired => CardsDeleteFailed {
            error: CardsDeleteError::NoAccess,
        }
        .into(),
        TokenNotFound => CardsDeleteFailed {
            error: CardsDeleteError::NoAccess,
        }
        .into(),
        CardNotFound => CardsDeleteFailed {
            error: CardsDeleteError::CardNotFound,
        }
        .into(),
    }
}
