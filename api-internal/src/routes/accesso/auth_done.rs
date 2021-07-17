use crate::accesso::exchange_token::{self, ExchangeToken, GrantType};
use actix_swagger::{Answer, ContentType};
use actix_web::http::StatusCode;
use actix_web::{
    cookie::CookieBuilder,
    web::{Data, Json},
};
use cardbox_app::SessionCookieConfig;
use cardbox_settings::Settings;
use reqwest::Client;
use serde::{Deserialize, Serialize};
use url::Url;

#[derive(Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct Body {
    authorization_code: String,
}

#[derive(Serialize)]
#[serde(untagged)]
pub enum Response {
    #[serde(rename_all = "camelCase")]
    Done { user_info: UserInfo },
    #[serde(rename_all = "camelCase")]
    Fail { error: PublicError },
}

#[derive(Serialize)]
#[serde(rename_all = "camelCase")]
pub struct UserInfo {
    first_name: String,
    last_name: String,
}

#[derive(Serialize)]
#[serde(rename_all = "snake_case")]
pub enum PublicError {
    /// Something goes wrong with accesso tokens
    AccessoFailed,

    /// Authorization code or password for grant type is invalid or expired
    TryLater,

    /// User declined authorization or client is unauthorized
    Unauthorized,

    // Something unexpected happened
    Unexpected,
}

pub async fn route(
    body: Json<Body>,
    config: Data<Settings>,
    config_session: Data<SessionCookieConfig>,
    client: Data<Client>,
    app: Data<cardbox_app::App>,
) -> Answer<'static, Response> {
    let grant_type = GrantType::AuthorizationCode;

    let payload = ExchangeToken {
        grant_type: grant_type.clone(),
        redirect_uri: config.accesso.redirect_back_url.clone(),
        code: body.authorization_code.clone(),
        client_id: config.accesso.client_id.clone(),
        client_secret: config.accesso.client_secret.clone(),
    };

    let exchange_token_url = {
        let mut uri = Url::parse(&config.accesso.url).expect("Failed to parse accesso_url");
        uri.set_path("/api/v0/oauth/token");
        uri.to_string()
    };

    tracing::debug!(%exchange_token_url, ?payload, "Sending request");

    let response = client
        .post(exchange_token_url)
        .json(&payload)
        .send()
        .await
        .expect("sent request")
        .json::<exchange_token::response::Answer>()
        .await;

    println!("DONE — {:#?}", response);

    use exchange_token::response::{
        Answer::{Failure, TokenCreated},
        Error, TokenType,
    };

    // https://www.oauth.com/oauth2-servers/access-tokens/access-token-response/
    match response {
        Ok(TokenCreated {
            expires_in,
            access_token,
            token_type,
        }) => {
            use chrono::{DateTime, NaiveDateTime, Utc};
            let naive = NaiveDateTime::from_timestamp(expires_in, 0);
            let datetime = DateTime::<Utc>::from_utc(naive, Utc);

            match token_type {
                TokenType::Bearer => {
                    println!("{}", datetime);

                    use crate::accesso::viewer_get::response::{
                        Answer::{self, Authorized, Failure},
                        Error,
                    };

                    let viewer_get_url = {
                        let mut uri =
                            Url::parse(&config.accesso.url).expect("Failed to parse accesso_url");
                        uri.set_path("/api/v0/viewer");
                        uri.to_string()
                    };

                    let result = client
                        .get(viewer_get_url)
                        .header("X-Access-Token", access_token)
                        .send()
                        .await
                        .expect("sent request")
                        .json::<Answer>()
                        .await;

                    match result {
                        Ok(Authorized {
                            first_name,
                            last_name,
                            id,
                        }) => {
                            use cardbox_core::app::{AccessoAuthorize, UpdateUserFailure};

                            let created = app
                                .authorize(cardbox_core::app::UserInfo {
                                    accesso_id: id,
                                    first_name,
                                    last_name,
                                })
                                .await;

                            match created {
                                Ok((user, session_token)) => Response::Done {
                                    user_info: UserInfo {
                                        first_name: user.first_name,
                                        last_name: user.last_name,
                                    },
                                }
                                .answer()
                                .cookie(
                                    CookieBuilder::new(
                                        config_session.name.clone(),
                                        session_token.token,
                                    )
                                    // TODO: extract to function or Trait
                                    .expires(time::OffsetDateTime::from_unix_timestamp(
                                        session_token.expires_at.timestamp(),
                                    ))
                                    .path(config_session.path.clone())
                                    .secure(config_session.secure)
                                    .http_only(config_session.http_only)
                                    .finish(),
                                ),
                                Err(UpdateUserFailure::Unexpected(_)) => {
                                    tracing::error!(
                                        "Failed to update user due to database unexpected error"
                                    );
                                    PublicError::Unexpected.answer()
                                }
                            }
                        }

                        Ok(Failure {
                            error: Error::InvalidToken,
                        }) => {
                            tracing::info!(
                                "Request for user data failed because access token is invalid"
                            );
                            PublicError::AccessoFailed.answer()
                        }

                        Ok(Failure {
                            error: Error::Unauthorized,
                        }) => {
                            tracing::info!(
                                "Unauthorized request to get user data with access token"
                            );
                            PublicError::Unauthorized.answer()
                        }

                        Err(error) => {
                            tracing::error!(
                                "Failed to parse json answer for accesso::viewer_get {:?}",
                                error
                            );
                            PublicError::Unexpected.answer()
                        }
                    }
                }
            }
        }
        Ok(Failure { error }) => match error {
            Error::InvalidRequest => {
                tracing::error!("Invalid request to accesso");
                PublicError::AccessoFailed.answer()
            }
            Error::InvalidClient => {
                tracing::error!(
                    "Invalid accesso client '{:#?}'",
                    config.accesso.client_id.clone()
                );
                PublicError::AccessoFailed.answer()
            }
            Error::InvalidGrant => {
                // The authorization code (or user’s password for the password grant type) is invalid or expired.
                // This is also the error you would return if the redirect URL given
                // in the authorization grant does not match the URL provided in this access token request.
                PublicError::TryLater.answer()
            }
            Error::InvalidScope => {
                tracing::error!("Invalid scope for accesso");
                PublicError::AccessoFailed.answer()
            }
            Error::UnauthorizedClient => {
                tracing::error!(
                    "Unauthorized accesso client '{:#?}'",
                    config.accesso.client_id.clone()
                );
                PublicError::Unauthorized.answer()
            }
            Error::UnsupportedGrantType => {
                tracing::error!("Unsupported grant type '{:#?}' for accesso", grant_type);
                PublicError::AccessoFailed.answer()
            }
            Error::UnknownAccessoError => {
                tracing::error!("Unknown error from accesso");
                PublicError::AccessoFailed.answer()
            }
        },
        Err(failure) => {
            tracing::error!("Failed to get response from accesso: {:#?}", failure);
            PublicError::Unexpected.answer()
        }
    }
}

impl Response {
    fn answer(self) -> Answer<'static, Self> {
        let status = match &self {
            Response::Done { .. } => StatusCode::OK,
            Response::Fail { error } => match error {
                PublicError::AccessoFailed => StatusCode::BAD_REQUEST,
                PublicError::TryLater => StatusCode::BAD_REQUEST,
                PublicError::Unauthorized => StatusCode::UNAUTHORIZED,
                PublicError::Unexpected => StatusCode::INTERNAL_SERVER_ERROR,
            },
        };

        Answer::new(self)
            .content_type(Some(ContentType::Json))
            .status(status)
    }
}

impl PublicError {
    fn answer(self) -> Answer<'static, Response> {
        Response::Fail { error: self }.answer()
    }
}
