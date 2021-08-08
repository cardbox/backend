use crate::generated::{
    components::{
        request_bodies::CardsSearchRequestBody, responses::CardsSearchSuccess, schemas::Card,
        schemas::User,
    },
    paths::cards_search::{Error, Response},
};
use actix_web::web::{self, Data};
use cardbox_core::app::{CardSearchError, Cards};

pub async fn route(
    app: Data<cardbox_app::App>,
    search: web::Json<CardsSearchRequestBody>,
) -> Result<Response, Error> {
    let body = search.into_inner();

    let search_results = app
        .cards_search(&body.query, body.limit)
        .await
        .map_err(map_card_search_error)?;

    let total = search_results.len();

    Ok(Response::Ok(CardsSearchSuccess {
        cards: search_results
            .iter()
            .cloned()
            .map(|(c, _)| Card {
                id: c.id,
                title: c.title,
                content: c.contents,
                created_at: c.created_at,
                updated_at: c.updated_at,
                author_id: c.author_id,
                tags: c.tags,
            })
            .collect(),
        users: search_results
            .into_iter()
            .map(|(_, u)| User {
                id: u.id,
                first_name: u.first_name,
                last_name: u.last_name,
                username: u.username,
                avatar: u.avatar,
                work: u.work,
                bio: u.bio,
                socials: u
                    .socials
                    .unwrap_or_else(Vec::new)
                    .into_iter()
                    .map(Into::into)
                    .collect(),
            })
            .collect(),
        total,
    }))
}

fn map_card_search_error(error: CardSearchError) -> Error {
    use CardSearchError::*;

    match error {
        Unexpected(e) => Error::InternalServerError(e),
    }
}
