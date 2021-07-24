use crate::generated::components::responses::{CardsCreateError, CardsCreateFailed};
use crate::generated::{
    components::{
        request_bodies::CardsCreateRequestBody, responses::CardsCreateSuccess, schemas::Card,
    },
    paths::cards_create::{Error, Response},
};
use actix_web::web::{self, Data};
use cardbox_core::app::extractors::SessionToken;
use cardbox_core::app::{CardCreateError, CardCreateForm, Cards};

pub async fn route(
    app: Data<cardbox_app::App>,
    card: web::Json<CardsCreateRequestBody>,
    session_token: SessionToken,
) -> Result<Response, Error> {
    let body = card.into_inner();

    let created_card = app
        .card_create(
            CardCreateForm {
                title: body.title,
                tags: body.tags,
                contents: &body.content,
            },
            session_token.into_inner(),
        )
        .await
        .map_err(map_new_card_error)?;

    Ok(Response::Ok(CardsCreateSuccess {
        card: created_card.into(),
    }))
}

impl From<cardbox_core::models::Card> for Card {
    fn from(card: cardbox_core::models::Card) -> Self {
        Self {
            id: card.id,
            title: card.title,
            created_at: card.created_at,
            updated_at: card.updated_at,
            content: card.contents,
            tags: card.tags,
            author_id: card.author_id,
        }
    }
}

fn map_new_card_error(error: CardCreateError) -> Error {
    use CardCreateError::*;

    match error {
        Unauthorized => Error::Unauthorized,
        ValidationError(_) => CardsCreateFailed {
            error: CardsCreateError::EmptyTitle,
        }
        .into(),
        Unexpected(e) => Error::InternalServerError(e),
    }
}
