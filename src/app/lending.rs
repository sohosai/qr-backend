use crate::Lending;
use axum::{extract::Json, http::StatusCode};
use chrono::{DateTime, Utc};
use sqlx::{pool::Pool, postgres::Postgres};
use std::collections::HashMap;
use std::sync::Arc;
use tracing::*;
use uuid::Uuid;

/// 備品情報の登録を行うエンドポイント
/// - https://github.com/sohosai/qr-backend/issues/11
pub async fn insert_lending(Json(lending): Json<Lending>, conn: Arc<Pool<Postgres>>) -> StatusCode {
    info!("Try insert lending: {lending:?}");
    match crate::database::insert_lending::insert_lending(&*conn, lending.clone()).await {
        Ok(()) => {
            info!("Success insert lending[{}]", &lending.id);
            StatusCode::ACCEPTED
        }
        Err(err) => {
            error!("Failed insert lending[{}]: {err}", &lending.id);
            StatusCode::BAD_REQUEST
        }
    }
}

pub async fn returned_lending(
    id: Option<Uuid>,
    qr_id: Option<String>,
    returned_at: DateTime<Utc>,
    conn: Arc<Pool<Postgres>>,
) -> StatusCode {
    use crate::database::get_one_fixtures::*;
    use crate::database::returned_lending::*;
    match id {
        Some(id) => {
            info!("Try returned lending with uuid[{}]", { id });
            match returned_lending(&*conn, id, returned_at).await {
                Ok(()) => {
                    info!("Success returned lending with uuid[{}]", id);
                    StatusCode::ACCEPTED
                }
                Err(err) => {
                    error!("Failed retunred lending with uuid[{}]: {err}", id);
                    StatusCode::BAD_REQUEST
                }
            }
        }
        None => match qr_id {
            // QRのIDに該当する物品情報を検索する
            Some(qr_id) => {
                info!("Try returned lending with qr_id[{}]", qr_id);
                match get_one_fixtures(&*conn, IdType::QrId(qr_id.clone())).await {
                    Ok(Some(fixtures)) => {
                        let id = fixtures.id;
                        match returned_lending(&*conn, id, returned_at).await {
                            Ok(()) => {
                                info!("Success returned lending with qr_id[{}]", qr_id);
                                StatusCode::ACCEPTED
                            }
                            Err(err) => {
                                error!("Failed retunred lending with qr_id[{}]: {err}", qr_id);
                                StatusCode::BAD_REQUEST
                            }
                        }
                    }
                    _ => {
                        error!("Not found fixtures: {}", qr_id);
                        StatusCode::BAD_REQUEST
                    }
                }
            }
            None => {
                error!("Not found lending id");
                StatusCode::BAD_REQUEST
            }
        },
    }
}

pub async fn get_lending_list(conn: Arc<Pool<Postgres>>) -> Json<Option<Vec<Lending>>> {
    info!("Try get lending list");
    match crate::database::get_lending_list::get_lending_list(&*conn).await {
        Ok(v) => {
            info!("Success get lending list");
            axum::Json(Some(v))
        }
        Err(err) => {
            error!("Failed get lending list: {err}");
            axum::Json(None)
        }
    }
}

pub async fn get_one_lending(
    query: HashMap<String, String>,
    conn: Arc<Pool<Postgres>>,
) -> Json<Option<Lending>> {
    use crate::database::get_one_lending::*;
    match (
        query.get("lending_id"),
        query.get("fixtures_id"),
        query.get("fixtures_qr_id"),
    ) {
        (Some(lending_id), _, _) => {
            info!("Try get one lending info with lending_id[{lending_id}]");
            let uuid_opt = Uuid::parse_str(lending_id).ok();
            if let Some(uuid) = uuid_opt {
                match get_one_lending(&*conn, IdType::LendingId(uuid)).await {
                    Ok(v) => {
                        info!("Success get lending with lending_id[{lending_id}]");
                        axum::Json(v)
                    }
                    Err(err) => {
                        error!("Failed get lending with lending_id[{lending_id}]: {err}");
                        axum::Json(None)
                    }
                }
            } else {
                error!("Break uuid: {lending_id}");
                Json(None)
            }
        }
        (_, Some(fixtures_id), _) => {
            info!("Try get one lending info with fixtures_id[{fixtures_id}]");
            let uuid_opt = Uuid::parse_str(fixtures_id).ok();
            if let Some(uuid) = uuid_opt {
                match get_one_lending(&*conn, IdType::FixturesId(uuid)).await {
                    Ok(v) => {
                        info!("Success get lending with fixtures_id[{fixtures_id}]");
                        axum::Json(v)
                    }
                    Err(err) => {
                        error!("Failed get lending with fixtures_id[{fixtures_id}]: {err}");
                        axum::Json(None)
                    }
                }
            } else {
                error!("Break uuid: {fixtures_id}");
                Json(None)
            }
        }
        (_, _, Some(qr_id)) => match {
            info!("Try get one lending info with fixtures_qr_id[{qr_id}]");
            get_one_lending(&*conn, IdType::QrId(qr_id.to_string())).await
        } {
            Ok(v) => {
                info!("Success get lending with fixtures_qr_id[{qr_id}]");
                axum::Json(v)
            }
            Err(err) => {
                error!("Failed get lending with fixtures_qr_id[{qr_id}]: {err}");
                axum::Json(None)
            }
        },
        _ => {
            error!("Invalid query");
            Json(None)
        }
    }
}

pub async fn get_is_lending(
    query: HashMap<String, String>,
    conn: Arc<Pool<Postgres>>,
) -> Json<bool> {
    use crate::database::get_one_lending::*;
    info!("Check exist lending info");
    match (
        query.get("lending_id"),
        query.get("fixtures_id"),
        query.get("fixtures_qr_id"),
    ) {
        (Some(lending_id), _, _) => {
            let uuid_opt = Uuid::parse_str(lending_id).ok();
            if let Some(uuid) = uuid_opt {
                match get_one_lending(&*conn, IdType::LendingId(uuid)).await {
                    Ok(Some(_)) => axum::Json(true),
                    _ => axum::Json(false),
                }
            } else {
                error!("Break uuid: {lending_id}");
                Json(false)
            }
        }
        (_, Some(fixtures_id), _) => {
            let uuid_opt = Uuid::parse_str(fixtures_id).ok();
            if let Some(uuid) = uuid_opt {
                match get_one_lending(&*conn, IdType::FixturesId(uuid)).await {
                    Ok(Some(_)) => axum::Json(true),
                    _ => axum::Json(false),
                }
            } else {
                error!("Break uuid: {fixtures_id}");
                Json(false)
            }
        }
        (_, _, Some(qr_id)) => match get_one_lending(&*conn, IdType::QrId(qr_id.to_string())).await
        {
            Ok(Some(_)) => axum::Json(true),
            _ => axum::Json(false),
        },
        _ => {
            error!("Invalid query");
            Json(false)
        }
    }
}

pub async fn update_lending(Json(lending): Json<Lending>, conn: Arc<Pool<Postgres>>) -> StatusCode {
    info!("Try update lending: {lending:?}");
    match crate::database::update_lending::update_lending(&*conn, lending.clone()).await {
        Ok(()) => {
            info!("Success update lending[{}]", lending.id);
            StatusCode::ACCEPTED
        }
        Err(err) => {
            error!("Failed update lending[{}]: {err}", lending.id);
            StatusCode::BAD_REQUEST
        }
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
                    "fixtures_qr_id": "x234",
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
