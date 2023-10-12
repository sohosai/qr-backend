use crate::database::get_fixtures_list::{self, SelectInfo};
use crate::database::get_one_fixtures::{get_one_fixtures, IdType};
use crate::search_engine::{SearchFixtures, SearchResult};
use crate::{Fixtures, Stroge};
use axum::{extract::Json, http::StatusCode};
use sqlx::{pool::Pool, postgres::Postgres};
use std::collections::HashMap;
use std::sync::Arc;
use uuid::Uuid;

/// 備品情報の登録を行うエンドポイント
/// - https://github.com/sohosai/qr-backend/issues/11
pub async fn insert_fixtures(
    Json(fixtures): Json<Fixtures>,
    conn: Arc<Pool<Postgres>>,
    context: Arc<SearchFixtures>,
) -> StatusCode {
    match crate::database::insert_fixtures::insert_fixtures(&*conn, fixtures.clone()).await {
        Ok(()) => match context.add_or_replace(&vec![fixtures]).await {
            Ok(_) => StatusCode::ACCEPTED,
            _ => StatusCode::BAD_REQUEST,
        },
        _ => StatusCode::BAD_REQUEST,
    }
}

pub async fn update_fixtures(
    Json(fixtures): Json<Fixtures>,
    conn: Arc<Pool<Postgres>>,
    context: Arc<SearchFixtures>,
) -> StatusCode {
    match crate::database::update_fixtures::update_fixtures(&*conn, fixtures.clone()).await {
        Ok(()) => match context.add_or_replace(&vec![fixtures]).await {
            Ok(_) => StatusCode::ACCEPTED,
            _ => StatusCode::BAD_REQUEST,
        },
        _ => StatusCode::BAD_REQUEST,
    }
}

pub async fn delete_fixtures(
    uuid: Option<Uuid>,
    conn: Arc<Pool<Postgres>>,
    context: Arc<SearchFixtures>,
) -> StatusCode {
    match uuid {
        Some(uuid) => match crate::database::delete_fixtures::delete_fixtures(&*conn, uuid).await {
            Ok(()) => {
                let context = &*context;
                match context.delete(&vec![uuid]).await {
                    Ok(_) => StatusCode::ACCEPTED,
                    _ => StatusCode::BAD_REQUEST,
                }
            }
            _ => StatusCode::BAD_REQUEST,
        },
        None => StatusCode::BAD_REQUEST,
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
                match get_one_fixtures(&*conn, IdType::FixturesId(uuid)).await {
                    Ok(f) => Json(f),
                    Err(_) => Json(None),
                }
            } else {
                Json(None)
            }
        }
        (_, Some(qr_id)) => match get_one_fixtures(&*conn, IdType::QrId(qr_id.clone())).await {
            Ok(f) => Json(f),
            Err(_) => Json(None),
        },
        _ => Json(None),
    }
}

pub async fn get_fixtures_list(
    query: HashMap<String, String>,
    conn: Arc<Pool<Postgres>>,
) -> Json<Option<Vec<Fixtures>>> {
    match (
        query.get("id"),
        query.get("qr_id"),
        query.get("name"),
        query.get("description"),
        query.get("storage"),
        query.get("parent_id"),
    ) {
        (Some(id), _, _, _, _, _) => {
            let uuid_opt = Uuid::parse_str(id).ok();
            if let Some(uuid) = uuid_opt {
                match get_fixtures_list::get_fixtures_list(&*conn, SelectInfo::Id(uuid)).await {
                    Ok(f) => axum::Json(Some(f)),
                    Err(_) => axum::Json(None),
                }
            } else {
                axum::Json(None)
            }
        }
        (_, Some(qr_id), _, _, _, _) => {
            match get_fixtures_list::get_fixtures_list(&*conn, SelectInfo::QrId(qr_id.clone()))
                .await
            {
                Ok(f) => axum::Json(Some(f)),
                Err(_) => axum::Json(None),
            }
        }
        (_, _, Some(name), _, _, _) => {
            match get_fixtures_list::get_fixtures_list(&*conn, SelectInfo::Name(name.clone())).await
            {
                Ok(f) => axum::Json(Some(f)),
                Err(_) => axum::Json(None),
            }
        }
        (_, _, _, Some(description), _, _) => {
            match get_fixtures_list::get_fixtures_list(
                &*conn,
                SelectInfo::Description(description.clone()),
            )
            .await
            {
                Ok(f) => axum::Json(Some(f)),
                Err(_) => axum::Json(None),
            }
        }
        (_, _, _, _, Some(storage), _) => {
            match get_fixtures_list::get_fixtures_list(
                &*conn,
                SelectInfo::Storage(Stroge::from(storage.clone())),
            )
            .await
            {
                Ok(f) => axum::Json(Some(f)),
                Err(_) => axum::Json(None),
            }
        }
        (_, _, _, _, _, Some(parent_id)) => {
            match get_fixtures_list::get_fixtures_list(
                &*conn,
                SelectInfo::ParentId(parent_id.clone()),
            )
            .await
            {
                Ok(f) => axum::Json(Some(f)),
                Err(_) => axum::Json(None),
            }
        }
        _ => axum::Json(None),
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
    match context.search(&keywords).await {
        Ok(res) => Json(Some(res)),
        _ => Json(None),
    }
}
