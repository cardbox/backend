use crate::generated::{
    components::{
        request_bodies::CardsEditRequestBody,
        responses::{CardsEditError, CardsEditFailed, CardsEditSuccess},
    },
    paths::cards_edit::{Error, Response},
};
use actix_web::web::{self, Data};
use cardbox_core::app::extractors::SessionToken;
use cardbox_core::app::{CardUpdateError, CardUpdateForm, Cards};

pub async fn route(
    app: Data<cardbox_app::App>,
    body: web::Json<CardsEditRequestBody>,
    session_token: SessionToken,
) -> Result<Response, Error> {
    let body = body.into_inner();

    let updated = app
        .card_update(
            CardUpdateForm {
                id: body.card_id,
                title: body.title,
                contents: body.content,
                tags: body.tags,
            },
            session_token.into_inner(),
        )
        .await
        .map_err(map_card_update_error)?;

    Ok(Response::Ok(CardsEditSuccess {
        card: updated.into(),
    }))
}

#[inline]
fn map_card_update_error(error: CardUpdateError) -> Error {
    use CardUpdateError::*;

    match error {
        Unexpected(e) => Error::InternalServerError(e),
        TokenNotFound => CardsEditFailed {
            error: CardsEditError::NoAccess,
        }
        .into(),
        CardNotFound => CardsEditFailed {
            error: CardsEditError::CardNotFound,
        }
        .into(),
    }
}
