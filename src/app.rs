use anyhow::Result;
use axum::{
    extract::Query,
    http::Method,
    routing::{delete, get, post},
    Router,
};
use chrono::Utc;
use std::collections::HashMap;
use std::net::SocketAddr;
use std::sync::Arc;
use tower_http::cors::{Any, CorsLayer};
use uuid::Uuid;

/// コンテナの管理を行うエンドポイントの定義
pub mod container;
/// 物品情報の登録を行うエンドポイントの定義
pub mod fixtures;
/// 貸出情報の管理を行うエンドポイントの定義
pub mod lending;
/// 場所の管理を行うエンドポイントの定義
pub mod spot;

/// サーバーの実体
/// データベースを起動してエントリポイントに応じて関数を呼び出す
pub async fn app(bind: SocketAddr) -> Result<()> {
    let conn = Arc::new(crate::database::create_pool().await?);

    // migrateファイルを適用
    crate::database::migrate(&mut conn.acquire().await?).await?;

    // pathと関数の実体の紐づけ
    let app = Router::new()
        .route("/ping", get(ping))
        .route(
            "/insert_fixtures",
            post({
                let conn = Arc::clone(&conn);
                move |body| fixtures::insert_fixtures(body, conn)
            }),
        )
        .route(
            "/update_fixtures",
            post({
                let conn = Arc::clone(&conn);
                move |body| fixtures::update_fixtures(body, conn)
            }),
        )
        .route(
            "/delete_fixtures",
            delete({
                let conn = Arc::clone(&conn);
                move |query: Query<HashMap<String, String>>| {
                    let uuid_opt = query.0.get("id").and_then(|s| Uuid::parse_str(s).ok());
                    fixtures::delete_fixtures(uuid_opt, conn)
                }
            }),
        )
        .route(
            "/get_fixtures",
            get({
                let conn = Arc::clone(&conn);
                move |Query(query)| fixtures::get_fixtures(query, conn)
            }),
        )
        .route(
            "/insert_lending",
            post({
                let conn = Arc::clone(&conn);
                move |body| lending::insert_lending(body, conn)
            }),
        )
        .route(
            "/update_lending",
            post({
                let conn = Arc::clone(&conn);
                move |body| lending::update_lending(body, conn)
            }),
        )
        .route(
            "/returned_lending",
            post({
                let conn = Arc::clone(&conn);
                move |query: Query<HashMap<String, String>>| {
                    let uuid_opt = query.0.get("id").and_then(|s| Uuid::parse_str(s).ok());
                    let qr_id_opt = query.0.get("qr_id").cloned();
                    let now = Utc::now();
                    lending::returned_lending(uuid_opt, qr_id_opt, now, conn)
                }
            }),
        )
        .route(
            "/get_lending_list",
            get({
                let conn = Arc::clone(&conn);
                move || lending::get_lending_list(conn)
            }),
        )
        .route(
            "/get_lending",
            get({
                let conn = Arc::clone(&conn);
                move |Query(query)| lending::get_one_lending(query, conn)
            }),
        )
        .route(
            "/get_is_lending",
            get({
                let conn = Arc::clone(&conn);
                move |Query(query)| lending::get_is_lending(query, conn)
            }),
        )
        .route(
            "/get_fixtures_list",
            get({
                let conn = Arc::clone(&conn);
                move |Query(query)| fixtures::get_fixtures_list(query, conn)
            }),
        )
        .route(
            "/insert_spot",
            post({
                let conn = Arc::clone(&conn);
                move |body| spot::insert_spot(body, conn)
            }),
        )
        .route(
            "/update_spot",
            post({
                let conn = Arc::clone(&conn);
                move |body| spot::update_spot(body, conn)
            }),
        )
        .route(
            "/get_spot",
            get({
                let conn = Arc::clone(&conn);
                move |query: Query<HashMap<String, String>>| {
                    let name = query.0.get("name").cloned();
                    spot::get_one_spot(name, conn)
                }
            }),
        )
        .route(
            "/get_spot_list",
            get({
                let conn = Arc::clone(&conn);
                move || spot::get_spot_list(conn)
            }),
        )
        .route(
            "/insert_container",
            post({
                let conn = Arc::clone(&conn);
                move |body| container::insert_container(body, conn)
            }),
        )
        .layer(
            CorsLayer::new()
                .allow_methods([Method::GET, Method::POST])
                .allow_origin(Any),
        );

    // サーバーの実行
    axum::Server::bind(&bind)
        .serve(app.into_make_service())
        .await?;

    Ok(())
}

/// ダミー
pub async fn ping() -> &'static str {
    "pong"
}
