use crate::generated::{
    components::{
        request_bodies::CardsSearchRequestBody, responses::CardsSearchSuccess, schemas::Card,
        schemas::User,
    },
    paths::cards_search::{Error, Response},
};
use actix_web::web::{self, BufMut, Bytes, BytesMut, Data};
use actix_web::HttpResponse;
use actix_web::{Error as ActixError, ResponseError};
use async_stream::stream;
use bytes::buf::Writer;
use cardbox_core::app::{CardSearchError, Cards};
use cardbox_core::models;
use futures::stream::BoxStream;
use futures::{ready, FutureExt, Stream, StreamExt, TryStreamExt};
use pin_project_lite::pin_project;
use serde::Serialize;
use std::pin::Pin;
use std::sync::Arc;
use std::task::{Context, Poll};

#[derive(Debug, serde::Serialize)]
struct CardUser {
    card: Card,
    user: User,
}

pin_project! {
    struct JsonStream<St> {
        #[pin]
        inner: St,
        buf: Writer<BytesMut>,
    }
}

impl<St, Ser> futures::Stream for JsonStream<St>
where
    Ser: Serialize,
    St: Stream<Item = Result<Ser, ActixError>>,
{
    type Item = Result<Bytes, ActixError>;

    fn poll_next(self: Pin<&mut Self>, cx: &mut Context<'_>) -> Poll<Option<Self::Item>> {
        let mut this = self.project();

        for v in b"[" {
            this.buf.get_mut().put_u8(*v);
        }

        match this.inner.as_mut().poll_next(cx) {
            Poll::Pending => return Poll::Pending,
            Poll::Ready(Some(Ok(json))) => {
                let res = serde_json::to_writer(&mut this.buf, &json);

                if let Err(e) = res {
                    return Poll::Ready(Some(Err(e.into())));
                }

                for v in b"," {
                    this.buf.get_mut().put_u8(*v);
                }

                Poll::Ready(Some(Ok(Bytes::copy_from_slice(&this.buf.get_ref()))))
            }
            Poll::Ready(Some(Err(e))) => return Poll::Ready(Some(Err(e))),
            Poll::Ready(None) => {
                for v in b"]" {
                    this.buf.get_mut().put_u8(*v);
                }

                Poll::Ready(None)
            }
        }
    }
}

pub async fn route(
    app: Data<cardbox_app::App>,
    search: web::Json<CardsSearchRequestBody>,
) -> Result<HttpResponse, Error> {
    let body = search.into_inner();
    let app = app.clone();

    let mut buf = BytesMut::new().writer();

    let search_results = stream! {
        let mut began = false;
        let mut stream = app
            .cards_search(&body.query, body.limit)
            .unwrap()
            .map_err(map_card_search_error);

        for v in b"[" {
            buf.get_mut().put_u8(*v);
        };

        while let Some(res) = stream.next().await {
            buf.get_mut().clear();
            if !began {
                for v in b"[" {
                    buf.get_mut().put_u8(*v);
                };

                began = true;
            };

            match res {
                Ok((card, user)) => {
                    let card_user = CardUser {
                        card: Card {
                            id: card.id,
                            author_id: card.author_id,
                            title: card.title,
                            created_at: card.created_at,
                            updated_at: card.updated_at,
                            content: card.contents,
                            tags: card.tags
                        },
                        user: User {
                            id: user.id,
                            first_name: user.first_name,
                            last_name: user.last_name
                        }
                    };

                    serde_json::to_writer(&mut buf, &card_user).unwrap();

                    for v in b"," {
                        buf.get_mut().put_u8(*v);
                    };

                    yield Ok(Bytes::copy_from_slice(&buf.get_ref()))
                },
                Err(e) => yield Err(e)
            }
        }
    };

    // let search_results = app
    //     .cards_search(&body.query, body.limit)?
    //     .map_err(map_card_search_error)
    //     .map_err(ActixError::from)
    //     .map_ok(|(card, user)| CardUser {
    //         card: Card {
    //             id: card.id,
    //             title: card.title,
    //             content: card.contents,
    //             created_at: card.created_at,
    //             updated_at: card.updated_at,
    //             author_id: card.author_id,
    //             tags: card.tags,
    //         },
    //         user: User {
    //             id: user.id,
    //             first_name: user.first_name,
    //             last_name: user.last_name,
    //         },
    //     });

    Ok(HttpResponse::Ok().streaming(Box::pin(search_results)))
}

fn map_card_search_error(error: CardSearchError) -> Error {
    use CardSearchError::*;

    match error {
        Unexpected(e) => Error::InternalServerError(e),
    }
}
