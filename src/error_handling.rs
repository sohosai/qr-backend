use axum::{extract::Json, http::StatusCode};
use serde::Serialize;
use thiserror::Error;

#[derive(Debug, Clone, Error)]
pub enum QrError {
    #[error("Couldn't found {} on environment", .0)]
    Environment(String),
    #[error("Couldn't add/replace documents to meilisearch")]
    MeilisearchAddOrReplaceDocuments,
    #[error("Couldn't delete documents to meilisearch")]
    MeilisearchDeleteDocuments,
    #[error("Couldn't delete documents to meilisearch")]
    MeilisearchSearchDocuments,
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
                MeilisearchAddOrReplaceDocuments => (
                    StatusCode::INTERNAL_SERVER_ERROR,
                    "MeilisearchAddOrReplaceDocuments",
                ),
                MeilisearchDeleteDocuments => (
                    StatusCode::INTERNAL_SERVER_ERROR,
                    "MeilisearchDeleteDocuments",
                ),
                MeilisearchSearchDocuments => (
                    StatusCode::INTERNAL_SERVER_ERROR,
                    "MeilisearchSearchDocuments",
                ),
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
