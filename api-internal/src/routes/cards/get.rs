use crate::generated::{
    components::{
        request_bodies::CardsGetRequestBody,
        responses::{CardsGetError as FailureVariant, CardsGetFailed as Failure, CardsGetSuccess},
    },
    paths::cards_get::{Error, Response},
};
use actix_web::web::{Data, Json};
use cardbox_core::app::CardGetError;
use cardbox_core::app::Cards;

pub async fn route(
    app: Data<cardbox_app::App>,
    body: Json<CardsGetRequestBody>,
) -> Result<Response, Error> {
    let body = body.into_inner();

    let card = app
        .card_get(body.card_id)
        .await
        .map_err(map_cards_get_error)?;

    Ok(Response::Ok(CardsGetSuccess { card: card.into() }))
}

fn map_cards_get_error(error: CardGetError) -> Error {
    use CardGetError::*;

    match error {
        Unexpected(e) => Error::InternalServerError(e),
        CardNotFound => Failure {
            error: FailureVariant::CardNotFound,
        }
        .into(),
    }
}
