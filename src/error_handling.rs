use axum::{extract::Json, http::StatusCode};
use serde::Serialize;
use thiserror::Error;
use tracing::*;

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
    #[error("Unauthorized")]
    Authorized,
    #[error("{} is broken UUID", .0)]
    BrokenUuid(String),
    // 外部から投げられたidなどが間違っていて
    // データが見つけられなかった状況
    #[error("Couldn't find {} from database", .0)]
    DatabaseNotFound(String),
    #[error("Failed to build the Tokio Runtime")]
    TokioRuntime,
    #[error("Failed to run migrations")]
    Migrations,
    #[error("Failed to connection pool")]
    ConnectionPool,
    #[error("Failed to set logging confing")]
    LoggingConfig,
    #[error("Failed to serve app")]
    Serve,
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

pub type ReturnData<T> = (StatusCode, Json<Msg<T>>);

pub async fn result_to_handler<T>(res: &Result<T>) -> ReturnData<T>
where
    T: Serialize + Clone,
{
    match res {
        Ok(t) => (
            StatusCode::OK,
            Json(Msg {
                ok: true,
                data: Some(t.clone()),
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
                Authorized => (StatusCode::UNAUTHORIZED, "Authorized"),
                BrokenUuid(_) => (StatusCode::BAD_REQUEST, "BrokenUuid"),
                DatabaseNotFound(_) => (StatusCode::BAD_REQUEST, "DatabaseNotFound"),
                TokioRuntime => (StatusCode::INTERNAL_SERVER_ERROR, "TokioRutime"),
                ConnectionPool => (StatusCode::INTERNAL_SERVER_ERROR, "ConnectionPool"),
                Migrations => (StatusCode::INTERNAL_SERVER_ERROR, "Migrations"),
                LoggingConfig => (StatusCode::INTERNAL_SERVER_ERROR, "LoggingConfing"),
                Serve => (StatusCode::INTERNAL_SERVER_ERROR, "Serve"),
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

pub async fn result_to_handler_with_log<S, F, T>(
    success_msg: S,
    failed_msg: F,
    res: &Result<T>,
) -> ReturnData<T>
where
    S: Fn(&String) -> Option<String>,
    F: Fn(&String) -> Option<String>,
    T: Serialize + Clone,
{
    match res.clone() {
        Ok(s) => {
            let s = success_msg(&serde_json::to_string(&s).unwrap());
            if let Some(s) = s {
                info!("{}", s)
            }
        }
        Err(e) => {
            let s = failed_msg(&e.to_string());
            if let Some(s) = s {
                error!("{}", s)
            }
        }
    }
    result_to_handler(res).await
}
