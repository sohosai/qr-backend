use crate::authentication::{get_role, Role};
use crate::{
    error_handling::{result_to_handler, result_to_handler_with_log, QrError, ReturnData},
    Lending,
};
use axum::{extract::Json, headers::authorization::Bearer};
use chrono::{DateTime, Utc};
use sqlx::{pool::Pool, postgres::Postgres};
use std::collections::HashMap;
use std::sync::Arc;
use tracing::*;
use uuid::Uuid;

/// 備品情報の登録を行うエンドポイント
/// - https://github.com/sohosai/qr-backend/issues/11
pub async fn insert_lending(
    bearer: Bearer,
    Json(lending): Json<Lending>,
    conn: Arc<Pool<Postgres>>,
) -> ReturnData<()> {
    let role = get_role(&*conn, bearer.token()).await;
    if Ok(Role::EquipmentManager) == role || Ok(Role::Administrator) == role {
        info!("Try insert lending: {lending:?}");
        let res = crate::database::insert_lending::insert_lending(&*conn, lending.clone()).await;
        result_to_handler_with_log(
            |_| Some(format!("Success insert lending[{}]", &lending.id)),
            |e| Some(format!("{e}[{}]", &lending.id)),
            &res,
        )
        .await
    } else {
        result_to_handler(&Err(QrError::Authorized)).await
    }
}

pub async fn returned_lending(
    bearer: Bearer,
    query: HashMap<String, String>,
    returned_at: DateTime<Utc>,
    conn: Arc<Pool<Postgres>>,
) -> ReturnData<()> {
    use crate::database::get_one_fixtures::*;
    use crate::database::returned_lending::*;
    let role = get_role(&*conn, bearer.token()).await;
    if Ok(Role::EquipmentManager) == role || Ok(Role::Administrator) == role {
        match (query.get("id"), query.get("qr_id")) {
            (Some(id), _) => {
                let uuid_opt = Uuid::parse_str(id).ok();
                if let Some(uuid) = uuid_opt {
                    info!("Try get fixtures with uuid: {uuid}");
                    let res = returned_lending(&*conn, uuid, returned_at).await;
                    result_to_handler_with_log(
                        |_| Some(format!("Success returned lending with uuid[{uuid}]")),
                        |e| Some(format!("{e} uuid[{uuid}]")),
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
                let fixtures = get_one_fixtures(&*conn, IdType::QrId(qr_id.clone())).await;
                match fixtures {
                    Ok(fixtures) => {
                        let res = returned_lending(&*conn, fixtures.id, returned_at).await;
                        result_to_handler_with_log(
                            |_| Some(format!("Success returned lending with qr_id[{qr_id}]")),
                            |e| Some(format!("{e} qr_id[{qr_id}]")),
                            &res,
                        )
                        .await
                    }
                    Err(e) => {
                        result_to_handler_with_log(
                            |_| None,
                            |e| Some(format!("{e} qr_id[{qr_id}]")),
                            &Err(e),
                        )
                        .await
                    }
                }
            }
            _ => {
                let err = Err(QrError::UrlQuery("qr_id, id".to_string()));
                result_to_handler_with_log(|_| None, |e| Some(e.to_string()), &err).await
            }
        }
    } else {
        result_to_handler(&Err(QrError::Authorized)).await
    }
}

pub async fn get_lending_list(conn: Arc<Pool<Postgres>>) -> ReturnData<Vec<Lending>> {
    info!("Try get lending list");
    let res = crate::database::get_lending_list::get_lending_list(&*conn).await;
    result_to_handler_with_log(
        |_| Some("Success get lending list".to_string()),
        |e| Some(e.to_string()),
        &res,
    )
    .await
}

pub async fn get_one_lending(
    query: HashMap<String, String>,
    conn: Arc<Pool<Postgres>>,
) -> ReturnData<Lending> {
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
                let res = get_one_lending(&*conn, IdType::LendingId(uuid)).await;
                result_to_handler_with_log(
                    |_| Some(format!("Success get lending with lending_id[{lending_id}]")),
                    |e| Some(format!("{e} lending_id[{lending_id}]")),
                    &res,
                )
                .await
            } else {
                let err = Err(QrError::BrokenUuid(lending_id.to_string()));
                result_to_handler_with_log(|_| None, |e| Some(e.to_string()), &err).await
            }
        }
        (_, Some(fixtures_id), _) => {
            info!("Try get one lending info with fixtures_id[{fixtures_id}]");
            let uuid_opt = Uuid::parse_str(fixtures_id).ok();
            if let Some(uuid) = uuid_opt {
                let res = get_one_lending(&*conn, IdType::FixturesId(uuid)).await;
                result_to_handler_with_log(
                    |_| {
                        Some(format!(
                            "Success get lending with fixtures_id[{fixtures_id}]"
                        ))
                    },
                    |e| Some(format!("{e} fixtures_id[{fixtures_id}]")),
                    &res,
                )
                .await
            } else {
                let err = Err(QrError::BrokenUuid(fixtures_id.to_string()));
                result_to_handler_with_log(|_| None, |e| Some(e.to_string()), &err).await
            }
        }
        (_, _, Some(qr_id)) => {
            info!("Try get one lending info with fixtures_qr_id[{qr_id}]");
            let res = get_one_lending(&*conn, IdType::QrId(qr_id.to_string())).await;
            result_to_handler_with_log(
                |_| Some(format!("Success get lending with fixtures_qr_id[{qr_id}]")),
                |e| Some(format!("{e} fixtures_qr_id[{qr_id}]")),
                &res,
            )
            .await
        }
        _ => {
            let err = Err(QrError::UrlQuery(
                "lending_id, fixtures_id, fixtures_qr_id".to_string(),
            ));
            result_to_handler_with_log(|_| None, |e| Some(e.to_string()), &err).await
        }
    }
}

pub async fn get_is_lending(
    query: HashMap<String, String>,
    conn: Arc<Pool<Postgres>>,
) -> ReturnData<bool> {
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
                    Ok(_) => result_to_handler_with_log(|_| None, |_| None, &Ok(true)).await,
                    Err(QrError::DatabaseNotFound(_)) => {
                        result_to_handler_with_log(|_| None, |_| None, &Ok(false)).await
                    }
                    Err(e) => result_to_handler_with_log(|_| None, |_| None, &Err(e)).await,
                }
            } else {
                let err = Err(QrError::BrokenUuid(lending_id.to_string()));
                result_to_handler_with_log(|_| None, |e| Some(e.to_string()), &err).await
            }
        }
        (_, Some(fixtures_id), _) => {
            let uuid_opt = Uuid::parse_str(fixtures_id).ok();
            if let Some(uuid) = uuid_opt {
                match get_one_lending(&*conn, IdType::FixturesId(uuid)).await {
                    Ok(_) => result_to_handler_with_log(|_| None, |_| None, &Ok(true)).await,
                    Err(QrError::DatabaseNotFound(_)) => {
                        result_to_handler_with_log(|_| None, |_| None, &Ok(false)).await
                    }
                    Err(e) => result_to_handler_with_log(|_| None, |_| None, &Err(e)).await,
                }
            } else {
                let err = Err(QrError::BrokenUuid(fixtures_id.to_string()));
                result_to_handler_with_log(|_| None, |e| Some(e.to_string()), &err).await
            }
        }
        (_, _, Some(qr_id)) => {
            match get_one_lending(&*conn, IdType::QrId(qr_id.to_string())).await {
                Ok(_) => result_to_handler_with_log(|_| None, |_| None, &Ok(true)).await,
                Err(QrError::DatabaseNotFound(_)) => {
                    result_to_handler_with_log(|_| None, |_| None, &Ok(false)).await
                }
                Err(e) => result_to_handler_with_log(|_| None, |_| None, &Err(e)).await,
            }
        }
        _ => {
            let err = Err(QrError::UrlQuery(
                "lending_id, fixtures_id, fixtures_qr_id".to_string(),
            ));
            result_to_handler_with_log(|_| None, |e| Some(e.to_string()), &err).await
        }
    }
}

pub async fn update_lending(
    bearer: Bearer,
    Json(lending): Json<Lending>,
    conn: Arc<Pool<Postgres>>,
) -> ReturnData<()> {
    let role = get_role(&*conn, bearer.token()).await;
    if Ok(Role::EquipmentManager) == role || Ok(Role::Administrator) == role {
        info!("Try update lending: {lending:?}");
        let res = crate::database::update_lending::update_lending(&*conn, lending.clone()).await;
        result_to_handler_with_log(
            |_| Some(format!("Success update lending[{}]", lending.id)),
            |e| Some(format!("{e} lending[{}]", lending.id)),
            &res,
        )
        .await
    } else {
        result_to_handler(&Err(QrError::Authorized)).await
    }
}
