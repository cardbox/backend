pub mod response {
    use serde::Deserialize;

    #[derive(Deserialize, Debug)]
    #[serde(untagged)]
    pub enum Answer {
        #[serde(rename_all = "camelCase")]
        Authorized {
            first_name: String,
            last_name: String,
            id: uuid::Uuid,
        },

        Failure {
            error: Error,
        },
    }

    #[derive(Debug, Deserialize)]
    #[serde(rename_all = "snake_case")]
    pub enum Error {
        InvalidToken,
        Unauthorized,
    }
}
