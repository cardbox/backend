use crate::generated::{
    components::responses::{CardsFeedSuccess, CardsFeedSuccessLatest, CardsFeedSuccessTop},
    paths::cards_feed::{Error, Response},
};
use actix_web::web::Data;
use cardbox_core::app::{Cards, CardsFeedError};
use cardbox_core::models::{Card, User};
use itertools::Itertools;

pub async fn route(app: Data<cardbox_app::App>) -> Result<Response, Error> {
    let feed = app.cards_feed().await.map_err(map_cards_feed_error)?;

    Ok(Response::Ok(CardsFeedSuccess {
        top: feed.top.into(),
        latest: feed.latest.into(),
    }))
}

fn map_cards_feed_error(error: CardsFeedError) -> Error {
    use CardsFeedError::*;

    match error {
        Unexpected(e) => Error::InternalServerError(e),
    }
}

impl From<Vec<(Card, User)>> for CardsFeedSuccessLatest {
    fn from(v: Vec<(Card, User)>) -> Self {
        let cards = v.iter().cloned().map(|(card, _)| card).collect::<Vec<_>>();

        let users = v
            .into_iter()
            .map(|(_, user)| user)
            .unique_by(|u| u.id)
            .collect::<Vec<_>>();

        Self {
            cards: cards.into_iter().map(Into::into).collect(),
            users: users.into_iter().map(Into::into).collect(),
        }
    }
}

impl From<Vec<(Card, User)>> for CardsFeedSuccessTop {
    fn from(v: Vec<(Card, User)>) -> Self {
        let cards = v.iter().cloned().map(|(card, _)| card).collect::<Vec<_>>();

        let users = v
            .into_iter()
            .map(|(_, user)| user)
            .unique_by(|u| u.id)
            .collect::<Vec<_>>();

        Self {
            cards: cards.into_iter().map(Into::into).collect(),
            users: users.into_iter().map(Into::into).collect(),
        }
    }
}
