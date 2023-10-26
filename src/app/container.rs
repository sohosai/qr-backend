use crate::{
    error_handling::{result_to_handler_with_log, ReturnData},
    Container,
};
use axum::extract::Json;
use sqlx::{pool::Pool, postgres::Postgres};
use std::sync::Arc;
use tracing::*;

pub async fn insert_container(
    Json(container): Json<Container>,
    conn: Arc<Pool<Postgres>>,
) -> ReturnData<()> {
    info!("Try insert container: {container:?}");
    let res = crate::database::insert_container::insert_container(&*conn, container.clone()).await;
    result_to_handler_with_log(
        |_| Some(format!("Success insert container[{}]", &container.id)),
        |e| Some(format!("{e} [{}]", &container.id)),
        &res,
    )
    .await
}
