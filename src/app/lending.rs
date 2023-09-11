use crate::Lending;
use axum::{extract::Json, http::StatusCode};
use chrono::{DateTime, Utc};
use sqlx::{pool::Pool, postgres::Postgres};
use std::sync::Arc;
use uuid::Uuid;

/// 備品情報の登録を行うエンドポイント
/// - https://github.com/sohosai/qr-backend/issues/11
pub async fn insert_lending(Json(lending): Json<Lending>, conn: Arc<Pool<Postgres>>) -> StatusCode {
    match crate::database::insert_lending::insert_lending(&*conn, lending).await {
        Ok(()) => StatusCode::ACCEPTED,
        _ => StatusCode::BAD_REQUEST,
    }
}

pub async fn returned_lending(
    id: Option<Uuid>,
    qr_id: Option<String>,
    returned_at: DateTime<Utc>,
    conn: Arc<Pool<Postgres>>,
) -> StatusCode {
    use crate::database::get_lending_list::*;
    use crate::database::get_one_fixtures::*;
    use crate::database::returned_lending::*;
    match id {
        Some(id) => match returned_lending(&*conn, id, returned_at).await {
            Ok(()) => StatusCode::ACCEPTED,
            _ => StatusCode::BAD_REQUEST,
        },
        None => match qr_id {
            Some(qr_id) => match get_one_fixtures(&*conn, IdType::QrId(qr_id)).await {
                Ok(Some(fixtures)) => match get_lending_list(&*conn).await {
                    Ok(list) => {
                        for lending in list
                            .iter()
                            .filter(|lending| lending.fixtures_id == fixtures.id)
                        {
                            let id = lending.id;
                            match returned_lending(&*conn, id, returned_at).await {
                                Ok(()) => {}
                                _ => return StatusCode::BAD_REQUEST,
                            }
                        }
                        StatusCode::ACCEPTED
                    }
                    _ => StatusCode::BAD_REQUEST,
                },
                _ => StatusCode::BAD_REQUEST,
            },
            None => StatusCode::BAD_REQUEST,
        },
    }
}

pub async fn get_lending_list(conn: Arc<Pool<Postgres>>) -> Json<Option<Vec<Lending>>> {
    match crate::database::get_lending_list::get_lending_list(&*conn).await {
        Ok(v) => axum::Json(Some(v)),
        _ => axum::Json(None),
    }
}

pub async fn update_lending(Json(lending): Json<Lending>, conn: Arc<Pool<Postgres>>) -> StatusCode {
    match crate::database::update_lending::update_lending(&*conn, lending).await {
        Ok(()) => StatusCode::ACCEPTED,
        _ => StatusCode::BAD_REQUEST,
    }
}

#[cfg(test)]
mod tests {
    use axum::{extract::Json, http::StatusCode};
    use serde_json::json;
    use sqlx::{pool::Pool, Postgres};
    use std::sync::Arc;
    use uuid::uuid;

    use crate::app::lending::insert_lending;

    #[sqlx::test(migrations = "./migrations")]
    async fn test_insert_lending(pool: Pool<Postgres>) {
        let conn = Arc::new(pool);
        let id = uuid!("550e8400-e29b-41d4-a716-446655440000");
        let fixtures_id = uuid!("550e8400-e29b-41d4-a716-446655440001");
        let status_code = insert_lending(
            Json(
                serde_json::from_value(json!({
                    "id": id,
                    "fixtures_id": fixtures_id,
                    "spot_name": "test",
                    "lending_at": "2023-08-07 15:56:35 UTC",
                    "borrower_name": "test",
                    "borrower_number": 202200000,
                    "borrower_org": "jsys"
                }))
                .unwrap(),
            ),
            conn,
        )
        .await;
        assert_eq!(status_code, StatusCode::ACCEPTED)
    }
}
