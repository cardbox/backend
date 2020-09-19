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
    pub enum Error {
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

        #[serde(other)]
        UnknownAccessoError,
    }
}
