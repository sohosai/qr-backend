use crate::Container;
use axum::{extract::Json, http::StatusCode};
use sqlx::{pool::Pool, postgres::Postgres};
use std::sync::Arc;

pub async fn insert_container(
    Json(container): Json<Container>,
    conn: Arc<Pool<Postgres>>,
) -> StatusCode {
    match crate::database::insert_container::insert_container(&*conn, container).await {
        Ok(()) => StatusCode::ACCEPTED,
        _ => StatusCode::BAD_REQUEST,
    }
}
