use crate::certification::{get_role, Role};
use crate::database::get_one_fixtures::{get_one_fixtures, IdType};
use crate::error_handling::{result_to_handler, result_to_handler_with_log, QrError, ReturnData};
use crate::search_engine::{SearchFixtures, SearchResult};
use crate::Fixtures;
use axum::{extract::Json, headers::authorization::Bearer};
use sqlx::{pool::Pool, postgres::Postgres};
use std::collections::HashMap;
use std::sync::Arc;
use tracing::*;
use uuid::Uuid;

/// 備品情報の登録を行うエンドポイント
/// - https://github.com/sohosai/qr-backend/issues/11
pub async fn insert_fixtures(
    bearer: Bearer,
    Json(fixtures): Json<Fixtures>,
    conn: Arc<Pool<Postgres>>,
    context: Arc<SearchFixtures>,
) -> ReturnData<()> {
    let role = get_role(&*conn, bearer.token()).await;
    if Ok(Role::EquipmentManager) == role {
        info!("Try insert fixtures: {fixtures:?}");
        let res = crate::database::insert_fixtures::insert_fixtures(&*conn, fixtures.clone()).await;

        // DBの処理が成功した時の結果
        let r1 = result_to_handler_with_log(
            |_| Some(format!("Success insert fixtures(DB)[{}]", &fixtures.id)),
            |e| Some(format!("{e}[{}]", &fixtures.id)),
            &res,
        )
        .await;

        if res.is_ok() {
            let res = context.add_or_replace(&[fixtures.clone()]).await;
            result_to_handler_with_log(
                |_| {
                    Some(format!(
                        "Success insert fixtures(Search Engine)[{}]",
                        &fixtures.id
                    ))
                },
                |e| Some(format!("{e}[{}]", &fixtures.id)),
                &res,
            )
            .await
        } else {
            r1
        }
    } else {
        result_to_handler(&Err(QrError::Authorized)).await
    }
}

pub async fn update_fixtures(
    bearer: Bearer,
    Json(fixtures): Json<Fixtures>,
    conn: Arc<Pool<Postgres>>,
    context: Arc<SearchFixtures>,
) -> ReturnData<()> {
    let role = get_role(&*conn, bearer.token()).await;
    if Ok(Role::EquipmentManager) == role {
        info!("Try update fixtures: {fixtures:?}");
        let res = crate::database::update_fixtures::update_fixtures(&*conn, fixtures.clone()).await;

        // DBの処理が成功した時の結果
        let r1 = result_to_handler_with_log(
            |_| Some(format!("Success update fixtures(DB)[{}]", &fixtures.id)),
            |e| Some(format!("{e}[{}]", &fixtures.id)),
            &res,
        )
        .await;

        if res.is_ok() {
            let res = context.add_or_replace(&[fixtures.clone()]).await;
            result_to_handler_with_log(
                |_| {
                    Some(format!(
                        "Success update fixtures(Search Engine)[{}]",
                        &fixtures.id
                    ))
                },
                |e| Some(format!("{e}[{}]", &fixtures.id)),
                &res,
            )
            .await
        } else {
            r1
        }
    } else {
        result_to_handler(&Err(QrError::Authorized)).await
    }
}

pub async fn delete_fixtures(
    bearer: Bearer,
    query: HashMap<String, String>,
    conn: Arc<Pool<Postgres>>,
    context: Arc<SearchFixtures>,
) -> ReturnData<()> {
    let role = get_role(&*conn, bearer.token()).await;
    if Ok(Role::EquipmentManager) == role {
        let id_opt = query.get("id");
        if let Some(id) = id_opt {
            let uuid_opt = Uuid::parse_str(id).ok();
            if let Some(uuid) = uuid_opt {
                info!("Try delete fixtures: {uuid}");
                let res = crate::database::delete_fixtures::delete_fixtures(&*conn, uuid).await;

                // DBの処理が成功した時の結果
                let r1 = result_to_handler_with_log(
                    |_| Some(format!("Success delete fixtures(DB)[{uuid}]")),
                    |e| Some(format!("{e}[{uuid}]")),
                    &res,
                )
                .await;

                if res.is_ok() {
                    let res = context.delete(&[uuid]).await;
                    result_to_handler_with_log(
                        |_| Some(format!("Success delete fixtures(Search Engine)[{uuid}]")),
                        |e| Some(format!("{e}[{uuid}]")),
                        &res,
                    )
                    .await
                } else {
                    r1
                }
            } else {
                let err = Err(QrError::BrokenUuid(id.to_string()));
                result_to_handler_with_log(|_| None, |e| Some(e.to_string()), &err).await
            }
        } else {
            let err = Err(QrError::UrlQuery("id".to_string()));
            result_to_handler_with_log(|_| None, |e| Some(e.to_string()), &err).await
        }
    } else {
        result_to_handler(&Err(QrError::Authorized)).await
    }
}

pub async fn get_fixtures(
    query: HashMap<String, String>,
    conn: Arc<Pool<Postgres>>,
) -> ReturnData<Fixtures> {
    match (query.get("id"), query.get("qr_id")) {
        (Some(id), _) => {
            let uuid_opt = Uuid::parse_str(id).ok();
            if let Some(uuid) = uuid_opt {
                info!("Try get fixtures with uuid: {uuid}");
                let res = get_one_fixtures(&*conn, IdType::FixturesId(uuid)).await;
                result_to_handler_with_log(
                    |_| Some(format!("Success get fixtures with uuid[{uuid}]")),
                    |e| Some(format!("{e}[{uuid}]")),
                    &res,
                )
                .await
            } else {
                let err = Err(QrError::BrokenUuid(id.to_string()));
                result_to_handler_with_log(|_| None, |e| Some(e.to_string()), &err).await
            }
        }
        (_, Some(qr_id)) => {
            info!("Try get fixtures with qr_id: {qr_id}");
            let res = get_one_fixtures(&*conn, IdType::QrId(qr_id.clone())).await;
            result_to_handler_with_log(
                |_| Some(format!("Success get fixtures with qr_id[{qr_id}]")),
                |e| Some(format!("{e}[{qr_id}]")),
                &res,
            )
            .await
        }
        _ => {
            let err = Err(QrError::UrlQuery("qr_id, id".to_string()));
            result_to_handler_with_log(|_| None, |e| Some(e.to_string()), &err).await
        }
    }
}

pub async fn search_fixtures(
    keywords_str: String,
    context: Arc<SearchFixtures>,
) -> ReturnData<Vec<SearchResult<Fixtures>>> {
    let keywords = keywords_str
        .split(',') // カンマ区切りであることを要求する
        .map(|s| s.to_string())
        .collect::<Vec<String>>();
    let context = &*context;
    info!("Try search fixtures: {keywords:?}");
    let res = context.search(&keywords).await;
    result_to_handler_with_log(
        |_| Some(format!("Success search fixtures[{keywords:?}]")),
        |e| Some(format!("{e}[{keywords:?}]")),
        &res,
    )
    .await
}
