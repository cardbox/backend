use crate::generated::{
    components::{
        request_bodies::CardsListRequestBody,
        responses::{
            CardsListError as FailureVariant, CardsListFailed as Failure, CardsListSuccess,
        },
    },
    paths::cards_list::{Error, Response},
};
use actix_web::web::{Data, Json};
use cardbox_core::app::extractors::SessionToken;
use cardbox_core::app::Cards;
use cardbox_core::app::CardsListError;
use itertools::Itertools;

pub async fn route(
    app: Data<cardbox_app::App>,
    body: Json<CardsListRequestBody>,
    token: Option<SessionToken>,
) -> Result<Response, Error> {
    let body = body.into_inner();

    let list = app
        .cards_list(
            body.author_id,
            token.map(|token| token.into_inner()),
            body.favorites,
        )
        .await
        .map_err(map_cards_list_error)?;

    let cards = list
        .iter()
        .cloned()
        .map(|(card, _)| card)
        .collect::<Vec<_>>();
    let users = list
        .into_iter()
        .map(|(_, user)| user)
        .unique_by(|u| u.id)
        .collect::<Vec<_>>();

    Ok(Response::Ok(CardsListSuccess {
        cards: cards.into_iter().map(Into::into).collect(),
        users: users.into_iter().map(Into::into).collect(),
    }))
}

fn map_cards_list_error(error: CardsListError) -> Error {
    use CardsListError::*;

    match error {
        Unexpected(e) => Error::InternalServerError(e),
        Unauthorized => Failure {
            error: FailureVariant::Unathorized,
        }
        .into(),
    }
}
