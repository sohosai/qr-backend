use axum::{extract::Json, http::StatusCode};
use sqlx::{pool::Pool, postgres::Postgres};
use std::sync::Arc;
use uuid::Uuid;

/// 備品情報の登録を行うエンドポイント
/// - https://github.com/sohosai/qr-backend/issues/11
pub async fn insert_equipment(
    Json(equipment): Json<crate::Equipment>,
    conn: Arc<Pool<Postgres>>,
) -> StatusCode {
    match crate::database::insert_equipment::insert_equipment(&*conn, equipment).await {
        Ok(()) => StatusCode::ACCEPTED,
        _ => StatusCode::BAD_REQUEST,
    }
}

pub async fn delete_equipment(uuid: Option<Uuid>, conn: Arc<Pool<Postgres>>) -> StatusCode {
    match uuid {
        Some(uuid) => {
            match crate::database::delete_equipment::delete_equipment(&*conn, uuid).await {
                Ok(()) => StatusCode::ACCEPTED,
                _ => StatusCode::BAD_REQUEST,
            }
        }
        None => StatusCode::BAD_REQUEST,
    }
}

#[cfg(test)]
mod tests {
    use axum::{extract::Json, http::StatusCode};
    use serde_json::json;
    use sqlx::{pool::Pool, Postgres};
    use std::sync::Arc;

    use crate::app::equipment::insert_equipment;

    #[sqlx::test(migrations = "./migrations")]
    async fn test_insert_equipment(pool: Pool<Postgres>) {
        let conn = Arc::new(pool);
        let status_code = insert_equipment(
            Json(
                serde_json::from_value(json!({
                  "id": "550e8400-e29b-41d4-a716-446655440000",
                  "qr_id": "test",
                  "created_at": "2023-08-07 15:56:35 UTC",
                  "qr_color":"red",
                  "name":"テスト物品",
                  "description":"テスト説明",
                  "storage": "Room101",
                  "usage": "無い",
                  "note": "DBを確認",
                  "parent_id": "null"
                }))
                .unwrap(),
            ),
            conn,
        )
        .await;
        assert_eq!(status_code, StatusCode::ACCEPTED)
    }
}
