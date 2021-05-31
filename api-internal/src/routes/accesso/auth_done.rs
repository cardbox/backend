use crate::accesso::exchange_token::{self, ExchangeToken, GrantType};
use crate::server::{create_request_client, Config, ConfigSession};
use actix_swagger::{Answer, ContentType};
use actix_web::http::StatusCode;
use actix_web::{
    cookie::CookieBuilder,
    web::{Data, Json},
};
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
    config: Data<Config>,
    config_session: Data<ConfigSession>,
    app: Data<crate::App>,
) -> Answer<'static, Response> {
    let grant_type = GrantType::AuthorizationCode;

    let payload = ExchangeToken {
        grant_type: grant_type.clone(),
        redirect_uri: config.accesso_redirect_back_url.clone(),
        code: body.authorization_code.clone(),
        client_id: config.accesso_client_id.clone(),
        client_secret: config.accesso_client_secret.clone(),
    };

    let exchange_token_url = {
        let mut uri = Url::parse(&config.accesso_url).expect("Failed to parse accesso_url");
        uri.set_path("/api/v0/oauth/token");
        uri.to_string()
    };

    println!("Send request to {} -> {:#?}", exchange_token_url, payload);
    println!(
        "JSON:::: {}",
        serde_json::to_string_pretty(&payload).unwrap()
    );

    let client = create_request_client(&config);

    let response = client
        .post(exchange_token_url)
        .send_json(&payload)
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
                            Url::parse(&config.accesso_url).expect("Failed to parse accesso_url");
                        uri.set_path("/api/v0/viewer");
                        uri.to_string()
                    };

                    let result = client
                        .get(viewer_get_url)
                        .insert_header(("X-Access-Token", access_token))
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
                                .lock()
                                .await
                                .authorize(cardbox_core::app::UserInfo {
                                    accesso_id: id,
                                    first_name,
                                    last_name,
                                })
                                .await;

                            match created {
                                Ok((user, session_token)) => Response::Done {
                                    user_info: UserInfo {
                                        first_name: user.first_name(),
                                        last_name: user.last_name(),
                                    },
                                }
                                .answer()
                                .cookie(
                                    CookieBuilder::new(
                                        config_session.name.clone(),
                                        session_token.token(),
                                    )
                                    // TODO: extract to function or Trait
                                    .expires(time::OffsetDateTime::from_unix_timestamp(
                                        chrono::DateTime::<chrono::Utc>::from_utc(
                                            session_token.expires_at(),
                                            chrono::Utc,
                                        )
                                        .timestamp(),
                                    ))
                                    .path(config_session.path.clone())
                                    .secure(config_session.secure)
                                    .http_only(config_session.http_only)
                                    .finish(),
                                ),
                                Err(UpdateUserFailure::Unexpected) => {
                                    log::error!(
                                        "Failed to update user due to database unexpected error"
                                    );
                                    PublicError::Unexpected.answer()
                                }
                            }
                        }

                        Ok(Failure {
                            error: Error::InvalidToken,
                        }) => {
                            log::info!(
                                "Request for user data failed because access token is invalid"
                            );
                            PublicError::AccessoFailed.answer()
                        }

                        Ok(Failure {
                            error: Error::Unauthorized,
                        }) => {
                            log::info!("Unauthorized request to get user data with access token");
                            PublicError::Unauthorized.answer()
                        }

                        Err(error) => {
                            log::error!(
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
                log::error!("Invalid request to accesso");
                PublicError::AccessoFailed.answer()
            }
            Error::InvalidClient => {
                log::error!(
                    "Invalid accesso client '{:#?}'",
                    config.accesso_client_id.clone()
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
                log::error!("Invalid scope for accesso");
                PublicError::AccessoFailed.answer()
            }
            Error::UnauthorizedClient => {
                log::error!(
                    "Unauthorized accesso client '{:#?}'",
                    config.accesso_client_id.clone()
                );
                PublicError::Unauthorized.answer()
            }
            Error::UnsupportedGrantType => {
                log::error!("Unsupported grant type '{:#?}' for accesso", grant_type);
                PublicError::AccessoFailed.answer()
            }
            Error::UnknownAccessoError => {
                log::error!("Unknown error from accesso");
                PublicError::AccessoFailed.answer()
            }
        },
        Err(failure) => {
            log::error!("Failed to get response from accesso: {:#?}", failure);
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
