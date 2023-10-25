use axum::{extract::Json, http::StatusCode};
use serde::Serialize;
use thiserror::Error;

#[derive(Debug, Clone, Error)]
pub enum QrError {
    #[error("Couldn't found {} on environment", .0)]
    Environment(String),
    #[error("Couldn't add/replace {} to search engine", .0)]
    SearchEngineAddOrReplace(String),
    #[error("Couldn't delete {} from search engine", .0)]
    SearchEngineDelete(String),
    #[error("Couldn't search {} from search engine", .0)]
    SearchEngineSearch(String),
    #[error("Couldn't add {} to database", .0)]
    DatabaseAdd(String),
    #[error("Couldn't update {} to database", .0)]
    DatabaseUpdate(String),
    #[error("Couldn't delete {} from database", .0)]
    DatabaseDelete(String),
    #[error("Couldn't get {} from database", .0)]
    DatabaseGet(String),
    #[error("Couldn't found {} query in the url", .0)]
    UrlQuery(String),
}

#[derive(Debug, Clone, Serialize)]
pub struct Msg<T>
where
    T: Serialize,
{
    ok: bool,
    data: Option<T>,
    error_type: Option<String>,
    error_message: Option<String>,
}

pub type Result<T> = std::result::Result<T, QrError>;

type ReturnData<T> = (StatusCode, Json<Msg<T>>);

pub fn result_to_handler<T>(res: Result<T>) -> ReturnData<T>
where
    T: Serialize + Clone,
{
    match res {
        Ok(t) => (
            StatusCode::OK,
            Json(Msg {
                ok: true,
                data: Some(t),
                error_type: None,
                error_message: None,
            }),
        ),

        Err(e) => {
            use QrError::*;
            let (code, error_type) = match e {
                Environment(_) => (StatusCode::SERVICE_UNAVAILABLE, "CouldNotFoundEnv"),
                SearchEngineAddOrReplace(_) => (
                    StatusCode::INTERNAL_SERVER_ERROR,
                    "SearchEngineAddOrReplace",
                ),
                SearchEngineDelete(_) => (StatusCode::INTERNAL_SERVER_ERROR, "SearchEngineDelete"),
                SearchEngineSearch(_) => (StatusCode::INTERNAL_SERVER_ERROR, "SearchEngineSearch"),
                DatabaseAdd(_) => (StatusCode::INTERNAL_SERVER_ERROR, "DatabaseAdd"),
                DatabaseUpdate(_) => (StatusCode::INTERNAL_SERVER_ERROR, "DatabaseUpdate"),
                DatabaseDelete(_) => (StatusCode::INTERNAL_SERVER_ERROR, "DatabaseDelete"),
                DatabaseGet(_) => (StatusCode::INTERNAL_SERVER_ERROR, "DatabaseGet"),
                UrlQuery(_) => (StatusCode::BAD_REQUEST, "UrlQuery"),
            };
            (
                code,
                Json(Msg {
                    ok: true,
                    data: None,
                    error_type: Some(error_type.to_string()),
                    error_message: Some(e.to_string()),
                }),
            )
        }
    }
}
