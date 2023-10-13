use crate::database::get_one_fixtures::{get_one_fixtures, IdType};
use crate::search_engine::{SearchFixtures, SearchResult};
use crate::Fixtures;
use axum::{extract::Json, http::StatusCode};
use sqlx::{pool::Pool, postgres::Postgres};
use std::collections::HashMap;
use std::sync::Arc;
use tracing::*;
use uuid::Uuid;

/// 備品情報の登録を行うエンドポイント
/// - https://github.com/sohosai/qr-backend/issues/11
pub async fn insert_fixtures(
    Json(fixtures): Json<Fixtures>,
    conn: Arc<Pool<Postgres>>,
    context: Arc<SearchFixtures>,
) -> StatusCode {
    info!("Try insert fixtures: {fixtures:?}");
    match crate::database::insert_fixtures::insert_fixtures(&*conn, fixtures.clone()).await {
        Ok(()) => {
            info!("Success insert fixtures(DB)[{}]", &fixtures.id);
            match context.add_or_replace(&[fixtures.clone()]).await {
                Ok(_) => {
                    info!("Success insert fixtures(Search Engine)[{}]", &fixtures.id);
                    StatusCode::ACCEPTED
                }
                Err(err) => {
                    error!(
                        "Failed insert fixtures(Search Engine)[{}]: {err}",
                        &fixtures.id
                    );
                    StatusCode::BAD_REQUEST
                }
            }
        }
        Err(err) => {
            error!("Failed insert fixtures(DB)[{}]: {err}", &fixtures.id);
            StatusCode::BAD_REQUEST
        }
    }
}

pub async fn update_fixtures(
    Json(fixtures): Json<Fixtures>,
    conn: Arc<Pool<Postgres>>,
    context: Arc<SearchFixtures>,
) -> StatusCode {
    info!("Try update fixtures: {fixtures:?}");
    match crate::database::update_fixtures::update_fixtures(&*conn, fixtures.clone()).await {
        Ok(()) => {
            info!("Success update fixtures(DB)[{}]", &fixtures.id);
            match context.add_or_replace(&[fixtures.clone()]).await {
                Ok(_) => {
                    info!("Success update fixtures(Search Engine)[{}]", &fixtures.id);
                    StatusCode::ACCEPTED
                }
                Err(err) => {
                    error!(
                        "Failed insert fixtures(Search Engine)[{}]: {err}",
                        &fixtures.id
                    );
                    StatusCode::BAD_REQUEST
                }
            }
        }
        Err(err) => {
            error!("Failed insert fixtures(DB)[{}]: {err}", &fixtures.id);
            StatusCode::BAD_REQUEST
        }
    }
}

pub async fn delete_fixtures(
    uuid: Option<Uuid>,
    conn: Arc<Pool<Postgres>>,
    context: Arc<SearchFixtures>,
) -> StatusCode {
    match uuid {
        Some(uuid) => {
            info!("Try delete fixtures: {uuid}");
            match crate::database::delete_fixtures::delete_fixtures(&*conn, uuid).await {
                Ok(()) => {
                    info!("Success delete fixtures(DB)[{uuid}]");
                    let context = &*context;
                    match context.delete(&[uuid]).await {
                        Ok(()) => {
                            info!("Success delete fixtures(Search Engine)[{uuid}]");
                            StatusCode::ACCEPTED
                        }
                        Err(err) => {
                            error!("Failed insert fixtures(Search Engine)[{uuid}]: {err}");
                            StatusCode::BAD_REQUEST
                        }
                    }
                }
                Err(err) => {
                    error!("Failed insert fixtures(DB)[{uuid}]: {err}");
                    StatusCode::BAD_REQUEST
                }
            }
        }
        None => {
            error!("Not found uuid");
            StatusCode::BAD_REQUEST
        }
    }
}

pub async fn get_fixtures(
    query: HashMap<String, String>,
    conn: Arc<Pool<Postgres>>,
) -> Json<Option<Fixtures>> {
    match (query.get("id"), query.get("qr_id")) {
        (Some(id), _) => {
            let uuid_opt = Uuid::parse_str(id).ok();
            if let Some(uuid) = uuid_opt {
                info!("Try get fixtures with uuid: {uuid}");
                match get_one_fixtures(&*conn, IdType::FixturesId(uuid)).await {
                    Ok(f) => {
                        if f.is_some() {
                            info!("Success get fixtures with uuid[{uuid}]");
                        } else {
                            info!("Not found fixtures[{uuid}]");
                        }
                        Json(f)
                    }
                    Err(err) => {
                        error!("Failed get fixtures with uuid[{uuid}]: {err}");
                        Json(None)
                    }
                }
            } else {
                error!("Break uuid: {id}");
                Json(None)
            }
        }
        (_, Some(qr_id)) => {
            info!("Try get fixtures with qr_id: {qr_id}");
            match get_one_fixtures(&*conn, IdType::QrId(qr_id.clone())).await {
                Ok(f) => {
                    if f.is_some() {
                        info!("Success get fixtures with qr_id[{qr_id}]");
                    } else {
                        info!("Failed get fixtures with qr_id[{qr_id}]");
                    }
                    Json(f)
                }
                Err(err) => {
                    error!("Failed get fixtures[{qr_id}]: {err}");
                    Json(None)
                }
            }
        }
        _ => {
            error!("Invalid query");
            Json(None)
        }
    }
}

pub async fn search_fixtures(
    keywords_str: String,
    context: Arc<SearchFixtures>,
) -> Json<Option<Vec<SearchResult<Fixtures>>>> {
    let keywords = keywords_str
        .split(',') // カンマ区切りであることを要求する
        .map(|s| s.to_string())
        .collect::<Vec<String>>();
    let context = &*context;
    info!("Try search fixtures: {keywords:?}");
    match context.search(&keywords).await {
        Ok(res) => {
            info!("Success search fixtures[{keywords:?}]");
            Json(Some(res))
        }
        Err(err) => {
            error!("Failed search fixtures[{keywords:?}]: {err}");
            Json(None)
        }
    }
}
