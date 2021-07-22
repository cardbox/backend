use serde::Serialize;

#[derive(Serialize, Debug, Clone)]
pub enum GrantType {
    #[serde(rename = "authorization_code")]
    AuthorizationCode,
}

#[derive(Serialize, Debug)]
pub struct ExchangeToken {
    pub grant_type: GrantType,
    pub code: String,
    pub redirect_uri: String,
    pub client_id: String,
    pub client_secret: String,
}

pub mod response {
    use serde::Deserialize;

    #[derive(Deserialize, Debug)]
    #[serde(untagged)]
    pub enum Answer {
        TokenCreated {
            access_token: String,
            token_type: TokenType,

            /// UTC Unix TimeStamp when the access token expires
            expires_in: i64,
        },

        Failure {
            error: Error,
        },
    }

    #[derive(Debug, Deserialize)]
    pub enum TokenType {
        #[serde(rename = "bearer")]
        Bearer,
    }

    #[derive(Debug, Deserialize)]
    #[serde(rename_all = "snake_case")]
    #[allow(clippy::enum_variant_names)]
    pub enum Error {
        InvalidClient,
        InvalidGrant,
        InvalidRequest,
        InvalidScope,
        UnauthorizedClient,
        UnsupportedGrantType,

        #[serde(other)]
        UnknownAccessoError,
    }
}
