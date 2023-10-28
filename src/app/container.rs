use crate::authentication::{get_role, Role};
use crate::{
    error_handling::{result_to_handler, result_to_handler_with_log, QrError, ReturnData},
    Container,
};
use axum::{extract::Json, headers::authorization::Bearer};
use sqlx::{pool::Pool, postgres::Postgres};
use std::sync::Arc;
use tracing::*;

pub async fn insert_container(
    bearer: Bearer,
    Json(container): Json<Container>,
    conn: Arc<Pool<Postgres>>,
) -> ReturnData<()> {
    let role = get_role(&*conn, bearer.token()).await;
    if Ok(Role::EquipmentManager) == role || Ok(Role::Administrator) == role {
        info!("Try insert container: {container:?}");
        let res =
            crate::database::insert_container::insert_container(&*conn, container.clone()).await;
        result_to_handler_with_log(
            |_| Some(format!("Success insert container[{}]", &container.id)),
            |e| Some(format!("{e} [{}]", &container.id)),
            &res,
        )
        .await
    } else {
        result_to_handler(&Err(QrError::Authorized)).await
    }
}
