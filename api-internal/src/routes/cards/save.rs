use crate::generated::{
    components::{
        request_bodies::CardsSaveRequestBody,
        responses::{
            CardsSaveError as FailureVariant, CardsSaveFailed as Failure, CardsSaveSuccess,
        },
    },
    paths::cards_save::{Error, Response},
};
use actix_web::web::{Data, Json};
use cardbox_core::app::extractors::SessionToken;
use cardbox_core::app::CardSaveError;
use cardbox_core::app::Cards;

#[tracing::instrument]
pub async fn route(
    app: Data<cardbox_app::App>,
    body: Json<CardsSaveRequestBody>,
    token: SessionToken,
) -> Result<Response, Error> {
    let body = body.into_inner();

    let (saved, box_id) = app
        .card_add_to_box(body.card_id, None, token.into_inner())
        .await
        .map_err(map_card_save_error)?;

    if saved.id != body.card_id {
        let span = tracing::span::Span::current();
        span.record("card_id", &tracing::field::display(body.card_id));
        span.record("saved_id", &tracing::field::display(saved.id));

        Err(Error::InternalServerError(eyre::eyre!(
            "Saved card has different id!"
        )))
    } else {
        Ok(Response::Ok(CardsSaveSuccess {
            card: saved.into(),
            box_id,
        }))
    }
}

fn map_card_save_error(error: CardSaveError) -> Error {
    use CardSaveError::*;

    match error {
        Unexpected(e) => Error::InternalServerError(e),
        CardNotFound => Failure {
            error: FailureVariant::CardNotFound,
        }
        .into(),
        TokenExpired => Failure {
            error: FailureVariant::NoAccess,
        }
        .into(),
        TokenNotFound => Failure {
            error: FailureVariant::NoAccess,
        }
        .into(),
        NoAccess => Failure {
            error: FailureVariant::NoAccess,
        }
        .into(),
        AlreadySaved => Failure {
            error: FailureVariant::AlreadySaved,
        }
        .into(),
        BoxNotFound => unimplemented!("User cannot request custom box for now"),
    }
}
