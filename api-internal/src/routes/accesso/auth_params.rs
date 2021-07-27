use crate::generated::{
    components::{request_bodies::AuthUrlRequestBody, responses::AuthUrlSuccess},
    paths::auth_params::{Error, Response},
};
use actix_web::web::{Data, Json};
use cardbox_settings::Settings;
use url::Url;

pub async fn route(
    body: Json<AuthUrlRequestBody>,
    config: Data<Settings>,
    accesso_url: Data<Url>,
) -> Result<Response, Error> {
    let mut accesso = Url::clone(&accesso_url);

    accesso.set_path("/oauth/authorize");

    {
        let mut pairs = accesso.query_pairs_mut();
        pairs
            .append_pair("response_type", "code")
            .append_pair("redirect_uri", &config.accesso.redirect_back_url)
            .append_pair("client_id", &config.accesso.client_id)
            .append_pair("state", &body.state);
    }

    let response = Response::Ok(AuthUrlSuccess {
        accesso_url: accesso.to_string(),
    });

    Ok(response)
}
