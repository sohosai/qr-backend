use crate::certification::{get_role, Role};
use crate::{
    error_handling::{result_to_handler, result_to_handler_with_log, QrError, ReturnData},
    Spot,
};
use axum::{extract::Json, headers::authorization::Bearer};
use sqlx::{pool::Pool, postgres::Postgres};
use std::collections::HashMap;
use std::sync::Arc;
use tracing::*;

/// 地点情報の登録を行うエンドポイント
pub async fn insert_spot(
    bearer: Bearer,
    Json(spot): Json<Spot>,
    conn: Arc<Pool<Postgres>>,
) -> ReturnData<()> {
    let role = get_role(&*conn, bearer.token()).await;
    info!("role: {role:?}");
    if Ok(Role::EquipmentManager) == role || Ok(Role::Administrator) == role {
        info!("Try insert spot: {spot:?}");
        let res = crate::database::insert_spot::insert_spot(&*conn, spot.clone()).await;
        result_to_handler_with_log(
            |_| Some(format!("Success insert spot[{}]", &spot.name)),
            |e| Some(format!("{e} spot[{}]", &spot.name)),
            &res,
        )
        .await
    } else {
        result_to_handler(&Err(QrError::Authorized)).await
    }
}

/// 地点情報の更新を行うエンドポイント
pub async fn update_spot(
    bearer: Bearer,
    Json(spot): Json<Spot>,
    conn: Arc<Pool<Postgres>>,
) -> ReturnData<()> {
    let role = get_role(&*conn, bearer.token()).await;
    if Ok(Role::EquipmentManager) == role && Ok(Role::Administrator) == role {
        info!("Try update spot: {spot:?}");
        let res = crate::database::update_spot::update_spot(&*conn, spot.clone()).await;
        result_to_handler_with_log(
            |_| Some(format!("Success update spot[{}]", &spot.name)),
            |e| Some(format!("{e} spot[{}]", &spot.name)),
            &res,
        )
        .await
    } else {
        result_to_handler(&Err(QrError::Authorized)).await
    }
}

/// 地点情報の取得を行うエンドポイント
pub async fn get_one_spot(
    query: HashMap<String, String>,
    conn: Arc<Pool<Postgres>>,
) -> ReturnData<Spot> {
    match query.get("name") {
        Some(name) => {
            info!("Try get one spot info: {name}");
            let res = crate::database::get_one_spot::get_one_spot(&*conn, name).await;
            result_to_handler_with_log(
                |_| Some(format!("Success get spot with name[{name}]")),
                |e| Some(format!("{e} spot[{name}]")),
                &res,
            )
            .await
        }
        None => {
            let err = Err(QrError::UrlQuery("name".to_string()));
            result_to_handler_with_log(|_| None, |e| Some(e.to_string()), &err).await
        }
    }
}

/// 地点情報一覧の取得を行うエンドポイント
pub async fn get_spot_list(conn: Arc<Pool<Postgres>>) -> ReturnData<Vec<Spot>> {
    info!("Try get spot list");
    let res = crate::database::get_spot_list::get_spot_list(&*conn).await;
    result_to_handler_with_log(
        |_| Some("Success get spot list".to_string()),
        |e| Some(e.to_string()),
        &res,
    )
    .await
}

/// 地点情報の削除を行うエンドポイント
pub async fn delte_spot(
    bearer: Bearer,
    query: HashMap<String, String>,
    conn: Arc<Pool<Postgres>>,
) -> ReturnData<()> {
    let role = get_role(&*conn, bearer.token()).await;
    if Ok(Role::Administrator) == role {
        match query.get("name") {
            Some(name) => {
                info!("Try get one spot info: {name}");
                let res = crate::database::delete_spot::delete_spot(&*conn, name).await;
                result_to_handler_with_log(
                    |_| Some(format!("Success delete spot[{name}]")),
                    |e| Some(format!("{e} spot[{name}]")),
                    &res,
                )
                .await
            }
            None => {
                let err = Err(QrError::UrlQuery("name".to_string()));
                result_to_handler_with_log(|_| None, |e| Some(e.to_string()), &err).await
            }
        }
    } else {
        result_to_handler(&Err(QrError::Authorized)).await
    }
}
