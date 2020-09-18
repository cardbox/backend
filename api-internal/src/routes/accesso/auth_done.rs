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
    let payload = accesso::ExchangeTokenPayload {
        grant_type: accesso::GrantType::AuthorizationCode,
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

    let response = result.unwrap().json::<accesso::OAuthResponse>().await;

    println!("DONE — {:#?}", response);

    match response {
        accesso::OAuthResponse(accesso::OAuthAccessTokenCreated { expires_in, .. }) => {
            let naive = chrono::NaiveDateTime::from_timestamp(expires_in, 0);
        }
        _ => {}
    }

    unimplemented!()
}

mod accesso {
    use serde::{Deserialize, Serialize};

    #[derive(Serialize, Debug)]
    pub enum GrantType {
        #[serde(rename = "authorization_code")]
        AuthorizationCode,
    }

    #[derive(Serialize, Debug)]
    pub struct ExchangeTokenPayload {
        pub grant_type: GrantType,
        pub code: String,
        pub redirect_uri: String,
        pub client_id: String,
        pub client_secret: String,
    }

    #[derive(Deserialize, Debug)]
    #[serde(untagged)]
    pub enum OAuthResponse {
        Success(OAuthAccessTokenCreated),
        Failure(OAuthAccessTokenFailure),
    }

    #[derive(Debug, Deserialize)]
    pub struct OAuthAccessTokenCreated {
        pub access_token: String,
        pub token_type: OAuthAccessTokenCreatedTokenType,

        /// UTC Unix TimeStamp when the access token expires
        pub expires_in: i64,
    }

    #[derive(Debug, Deserialize)]
    pub enum OAuthAccessTokenCreatedTokenType {
        #[serde(rename = "bearer")]
        Bearer,
    }

    #[derive(Debug, Deserialize)]
    pub struct OAuthAccessTokenFailure {
        pub error: OAuthAccessTokenFailureError,
    }

    #[derive(Debug, Deserialize)]
    pub enum OAuthAccessTokenFailureError {
        #[serde(rename = "invalid_request")]
        InvalidRequest,

        #[serde(rename = "invalid_client")]
        InvalidClient,

        #[serde(rename = "invalid_grant")]
        InvalidGrant,

        #[serde(rename = "invalid_scope")]
        InvalidScope,

        #[serde(rename = "unauthorized_client")]
        UnauthorizedClient,

        #[serde(rename = "unsupported_grant_type")]
        UnsupportedGrantType,
    }
}
