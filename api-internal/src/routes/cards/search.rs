use crate::generated::{
    components::{request_bodies::CardsSearchRequestBody, responses::CardsSearchSuccess},
    paths::cards_search::{Error, Response},
};
use actix_web::web::{self, Data};
use cardbox_core::app::{CardSearchError, Cards};
use itertools::Itertools;

pub async fn route(
    app: Data<cardbox_app::App>,
    search: web::Json<CardsSearchRequestBody>,
) -> Result<Response, Error> {
    let body = search.into_inner();

    let search_results = app
        .cards_search(&body.query, body.limit)
        .await
        .map_err(map_card_search_error)?;

    let cards = search_results
        .iter()
        .cloned()
        .map(|(card, _)| card)
        .collect::<Vec<_>>();

    let users = search_results
        .into_iter()
        .map(|(_, user)| user)
        .unique_by(|u| u.id)
        .collect::<Vec<_>>();

    Ok(Response::Ok(CardsSearchSuccess {
        cards: cards.into_iter().map(Into::into).collect(),
        users: users.into_iter().map(Into::into).collect(),
    }))
}

fn map_card_search_error(error: CardSearchError) -> Error {
    use CardSearchError::*;

    match error {
        Unexpected(e) => Error::InternalServerError(e),
    }
}
