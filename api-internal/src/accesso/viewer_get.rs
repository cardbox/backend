pub mod response {
    use serde::Deserialize;

    #[derive(Deserialize, Debug)]
    #[serde(untagged)]
    pub enum Answer {
        Authorized {
            #[serde(rename = "firstName")]
            first_name: String,

            #[serde(rename = "lastName")]
            last_name: String,

            #[serde(rename = "id")]
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
