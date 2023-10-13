use crate::Container;
use axum::{extract::Json, http::StatusCode};
use sqlx::{pool::Pool, postgres::Postgres};
use std::sync::Arc;
use tracing::*;

pub async fn insert_container(
    Json(container): Json<Container>,
    conn: Arc<Pool<Postgres>>,
) -> StatusCode {
    info!("Try insert container: {container:?}");
    match crate::database::insert_container::insert_container(&*conn, container.clone()).await {
        Ok(()) => {
            info!("Success insert container[{}]", &container.id);
            StatusCode::ACCEPTED
        }
        Err(err) => {
            error!("Failed insert container[{}]: {err}", &container.id);
            StatusCode::BAD_REQUEST
        }
    }
}
