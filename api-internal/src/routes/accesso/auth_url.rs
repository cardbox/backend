use crate::server::Config;
use actix_swagger::{Answer, ContentType};
use actix_web::http::StatusCode;
use actix_web::web::{Data, Json};
use serde::{Deserialize, Serialize};
use url::Url;

#[derive(Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct Body {
    state: String,
}

#[derive(Serialize)]
#[serde(rename_all = "camelCase")]
pub struct Response {
    accesso_url: String,
}

pub async fn route(body: Json<Body>, config: Data<Config>) -> Answer<'static, Response> {
    let mut accesso = Url::parse(&config.accesso_url).expect("Failed to parse accesso_url");

    accesso.set_path("/oauth/authorize");

    {
        let mut pairs = accesso.query_pairs_mut();
        pairs
            .append_pair("response_type", "code")
            .append_pair("redirect_uri", &config.accesso_redirect_back_url)
            .append_pair("client_id", &config.accesso_client_id)
            .append_pair("state", &body.state);
    }

    Answer::new(Response {
        accesso_url: accesso.to_string(),
    })
    .content_type(Some(ContentType::Json))
    .status(StatusCode::OK)
}
