#![allow(dead_code)]
#![allow(clippy::from_over_into)]
#![allow(clippy::enum_variant_names)]

pub mod api {
    use actix_swagger::Api;
    use actix_web::{
        dev::{AppService, Handler, HttpServiceFactory},
        http::Method,
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
        pub fn bind_auth_params<F, T, R>(mut self, handler: F) -> Self
        where
            F: Handler<T, R>,
            T: FromRequest + 'static,
            R: Future<
                    Output = Result<
                        super::paths::auth_params::Response,
                        super::paths::auth_params::Error,
                    >,
                > + 'static,
        {
            self.api = self.api.bind("/accesso/auth.params", Method::POST, handler);
            self
        }

        pub fn bind_auth_done<F, T, R, Res>(mut self, handler: F) -> Self
        where
            F: Handler<T, R>,
            T: FromRequest + 'static,
            Res: Responder + 'static,
            R: Future<Output = Result<Res, super::paths::auth_done::Error>> + 'static,
        {
            self.api = self.api.bind("/accesso/auth.done", Method::POST, handler);
            self
        }

        pub fn bind_cards_create<F, T, R>(mut self, handler: F) -> Self
        where
            F: Handler<T, R>,
            T: FromRequest + 'static,
            R: Future<
                    Output = Result<
                        super::paths::cards_create::Response,
                        super::paths::cards_create::Error,
                    >,
                > + 'static,
        {
            self.api = self.api.bind("/cards.create", Method::POST, handler);
            self
        }

        pub fn bind_cards_search<F, T, R>(mut self, handler: F) -> Self
        where
            F: Handler<T, R>,
            T: FromRequest + 'static,
            R: Future<
                    Output = Result<
                        super::paths::cards_search::Response,
                        super::paths::cards_search::Error,
                    >,
                > + 'static,
        {
            self.api = self.api.bind("/cards.search", Method::POST, handler);
            self
        }

        pub fn bind_cards_edit<F, T, R>(mut self, handler: F) -> Self
        where
            F: Handler<T, R>,
            T: FromRequest + 'static,
            R: Future<
                    Output = Result<
                        super::paths::cards_edit::Response,
                        super::paths::cards_edit::Error,
                    >,
                > + 'static,
        {
            self.api = self.api.bind("/cards.edit", Method::POST, handler);
            self
        }

        pub fn bind_cards_delete<F, T, R>(mut self, handler: F) -> Self
        where
            F: Handler<T, R>,
            T: FromRequest + 'static,
            R: Future<
                    Output = Result<
                        super::paths::cards_delete::Response,
                        super::paths::cards_delete::Error,
                    >,
                > + 'static,
        {
            self.api = self.api.bind("/cards.delete", Method::POST, handler);
            self
        }

        pub fn bind_cards_save<F, T, R>(mut self, handler: F) -> Self
        where
            F: Handler<T, R>,
            T: FromRequest + 'static,
            R: Future<
                    Output = Result<
                        super::paths::cards_save::Response,
                        super::paths::cards_save::Error,
                    >,
                > + 'static,
        {
            self.api = self.api.bind("/cards.save", Method::POST, handler);
            self
        }

        pub fn bind_cards_unsave<F, T, R>(mut self, handler: F) -> Self
        where
            F: Handler<T, R>,
            T: FromRequest + 'static,
            R: Future<
                    Output = Result<
                        super::paths::cards_unsave::Response,
                        super::paths::cards_unsave::Error,
                    >,
                > + 'static,
        {
            self.api = self.api.bind("/cards.unsave", Method::POST, handler);
            self
        }

        pub fn bind_cards_list<F, T, R>(mut self, handler: F) -> Self
        where
            F: Handler<T, R>,
            T: FromRequest + 'static,
            R: Future<
                    Output = Result<
                        super::paths::cards_list::Response,
                        super::paths::cards_list::Error,
                    >,
                > + 'static,
        {
            self.api = self.api.bind("/cards.list", Method::POST, handler);
            self
        }

        pub fn bind_cards_get<F, T, R>(mut self, handler: F) -> Self
        where
            F: Handler<T, R>,
            T: FromRequest + 'static,
            R: Future<
                    Output = Result<
                        super::paths::cards_get::Response,
                        super::paths::cards_get::Error,
                    >,
                > + 'static,
        {
            self.api = self.api.bind("/cards.get", Method::POST, handler);
            self
        }

        pub fn bind_users_get<F, T, R>(mut self, handler: F) -> Self
        where
            F: Handler<T, R>,
            T: FromRequest + 'static,
            R: Future<
                    Output = Result<
                        super::paths::users_get::Response,
                        super::paths::users_get::Error,
                    >,
                > + 'static,
        {
            self.api = self.api.bind("/users.get", Method::POST, handler);
            self
        }

        pub fn bind_session_get<F, T, R>(mut self, handler: F) -> Self
        where
            F: Handler<T, R>,
            T: FromRequest + 'static,
            R: Future<
                    Output = Result<
                        super::paths::session_get::Response,
                        super::paths::session_get::Error,
                    >,
                > + 'static,
        {
            self.api = self.api.bind("/session.get", Method::POST, handler);
            self
        }

        pub fn bind_users_search<F, T, R>(mut self, handler: F) -> Self
        where
            F: Handler<T, R>,
            T: FromRequest + 'static,
            R: Future<
                    Output = Result<
                        super::paths::users_search::Response,
                        super::paths::users_search::Error,
                    >,
                > + 'static,
        {
            self.api = self.api.bind("/users.search", Method::POST, handler);
            self
        }

        pub fn bind_cards_feed<F, T, R>(mut self, handler: F) -> Self
        where
            F: Handler<T, R>,
            T: FromRequest + 'static,
            R: Future<
                    Output = Result<
                        super::paths::cards_feed::Response,
                        super::paths::cards_feed::Error,
                    >,
                > + 'static,
        {
            self.api = self.api.bind("/cards.feed", Method::POST, handler);
            self
        }
    }
}

pub mod components {
    pub mod responses {
        use super::schemas;
        use serde::Serialize;
        use uuid::Uuid;

        #[derive(Debug, Serialize)]
        #[serde(rename_all = "camelCase")]
        pub struct AuthUrlSuccess {
            /// Accesso URL
            pub accesso_url: String,
        }

        #[derive(Debug, Serialize)]
        #[serde(rename_all = "camelCase")]
        pub struct AuthDoneSuccess {
            pub user: schemas::UserInfo,
        }

        #[derive(Debug, Serialize, thiserror::Error)]
        #[serde(rename_all = "camelCase")]
        #[error(transparent)]
        pub struct AuthDoneFailed {
            #[from]
            error: AuthDoneError,
        }

        #[derive(Debug, Serialize, thiserror::Error)]
        #[serde(rename_all = "snake_case")]
        pub enum AuthDoneError {
            #[error("Accesso failed")]
            AccessoFailed,
            #[error("Try later")]
            TryLater,
        }

        #[derive(Debug, Serialize)]
        #[serde(rename_all = "camelCase")]
        pub struct CardsCreateSuccess {
            pub card: schemas::Card,
        }

        #[derive(Debug, Serialize, thiserror::Error)]
        #[error(transparent)]
        pub struct CardsCreateFailed {
            #[from]
            pub error: CardsCreateError,
        }

        #[derive(Debug, Serialize, thiserror::Error)]
        #[serde(rename_all = "snake_case")]
        pub enum CardsCreateError {
            #[error("Empty title")]
            EmptyTitle,
            #[error("Invalid content")]
            InvalidContent,
        }

        #[derive(Debug, Serialize)]
        #[serde(rename_all = "camelCase")]
        pub struct CardsSearchSuccess {
            pub cards: Vec<schemas::Card>,
            pub users: Vec<schemas::User>,
        }

        #[derive(Debug, Serialize)]
        #[serde(rename_all = "camelCase")]
        pub struct CardsEditSuccess {
            pub card: schemas::Card,
        }

        #[derive(Debug, Serialize, thiserror::Error)]
        #[serde(rename_all = "camelCase")]
        #[error(transparent)]
        pub struct CardsEditFailed {
            #[from]
            pub error: CardsEditError,
        }

        #[derive(Debug, Serialize, thiserror::Error)]
        #[serde(rename_all = "snake_case")]
        pub enum CardsEditError {
            #[error("Card not found")]
            CardNotFound,
            #[error("Invalid payload")]
            InvalidPayload(#[serde(skip)] eyre::Report),
            #[error("No access")]
            NoAccess,
        }

        #[derive(Debug, Serialize)]
        #[serde(rename_all = "camelCase")]
        pub struct CardsDeleteSuccess {
            pub card_id: Uuid,
        }

        #[derive(Debug, Serialize, thiserror::Error)]
        #[error(transparent)]
        pub struct CardsDeleteFailed {
            #[from]
            pub error: CardsDeleteError,
        }

        #[derive(Debug, Serialize, thiserror::Error)]
        #[serde(rename_all = "snake_case")]
        pub enum CardsDeleteError {
            #[error("Card not found")]
            CardNotFound,
            #[error("No access")]
            NoAccess,
        }

        #[derive(Debug, Serialize)]
        #[serde(rename_all = "camelCase")]
        pub struct CardsSaveSuccess {
            pub card: schemas::Card,
            pub box_id: Uuid,
        }

        #[derive(Debug, Serialize, thiserror::Error)]
        #[error(transparent)]
        pub struct CardsSaveFailed {
            #[from]
            pub error: CardsSaveError,
        }

        #[derive(Debug, Serialize, thiserror::Error)]
        #[serde(rename_all = "snake_case")]
        pub enum CardsSaveError {
            #[error("Already saved")]
            AlreadySaved,
            #[error("Card not found")]
            CardNotFound,
            #[error("No access")]
            NoAccess,
        }

        #[derive(Debug, Serialize)]
        #[serde(rename_all = "camelCase")]
        pub struct CardsUnsaveSuccess {
            pub card: schemas::Card,
            pub box_id: Uuid,
        }

        #[derive(Debug, Serialize, thiserror::Error)]
        #[error(transparent)]
        pub struct CardsUnsaveFailed {
            #[from]
            pub error: CardsUnsaveError,
        }

        #[derive(Debug, Serialize, thiserror::Error)]
        #[serde(rename_all = "snake_case")]
        pub enum CardsUnsaveError {
            #[error("Already unsaved")]
            AlreadyUnsaved,
            #[error("Card not found")]
            CardNotFound,
            #[error("No access")]
            NoAccess,
        }

        #[derive(Debug, Serialize)]
        #[serde(rename_all = "camelCase")]
        pub struct CardsListSuccess {
            pub cards: Vec<schemas::Card>,
            pub users: Vec<schemas::User>,
        }

        #[derive(Debug, Serialize, thiserror::Error)]
        #[error(transparent)]
        pub struct CardsListFailed {
            pub error: CardsListError,
        }

        #[derive(Debug, Serialize, thiserror::Error)]
        #[serde(rename_all = "snake_case")]
        pub enum CardsListError {
            #[error("Invalid params")]
            InvalidParams,
            #[error("Unauthorized")]
            Unathorized,
        }

        #[derive(Debug, Serialize)]
        #[serde(rename_all = "camelCase")]
        pub struct CardsGetSuccess {
            pub card: schemas::Card,
            pub user: schemas::User,
        }

        #[derive(Debug, Serialize, thiserror::Error)]
        #[error(transparent)]
        pub struct CardsGetFailed {
            pub error: CardsGetError,
        }

        #[derive(Debug, Serialize, thiserror::Error)]
        #[serde(rename_all = "snake_case")]
        pub enum CardsGetError {
            #[error("Card not found")]
            CardNotFound,
        }

        #[derive(Debug, Serialize)]
        #[serde(rename_all = "camelCase")]
        pub struct UsersGetSuccess {
            pub user: schemas::User,
        }

        #[derive(Debug, Serialize, thiserror::Error)]
        #[error(transparent)]
        pub struct UsersGetFailed {
            pub error: UsersGetError,
        }

        #[derive(Debug, Serialize, thiserror::Error)]
        #[serde(rename_all = "snake_case")]
        pub enum UsersGetError {
            #[error("User not found")]
            UserNotFound,
        }

        #[derive(Debug, Serialize)]
        #[serde(rename_all = "camelCase")]
        pub struct SessionGetSuccess {
            pub user: schemas::SessionUser,
        }

        #[derive(Debug, Serialize)]
        #[serde(rename_all = "camelCase")]
        pub struct UsersSearchSuccess {
            pub users: Vec<schemas::User>,
        }

        #[derive(Debug, Serialize)]
        #[serde(rename_all = "camelCase")]
        pub struct CardsFeedSuccessTop {
            pub cards: Vec<schemas::Card>,
            pub users: Vec<schemas::User>,
        }

        #[derive(Debug, Serialize)]
        #[serde(rename_all = "camelCase")]
        pub struct CardsFeedSuccessLatest {
            pub cards: Vec<schemas::Card>,
            pub users: Vec<schemas::User>,
        }

        #[derive(Debug, Serialize)]
        #[serde(rename_all = "camelCase")]
        pub struct CardsFeedSuccess {
            pub top: CardsFeedSuccessTop,
            pub latest: CardsFeedSuccessLatest,
        }
    }

    pub mod request_bodies {
        use serde::Deserialize;
        use uuid::Uuid;

        #[derive(Debug, Deserialize)]
        #[serde(rename_all = "camelCase")]
        pub struct AuthUrlRequestBody {
            /// oauth state
            pub state: String,
        }

        #[derive(Debug, Deserialize)]
        #[serde(rename_all = "camelCase")]
        pub struct AuthDoneRequestBody {
            /// Authorization code
            pub authorization_code: String,
        }

        #[derive(Debug, Deserialize)]
        #[serde(rename_all = "camelCase")]
        pub struct CardsCreateRequestBody {
            pub title: String,
            pub content: Box<serde_json::value::RawValue>,
            pub tags: Vec<String>,
        }

        #[derive(Debug, Deserialize)]
        #[serde(rename_all = "camelCase")]
        pub struct CardsSearchRequestBody {
            pub query: String,
            pub limit: Option<i64>,
        }

        #[derive(Debug, Deserialize)]
        #[serde(rename_all = "camelCase")]
        pub struct CardsEditRequestBody {
            pub card_id: Uuid,
            pub title: Option<String>,
            pub content: Option<Box<serde_json::value::RawValue>>,
            pub tags: Option<Vec<String>>,
        }

        #[derive(Debug, Deserialize)]
        #[serde(rename_all = "camelCase")]
        pub struct CardsDeleteRequestBody {
            pub card_id: Uuid,
        }

        #[derive(Debug, Deserialize)]
        #[serde(rename_all = "camelCase")]
        pub struct CardsSaveRequestBody {
            pub card_id: Uuid,
        }

        #[derive(Debug, Deserialize)]
        #[serde(rename_all = "camelCase")]
        pub struct CardsUnsaveRequestBody {
            pub card_id: Uuid,
        }

        fn __default_cards_list_request_body_favorites() -> bool {
            false
        }

        #[derive(Debug, Deserialize)]
        #[serde(rename_all = "camelCase")]
        pub struct CardsListRequestBody {
            /// Author id
            pub author_id: Option<Uuid>,
            #[serde(default = "__default_cards_list_request_body_favorites")]
            pub favorites: bool,
        }

        #[derive(Debug, Deserialize)]
        #[serde(rename_all = "camelCase")]
        pub struct CardsGetRequestBody {
            pub card_id: Uuid,
        }

        #[derive(Debug, Deserialize)]
        #[serde(rename_all = "camelCase")]
        pub struct UsersGetRequestBody {
            pub username: String,
        }

        #[derive(Debug, Deserialize)]
        #[serde(rename_all = "camelCase")]
        pub struct UsersSearchRequestBody {
            pub query: String,
        }
    }

    pub mod schemas {
        use chrono::{DateTime, Utc};
        use serde::Serialize;
        use uuid::Uuid;

        #[derive(Debug, Serialize)]
        #[serde(rename_all = "camelCase")]
        pub struct UserInfo {
            pub id: Uuid,
            pub first_name: String,
            pub last_name: String,
        }

        #[derive(Debug, Serialize)]
        #[serde(rename_all = "camelCase")]
        pub struct User {
            pub id: Uuid,
            pub username: String,
            pub first_name: String,
            pub last_name: String,
            pub bio: Option<String>,
            pub avatar: Option<String>,
            pub socials: Vec<UserSocial>,
            pub work: Option<String>,
        }

        #[derive(Debug, Serialize)]
        #[serde(rename_all = "camelCase")]
        pub struct SessionUser {
            pub id: Uuid,
            pub first_name: String,
            pub last_name: String,
        }

        #[derive(Debug, Serialize)]
        #[serde(rename_all = "camelCase")]
        pub struct UserSocial {
            pub id: Uuid,
            #[serde(rename = "type")]
            pub r#type: String,
            pub username: String,
            pub link: String,
        }

        #[derive(Debug, Serialize)]
        #[serde(rename_all = "camelCase")]
        pub struct Card {
            pub id: Uuid,
            pub title: String,
            pub content: serde_json::Value,
            pub created_at: DateTime<Utc>,
            pub updated_at: DateTime<Utc>,
            /// Author user uuid
            pub author_id: Uuid,
            pub tags: Vec<String>,
        }
    }
}
pub mod paths {
    use super::components::responses;

    pub mod auth_done {
        use super::responses;
        use actix_swagger::ContentType;
        use actix_web::http::StatusCode;
        use actix_web::{HttpRequest, HttpResponse, Responder, ResponseError};
        use serde::Serialize;

        #[derive(Debug, Serialize, thiserror::Error)]
        #[serde(untagged)]
        pub enum Error {
            #[error(transparent)]
            BadRequest(#[from] responses::AuthDoneFailed),
            #[error("Unauthorized")]
            Unauthorized,
            #[error(transparent)]
            Unexpected(
                #[from]
                #[serde(skip)]
                eyre::Report,
            ),
        }

        #[derive(Debug, Serialize)]
        #[serde(untagged)]
        pub enum Response {
            Ok(responses::AuthDoneSuccess),
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
                    Error::Unexpected(_) => StatusCode::INTERNAL_SERVER_ERROR,
                    Error::BadRequest(_) => StatusCode::BAD_REQUEST,
                    Error::Unauthorized => StatusCode::UNAUTHORIZED,
                }
            }

            fn error_response(&self) -> HttpResponse {
                let content_type = match self {
                    Self::BadRequest(_) => Some(ContentType::Json),
                    _ => None,
                };

                let mut res = &mut HttpResponse::build(self.status_code());
                if let Some(content_type) = content_type {
                    res = res.content_type(content_type.to_string());

                    match content_type {
                        ContentType::Json => res.json(self),
                        ContentType::FormData => res.body(serde_plain::to_string(self).unwrap()),
                    }
                } else {
                    HttpResponse::build(self.status_code()).finish()
                }
            }
        }
    }

    pub mod auth_params {
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

    pub mod cards_create {
        use super::responses;
        use actix_swagger::ContentType;
        use actix_web::http::StatusCode;
        use actix_web::{HttpRequest, HttpResponse, Responder, ResponseError};
        use serde::Serialize;

        #[derive(Debug, Serialize)]
        #[serde(untagged)]
        pub enum Response {
            Ok(responses::CardsCreateSuccess),
        }

        #[derive(Debug, Serialize, thiserror::Error)]
        #[serde(untagged)]
        pub enum Error {
            #[error(transparent)]
            BadRequest(#[from] responses::CardsCreateFailed),
            #[error("Unauthorized")]
            Unauthorized,
            #[error(transparent)]
            InternalServerError(
                #[from]
                #[serde(skip)]
                eyre::Report,
            ),
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
                    Error::BadRequest(_) => StatusCode::BAD_REQUEST,
                    Error::Unauthorized => StatusCode::UNAUTHORIZED,
                }
            }

            fn error_response(&self) -> HttpResponse {
                let content_type = match self {
                    Self::BadRequest(_) => Some(ContentType::Json),
                    _ => None,
                };

                let mut res = &mut HttpResponse::build(self.status_code());
                if let Some(content_type) = content_type {
                    res = res.content_type(content_type.to_string());

                    match content_type {
                        ContentType::Json => res.json(self),
                        ContentType::FormData => res.body(serde_plain::to_string(self).unwrap()),
                    }
                } else {
                    HttpResponse::build(self.status_code()).finish()
                }
            }
        }
    }

    pub mod cards_search {
        use super::responses;
        use actix_web::http::StatusCode;
        use actix_web::{HttpRequest, HttpResponse, Responder, ResponseError};
        use serde::Serialize;

        #[derive(Debug, Serialize)]
        #[serde(untagged)]
        pub enum Response {
            Ok(responses::CardsSearchSuccess),
        }

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

        impl Responder for Response {
            #[inline]
            fn respond_to(self, _: &HttpRequest) -> HttpResponse {
                match self {
                    Response::Ok(r) => HttpResponse::build(StatusCode::OK).json(r),
                }
            }
        }

        impl ResponseError for Error {
            #[inline]
            fn status_code(&self) -> StatusCode {
                match self {
                    Error::InternalServerError(_) => StatusCode::INTERNAL_SERVER_ERROR,
                }
            }

            #[inline]
            fn error_response(&self) -> HttpResponse {
                HttpResponse::build(self.status_code()).finish()
            }
        }
    }

    pub mod cards_edit {
        use super::responses;
        use actix_swagger::ContentType;
        use actix_web::http::StatusCode;
        use actix_web::{HttpRequest, HttpResponse, Responder, ResponseError};
        use serde::Serialize;

        #[derive(Debug, Serialize)]
        #[serde(untagged)]
        pub enum Response {
            Ok(responses::CardsEditSuccess),
        }

        #[derive(Debug, Serialize, thiserror::Error)]
        #[serde(untagged)]
        pub enum Error {
            #[error(transparent)]
            BadRequest(#[from] responses::CardsEditFailed),
            #[error(transparent)]
            InternalServerError(
                #[from]
                #[serde(skip)]
                eyre::Report,
            ),
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
                    Error::BadRequest(_) => StatusCode::BAD_REQUEST,
                }
            }

            fn error_response(&self) -> HttpResponse {
                let content_type = match self {
                    Self::BadRequest(_) => Some(ContentType::Json),
                    _ => None,
                };

                let mut res = &mut HttpResponse::build(self.status_code());
                if let Some(content_type) = content_type {
                    res = res.content_type(content_type.to_string());

                    match content_type {
                        ContentType::Json => res.json(self),
                        ContentType::FormData => res.body(serde_plain::to_string(self).unwrap()),
                    }
                } else {
                    HttpResponse::build(self.status_code()).finish()
                }
            }
        }
    }

    pub mod cards_delete {
        use super::responses;
        use actix_swagger::ContentType;
        use actix_web::http::StatusCode;
        use actix_web::{HttpRequest, HttpResponse, Responder, ResponseError};
        use serde::Serialize;

        #[derive(Debug, Serialize)]
        #[serde(untagged)]
        pub enum Response {
            Ok(responses::CardsDeleteSuccess),
        }

        #[derive(Debug, Serialize, thiserror::Error)]
        #[serde(untagged)]
        pub enum Error {
            #[error(transparent)]
            BadRequest(#[from] responses::CardsDeleteFailed),
            #[error(transparent)]
            InternalServerError(
                #[from]
                #[serde(skip)]
                eyre::Report,
            ),
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
                    Error::BadRequest(_) => StatusCode::BAD_REQUEST,
                }
            }

            fn error_response(&self) -> HttpResponse {
                let content_type = match self {
                    Self::BadRequest(_) => Some(ContentType::Json),
                    _ => None,
                };

                let mut res = &mut HttpResponse::build(self.status_code());
                if let Some(content_type) = content_type {
                    res = res.content_type(content_type.to_string());

                    match content_type {
                        ContentType::Json => res.json(self),
                        ContentType::FormData => res.body(serde_plain::to_string(self).unwrap()),
                    }
                } else {
                    HttpResponse::build(self.status_code()).finish()
                }
            }
        }
    }

    pub mod cards_save {
        use super::responses;
        use actix_swagger::ContentType;
        use actix_web::http::StatusCode;
        use actix_web::{HttpRequest, HttpResponse, Responder, ResponseError};
        use serde::Serialize;

        #[derive(Debug, Serialize)]
        #[serde(untagged)]
        pub enum Response {
            Ok(responses::CardsSaveSuccess),
        }

        #[derive(Debug, Serialize, thiserror::Error)]
        #[serde(untagged)]
        pub enum Error {
            #[error(transparent)]
            BadRequest(#[from] responses::CardsSaveFailed),
            #[error(transparent)]
            InternalServerError(
                #[from]
                #[serde(skip)]
                eyre::Report,
            ),
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
                    Error::BadRequest(_) => StatusCode::BAD_REQUEST,
                }
            }

            fn error_response(&self) -> HttpResponse {
                let content_type = match self {
                    Self::BadRequest(_) => Some(ContentType::Json),
                    _ => None,
                };

                let mut res = &mut HttpResponse::build(self.status_code());
                if let Some(content_type) = content_type {
                    res = res.content_type(content_type.to_string());

                    match content_type {
                        ContentType::Json => res.json(self),
                        ContentType::FormData => res.body(serde_plain::to_string(self).unwrap()),
                    }
                } else {
                    HttpResponse::build(self.status_code()).finish()
                }
            }
        }
    }

    pub mod cards_unsave {
        use super::responses;
        use actix_swagger::ContentType;
        use actix_web::http::StatusCode;
        use actix_web::{HttpRequest, HttpResponse, Responder, ResponseError};
        use serde::Serialize;

        #[derive(Debug, Serialize)]
        #[serde(untagged)]
        pub enum Response {
            Ok(responses::CardsUnsaveSuccess),
        }

        #[derive(Debug, Serialize, thiserror::Error)]
        #[serde(untagged)]
        pub enum Error {
            #[error(transparent)]
            BadRequest(#[from] responses::CardsUnsaveFailed),
            #[error(transparent)]
            InternalServerError(
                #[from]
                #[serde(skip)]
                eyre::Report,
            ),
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
                    Error::BadRequest(_) => StatusCode::BAD_REQUEST,
                }
            }

            fn error_response(&self) -> HttpResponse {
                let content_type = match self {
                    Self::BadRequest(_) => Some(ContentType::Json),
                    _ => None,
                };

                let mut res = &mut HttpResponse::build(self.status_code());
                if let Some(content_type) = content_type {
                    res = res.content_type(content_type.to_string());

                    match content_type {
                        ContentType::Json => res.json(self),
                        ContentType::FormData => res.body(serde_plain::to_string(self).unwrap()),
                    }
                } else {
                    HttpResponse::build(self.status_code()).finish()
                }
            }
        }
    }

    pub mod cards_list {
        use super::responses;
        use actix_swagger::ContentType;
        use actix_web::http::StatusCode;
        use actix_web::{HttpRequest, HttpResponse, Responder, ResponseError};
        use serde::Serialize;

        #[derive(Debug, Serialize)]
        #[serde(untagged)]
        pub enum Response {
            Ok(responses::CardsListSuccess),
        }

        #[derive(Debug, Serialize, thiserror::Error)]
        #[serde(untagged)]
        pub enum Error {
            #[error(transparent)]
            BadRequest(#[from] responses::CardsListFailed),
            #[error(transparent)]
            InternalServerError(
                #[from]
                #[serde(skip)]
                eyre::Report,
            ),
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
                    Error::BadRequest(_) => StatusCode::BAD_REQUEST,
                }
            }

            fn error_response(&self) -> HttpResponse {
                let content_type = match self {
                    Self::BadRequest(_) => Some(ContentType::Json),
                    _ => None,
                };

                let mut res = &mut HttpResponse::build(self.status_code());
                if let Some(content_type) = content_type {
                    res = res.content_type(content_type.to_string());

                    match content_type {
                        ContentType::Json => res.json(self),
                        ContentType::FormData => res.body(serde_plain::to_string(self).unwrap()),
                    }
                } else {
                    HttpResponse::build(self.status_code()).finish()
                }
            }
        }
    }

    pub mod cards_get {
        use super::responses;
        use actix_swagger::ContentType;
        use actix_web::http::StatusCode;
        use actix_web::{HttpRequest, HttpResponse, Responder, ResponseError};
        use serde::Serialize;

        #[derive(Debug, Serialize)]
        #[serde(untagged)]
        pub enum Response {
            Ok(responses::CardsGetSuccess),
        }

        #[derive(Debug, Serialize, thiserror::Error)]
        #[serde(untagged)]
        pub enum Error {
            #[error(transparent)]
            BadRequest(#[from] responses::CardsGetFailed),
            #[error(transparent)]
            InternalServerError(
                #[from]
                #[serde(skip)]
                eyre::Report,
            ),
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
                    Error::BadRequest(_) => StatusCode::BAD_REQUEST,
                }
            }

            fn error_response(&self) -> HttpResponse {
                let content_type = match self {
                    Self::BadRequest(_) => Some(ContentType::Json),
                    _ => None,
                };

                let mut res = &mut HttpResponse::build(self.status_code());
                if let Some(content_type) = content_type {
                    res = res.content_type(content_type.to_string());

                    match content_type {
                        ContentType::Json => res.json(self),
                        ContentType::FormData => res.body(serde_plain::to_string(self).unwrap()),
                    }
                } else {
                    HttpResponse::build(self.status_code()).finish()
                }
            }
        }
    }

    pub mod users_get {
        use super::responses;
        use actix_swagger::ContentType;
        use actix_web::http::StatusCode;
        use actix_web::{HttpRequest, HttpResponse, Responder, ResponseError};
        use serde::Serialize;

        #[derive(Debug, Serialize)]
        #[serde(untagged)]
        pub enum Response {
            Ok(responses::UsersGetSuccess),
        }

        #[derive(Debug, Serialize, thiserror::Error)]
        #[serde(untagged)]
        pub enum Error {
            #[error(transparent)]
            BadRequest(#[from] responses::UsersGetFailed),
            #[error(transparent)]
            InternalServerError(
                #[from]
                #[serde(skip)]
                eyre::Report,
            ),
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
                    Error::BadRequest(_) => StatusCode::BAD_REQUEST,
                }
            }

            fn error_response(&self) -> HttpResponse {
                let content_type = match self {
                    Self::BadRequest(_) => Some(ContentType::Json),
                    _ => None,
                };

                let mut res = &mut HttpResponse::build(self.status_code());
                if let Some(content_type) = content_type {
                    res = res.content_type(content_type.to_string());

                    match content_type {
                        ContentType::Json => res.json(self),
                        ContentType::FormData => res.body(serde_plain::to_string(self).unwrap()),
                    }
                } else {
                    HttpResponse::build(self.status_code()).finish()
                }
            }
        }
    }

    pub mod session_get {
        use super::responses;
        use actix_web::http::StatusCode;
        use actix_web::{HttpRequest, HttpResponse, Responder, ResponseError};
        use serde::Serialize;

        #[derive(Debug, Serialize)]
        #[serde(untagged)]
        pub enum Response {
            Ok(responses::SessionGetSuccess),
        }

        #[derive(Debug, Serialize, thiserror::Error)]
        #[serde(untagged)]
        pub enum Error {
            #[error("Unauthorized")]
            Unauthorized,
            #[error(transparent)]
            InternalServerError(
                #[from]
                #[serde(skip)]
                eyre::Report,
            ),
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
                    Error::Unauthorized => StatusCode::UNAUTHORIZED,
                }
            }
        }
    }

    pub mod users_search {
        use super::responses;
        use actix_web::http::StatusCode;
        use actix_web::{HttpRequest, HttpResponse, Responder, ResponseError};
        use serde::Serialize;

        #[derive(Debug, Serialize)]
        #[serde(untagged)]
        pub enum Response {
            Ok(responses::UsersSearchSuccess),
        }

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

        impl Responder for Response {
            #[inline]
            fn respond_to(self, _: &HttpRequest) -> HttpResponse {
                match self {
                    Response::Ok(r) => HttpResponse::build(StatusCode::OK).json(r),
                }
            }
        }

        impl ResponseError for Error {
            #[inline]
            fn status_code(&self) -> StatusCode {
                match self {
                    Error::InternalServerError(_) => StatusCode::INTERNAL_SERVER_ERROR,
                }
            }
        }
    }

    pub mod cards_feed {
        use super::responses;
        use actix_web::http::StatusCode;
        use actix_web::{HttpRequest, HttpResponse, Responder, ResponseError};
        use serde::Serialize;

        #[derive(Debug, Serialize)]
        #[serde(untagged)]
        pub enum Response {
            Ok(responses::CardsFeedSuccess),
        }

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

        impl Responder for Response {
            #[inline]
            fn respond_to(self, _: &HttpRequest) -> HttpResponse {
                match self {
                    Response::Ok(r) => HttpResponse::build(StatusCode::OK).json(r),
                }
            }
        }

        impl ResponseError for Error {
            #[inline]
            fn status_code(&self) -> StatusCode {
                match self {
                    Error::InternalServerError(_) => StatusCode::INTERNAL_SERVER_ERROR,
                }
            }
        }
    }
}
