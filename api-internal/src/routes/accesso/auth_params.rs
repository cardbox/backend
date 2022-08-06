use crate::generated::{
    components::{request_bodies::AuthUrlRequestBody, responses::AuthUrlSuccess},
    paths::auth_params::{Error, Response},
};
use crate::AccessoAuthorizeUrl;
use actix_web::web::{Data, Json};
use cardbox_settings::Settings;

pub async fn route(
    body: Json<AuthUrlRequestBody>,
    config: Data<Settings>,
    accesso_authorize_url: Data<AccessoAuthorizeUrl>,
) -> Result<Response, Error> {
    let mut accesso_authorize = AccessoAuthorizeUrl::clone(&accesso_authorize_url);

    {
        let mut pairs = accesso_authorize.query_pairs_mut();
        pairs
            .append_pair("response_type", "code")
            .append_pair("redirect_uri", &config.accesso.redirect_back_url)
            .append_pair("client_id", &config.accesso.client_id)
            .append_pair("state", &body.state);
    }

    let response = Response::Ok(AuthUrlSuccess {
        accesso_url: accesso_authorize.to_string(),
    });

    Ok(response)
}
