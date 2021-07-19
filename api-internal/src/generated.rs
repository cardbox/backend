#![allow(dead_code)]
#![allow(clippy::from_over_into)]

pub mod api {
    use actix_swagger::{Api, Method};
    use actix_web::{
        dev::{AppService, Handler, HttpServiceFactory},
        FromRequest, Responder,
    };
    use std::future::Future;

    pub struct CardboxAPIInternal {
        api: Api,
    }

    pub fn create() -> CardboxAPIInternal {
        CardboxAPIInternal { api: Api::new() }
    }

    impl HttpServiceFactory for CardboxAPIInternal {
        fn register(self, config: &mut AppService) {
            self.api.register(config)
        }
    }

    impl CardboxAPIInternal {
        pub fn bind_auth_url<F, T, R>(mut self, handler: F) -> Self
        where
            F: Handler<T, R>,
            T: FromRequest + 'static,
            R: Future<
                    Output = Result<
                        super::paths::auth_url::Response,
                        super::paths::auth_url::Error,
                    >,
                > + 'static,
        {
            self.api = self.api.bind("/auth.url".into(), Method::POST, handler);
            self
        }
    }
}

pub mod components {
    pub mod responses {
        use serde::Serialize;

        #[derive(Debug, Serialize)]
        #[serde(rename_all = "camelCase")]
        pub struct AuthUrlSuccess {
            /// Accesso URL
            pub accesso_url: String,
        }
    }

    pub mod request_bodies {
        use serde::Deserialize;

        #[derive(Debug, Deserialize)]
        #[serde(rename_all = "camelCase")]
        pub struct AuthUrlRequestBody {
            /// oauth state
            pub state: String,
        }
    }
}
pub mod paths {
    use super::components::responses;

    pub mod auth_url {
        use super::responses;
        use actix_web::http::StatusCode;
        use actix_web::{HttpRequest, HttpResponse, Responder, ResponseError};
        use serde::Serialize;

        #[derive(Debug, Serialize, thiserror::Error)]
        #[serde(untagged)]
        pub enum Error {
            #[error(transparent)]
            InternalServerError(
                #[from]
                #[serde(skip)]
                eyre::Report,
            ),
        }

        #[derive(Debug, Serialize)]
        #[serde(untagged)]
        pub enum Response {
            Ok(responses::AuthUrlSuccess),
        }

        impl Responder for Response {
            fn respond_to(self, _: &HttpRequest) -> HttpResponse {
                match self {
                    Response::Ok(r) => HttpResponse::build(StatusCode::OK).json(r),
                }
            }
        }

        impl ResponseError for Error {
            fn status_code(&self) -> StatusCode {
                match self {
                    Error::InternalServerError(_) => StatusCode::INTERNAL_SERVER_ERROR,
                }
            }

            fn error_response(&self) -> HttpResponse {
                HttpResponse::build(self.status_code()).finish()
            }
        }
    }
}
