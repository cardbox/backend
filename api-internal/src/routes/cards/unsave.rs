use crate::generated::{
    components::{
        request_bodies::CardsUnsaveRequestBody,
        responses::{
            CardsUnsaveError as FailureVariant, CardsUnsaveFailed as Failure, CardsUnsaveSuccess,
        },
    },
    paths::cards_unsave::{Error, Response},
};
use actix_web::web::{Data, Json};
use cardbox_core::app::extractors::SessionToken;
use cardbox_core::app::CardUnsaveError;
use cardbox_core::app::Cards;

#[tracing::instrument]
pub async fn route(
    app: Data<cardbox_app::App>,
    body: Json<CardsUnsaveRequestBody>,
    token: SessionToken,
) -> Result<Response, Error> {
    let body = body.into_inner();

    let (unsaved, box_id) = app
        .card_remove_from_box(body.card_id, None, token.into_inner())
        .await
        .map_err(map_card_unsave_error)?;

    if unsaved.id != body.card_id {
        let span = tracing::span::Span::current();
        span.record("card_id", &tracing::field::display(body.card_id));
        span.record("unsaved_id", &tracing::field::display(unsaved.id));

        Err(Error::InternalServerError(eyre::eyre!(
            "Unsaved card has different id!"
        )))
    } else {
        Ok(Response::Ok(CardsUnsaveSuccess {
            card: unsaved.into(),
            box_id,
        }))
    }
}

fn map_card_unsave_error(error: CardUnsaveError) -> Error {
    use CardUnsaveError::*;

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
        AlreadyUnsaved => Failure {
            error: FailureVariant::AlreadyUnsaved,
        }
        .into(),
        BoxNotFound => unimplemented!("User cannot request custom box for now"),
    }
}
