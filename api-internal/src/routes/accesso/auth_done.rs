use crate::accesso::exchange_token::{self, ExchangeToken, GrantType};
use crate::server::{create_request_client, Config};
use actix_swagger::{Answer, ContentType};
use actix_web::http::StatusCode;
use actix_web::web::{Data, Json};
use serde::{Deserialize, Serialize};
use url::Url;

#[derive(Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct Body {
    authorization_code: String,
}

#[derive(Serialize)]
#[serde(untagged, rename_all = "camelCase")]
pub enum Response {
    Done { ok: bool },
}

pub async fn route(body: Json<Body>, config: Data<Config>) -> Answer<'static, Response> {
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

    let result = create_request_client(&config)
        .get(exchange_token_url)
        .method(awc::http::Method::POST)
        .send_json(&payload)
        .await;

    let response = result
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
                    // send GET /viewer request with X-Access-Token header
                    // create/update user with viewer.id: uuid::Uuid
                }
            }
        }
        Ok(Failure { error }) => match error {
            Error::InvalidRequest => {
                log::error!("Invalid request to accesso");
            }
            Error::InvalidClient => {
                log::error!(
                    "Invalid accesso client '{:#?}'",
                    config.accesso_client_id.clone()
                );
            }
            Error::InvalidGrant => {}
            Error::InvalidScope => {
                log::error!("Invalid scope for accesso");
            }
            Error::UnauthorizedClient => {
                log::error!(
                    "Unauthorized accesso client '{:#?}'",
                    config.accesso_client_id.clone()
                );
            }
            Error::UnsupportedGrantType => {
                log::error!("Unsupported grant type '{:#?}' for accesso", grant_type);
            }
            Error::UnknownAccessoError => {
                log::error!("Unknown error from accesso");
            }
        },
        Err(failure) => {
            log::error!("Failed to get response from accesso: {:#?}", failure);
        }
    }

    unimplemented!()
}
