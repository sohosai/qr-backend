use crate::Spot;
use axum::{extract::Json, http::StatusCode};
use sqlx::{pool::Pool, postgres::Postgres};
use std::sync::Arc;

/// 地点情報の登録を行うエンドポイント
pub async fn insert_spot(Json(spot): Json<Spot>, conn: Arc<Pool<Postgres>>) -> StatusCode {
    match crate::database::insert_spot::insert_spot(&*conn, spot).await {
        Ok(()) => StatusCode::ACCEPTED,
        _ => StatusCode::BAD_REQUEST,
    }
}

/// 地点情報の更新を行うエンドポイント
pub async fn update_spot(Json(spot): Json<Spot>, conn: Arc<Pool<Postgres>>) -> StatusCode {
    match crate::database::update_spot::update_spot(&*conn, spot).await {
        Ok(()) => StatusCode::ACCEPTED,
        _ => StatusCode::BAD_REQUEST,
    }
}

/// 地点情報の取得を行うエンドポイント
pub async fn get_one_spot(name: Option<String>, conn: Arc<Pool<Postgres>>) -> Json<Option<Spot>> {
    match name {
        Some(name) => match crate::database::get_one_spot::get_one_spot(&*conn, &name).await {
            Ok(spot) => Json(spot),
            _ => Json(None),
        },
        _ => Json(None),
    }
}

/// 地点情報一覧の取得を行うエンドポイント
pub async fn get_spot_list(conn: Arc<Pool<Postgres>>) -> Json<Option<Vec<Spot>>> {
    match crate::database::get_spot_list::get_spot_list(&*conn).await {
        Ok(spot) => Json(Some(spot)),
        _ => Json(None),
    }
}

#[cfg(test)]
mod tests {
    use axum::{extract::Json, http::StatusCode};
    use serde_json::json;
    use sqlx::{pool::Pool, Postgres};
    use std::sync::Arc;

    use crate::app::spot::insert_spot;

    #[sqlx::test(migrations = "./migrations")]
    async fn test_insert_spot(pool: Pool<Postgres>) {
        let conn = Arc::new(pool);
        let status_code = insert_spot(
            Json(
                serde_json::from_value(json!({
                  "name": "test",
                  "area": "area1",
                  "building": "3C",
                  "floor": 2,
                }))
                .unwrap(),
            ),
            conn,
        )
        .await;
        assert_eq!(status_code, StatusCode::ACCEPTED)
    }
}
