use crate::accesso::exchange_token::{self, response::Error, ExchangeToken, GrantType};
use crate::generated::components::schemas::UserInfo;
use crate::generated::{
    components::{
        request_bodies::AuthDoneRequestBody,
        responses::{AuthDoneError, AuthDoneSuccess},
    },
    paths::auth_done::{Error as AuthDoneFailure, Response},
};
use actix_web::http::header;
use actix_web::{
    web::{Data, Json},
    HttpRequest, Responder,
};
use cardbox_app::SessionCookieConfig;
use cardbox_core::app::UpdateUserFailure;
use cardbox_settings::Settings;
use eyre::WrapErr;
use reqwest::Client;
use tracing::Span;
use url::Url;

pub async fn route(
    body: Json<AuthDoneRequestBody>,
    config: Data<Settings>,
    config_session: Data<SessionCookieConfig>,
    client: Data<Client>,
    app: Data<cardbox_app::App>,
    accesso_url: Data<Url>,
    req: HttpRequest,
) -> Result<impl Responder, AuthDoneFailure> {
    let grant_type = GrantType::AuthorizationCode;

    let payload = ExchangeToken {
        grant_type: grant_type.clone(),
        redirect_uri: config.accesso.redirect_back_url.clone(),
        code: body.authorization_code.clone(),
        client_id: config.accesso.client_id.clone(),
        client_secret: config.accesso.client_secret.clone(),
    };

    let exchange_token_url = {
        let mut uri = Url::clone(&accesso_url);
        let clone = uri.clone();
        let host = clone.host_str();
        uri.set_host(host.map(|host| format!("api.{}", host)).as_deref())
            .wrap_err("Could not set host")?;
        uri.set_path("/v0/oauth/token");
        uri.to_string()
    };

    tracing::debug!(%exchange_token_url, ?payload, "Sending request");

    let response = client
        .post(exchange_token_url)
        .json(&payload)
        .send()
        .await
        .wrap_err("Could not send exchange token request")?
        .json::<exchange_token::response::Answer>()
        .await
        .wrap_err("Could not deserialize into Answer")?;

    tracing::debug!(?response, "DONE");

    use exchange_token::response::{
        Answer::{Failure, TokenCreated},
        TokenType,
    };

    match response {
        TokenCreated {
            expires_in,
            access_token,
            token_type,
        } => {
            use chrono::{DateTime, NaiveDateTime, Utc};
            let naive = NaiveDateTime::from_timestamp(expires_in, 0);
            let datetime = DateTime::<Utc>::from_utc(naive, Utc);

            match token_type {
                TokenType::Bearer => {
                    Span::current().record("datetime", &tracing::field::display(datetime));

                    use crate::accesso::viewer_get::response::{
                        Answer::{self, Authorized, Failure},
                        Error,
                    };

                    let viewer_get_url = {
                        let mut uri = Url::clone(&accesso_url);
                        let clone = uri.clone();
                        let host = clone.host_str();
                        uri.set_host(host.map(|host| format!("api.{}", host)).as_deref())
                            .wrap_err("Could not set host")?;
                        uri.set_path("/v0/viewer.get");
                        uri.to_string()
                    };

                    let result = client
                        .post(viewer_get_url)
                        .header(header::AUTHORIZATION, access_token)
                        .send()
                        .await
                        .wrap_err("Could not send viewer request")?
                        .json::<Answer>()
                        .await
                        .wrap_err("Could not deserialize into viewer get Answer")?;

                    match result {
                        Authorized {
                            first_name,
                            last_name,
                            id,
                        } => {
                            use cardbox_core::app::AccessoAuthorize;

                            let (user, session_token) = app
                                .authorize(cardbox_core::app::UserInfo {
                                    accesso_id: id,
                                    first_name,
                                    last_name,
                                })
                                .await
                                .map_err(map_authorize_error)?;

                            let mut response = Response::Ok(AuthDoneSuccess {
                                user_info: UserInfo {
                                    first_name: user.first_name,
                                    last_name: user.last_name,
                                },
                            })
                            .respond_to(&req);

                            response
                                .add_cookie(&config_session.to_cookie(session_token))
                                .wrap_err("Could not add cookie")?;

                            Ok(response)
                        }
                        Failure {
                            error: Error::InvalidToken,
                        } => {
                            tracing::info!(
                                "Request for user data failed because access token is invalid"
                            );
                            Err(AuthDoneFailure::BadRequest(
                                AuthDoneError::AccessoFailed.into(),
                            ))
                        }
                        Failure {
                            error: Error::Unauthorized,
                        } => {
                            tracing::info!(
                                "Unauthorized request to get user data with access token"
                            );
                            Err(AuthDoneFailure::Unauthorized)
                        }
                    }
                }
            }
        }
        Failure { error } => Err(map_exchange_token_error(error, &config, grant_type)),
    }
}

fn map_exchange_token_error(
    error: Error,
    config: &Settings,
    grant_type: GrantType,
) -> AuthDoneFailure {
    match error {
        Error::InvalidRequest => {
            tracing::error!("Invalid request to accesso");
            AuthDoneFailure::BadRequest(AuthDoneError::AccessoFailed.into())
        }
        Error::InvalidClient => {
            tracing::error!(
                "Invalid accesso client '{:#?}'",
                config.accesso.client_id.clone()
            );
            AuthDoneFailure::BadRequest(AuthDoneError::AccessoFailed.into())
        }
        Error::InvalidGrant => {
            // The authorization code (or user’s password for the password grant type) is invalid or expired.
            // This is also the error you would return if the redirect URL given
            // in the authorization grant does not match the URL provided in this access token request.
            AuthDoneFailure::BadRequest(AuthDoneError::TryLater.into())
        }
        Error::InvalidScope => {
            tracing::error!("Invalid scope for accesso");
            AuthDoneFailure::BadRequest(AuthDoneError::AccessoFailed.into())
        }
        Error::UnauthorizedClient => {
            tracing::error!(
                "Unauthorized accesso client '{:#?}'",
                config.accesso.client_id.clone()
            );
            AuthDoneFailure::Unauthorized
        }
        Error::UnsupportedGrantType => {
            tracing::error!("Unsupported grant type '{:#?}' for accesso", grant_type);
            AuthDoneFailure::BadRequest(AuthDoneError::AccessoFailed.into())
        }
        Error::UnknownAccessoError => {
            tracing::error!("Unknown error from accesso");
            AuthDoneFailure::BadRequest(AuthDoneError::AccessoFailed.into())
        }
    }
}

fn map_authorize_error(err: UpdateUserFailure) -> AuthDoneFailure {
    match err {
        UpdateUserFailure::Unexpected(e) => AuthDoneFailure::Unexpected(e),
    }
}
