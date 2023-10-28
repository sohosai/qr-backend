use crate::{
    authentication::{self, str_to_role_opt},
    error_handling::{result_to_handler_with_log, QrError, Result, ReturnData},
};
use axum::headers::authorization::Basic;
use sqlx::{pool::Pool, postgres::Postgres};
use std::sync::Arc;
use tracing::*;

pub async fn api_gen_passtoken(token_info: Basic, conn: Arc<Pool<Postgres>>) -> ReturnData<String> {
    info!("Try gen passtoken");
    let res = gen_passtoken(token_info, conn).await;
    result_to_handler_with_log(
        |_| Some("Success gen passtoken".to_string()),
        |e| Some(format!("Failed gen passtoken: {e}")),
        &res,
    )
    .await
}

pub async fn gen_passtoken(token_info: Basic, conn: Arc<Pool<Postgres>>) -> Result<String> {
    let role_str = token_info.username();
    let key = token_info.password();
    match str_to_role_opt(role_str) {
        Some(role) => {
            let passtoken = authentication::gen_passtoken(role, key)?;
            authentication::insert_passtoken(&*conn, &passtoken).await?;
            Ok(passtoken.token)
        }
        None => Err(QrError::Authorized),
    }
}
