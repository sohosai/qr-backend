use crate::error_handling::{QrError, Result};
use axum::{
    extract::{Query, TypedHeader},
    headers::authorization::{Authorization, Basic, Bearer},
    http::Method,
    routing::{delete, get, post},
    Router,
};
use chrono::Utc;
use reqwest::header::{AUTHORIZATION, CONTENT_TYPE};
use std::collections::HashMap;
use std::net::SocketAddr;
use std::sync::Arc;
use tower_http::cors::{Any, CorsLayer};
use tracing::*;

use crate::search_engine;

/// 認証まわりのエンドポイントの定義
pub mod authentication;
/// コンテナの管理を行うエンドポイントの定義
pub mod container;
/// 物品情報の登録を行うエンドポイントの定義
pub mod fixtures;
/// 貸出情報の管理を行うエンドポイントの定義
pub mod lending;
/// 場所の管理を行うエンドポイントの定義
pub mod spot;

/// ログを出力するための設定など
async fn init_logger() -> Result<()> {
    let subscriber = tracing_subscriber::fmt()
        .with_max_level(tracing::Level::INFO)
        .finish();
    tracing::subscriber::set_global_default(subscriber).map_err(|_| QrError::LoggingConfig)?;
    Ok(())
}

/// サーバーの実体
/// データベースを起動してエントリポイントに応じて関数を呼び出す
pub async fn app(bind: SocketAddr) -> Result<()> {
    init_logger().await?;

    info!("Try generate DB connection pool");
    let conn = Arc::new(crate::database::create_pool().await?);
    info!("Success generate DB connection pool");

    info!("Try generate search engine context for fixtures");
    let search_fixtures_context = Arc::new(search_engine::SearchFixtures::new().await?);
    info!("Success generate search engine context for fixtures");

    // migrateファイルを適用
    crate::database::migrate(&mut conn.acquire().await.map_err(|_| QrError::ConnectionPool)?)
        .await?;

    // pathと関数の実体の紐づけ
    let app = Router::new()
        .route(
            "/ping",
            get({
                info!("GET /ping");
                ping
            }),
        )
        .route(
            "/insert_fixtures",
            post({
                info!("POST /insert_fixtures");
                let conn = Arc::clone(&conn);
                let context = Arc::clone(&search_fixtures_context);
                move |TypedHeader(Authorization(bearer)): TypedHeader<Authorization<Bearer>>,
                      body| {
                    fixtures::insert_fixtures(bearer, body, conn, context)
                }
            }),
        )
        .route(
            "/update_fixtures",
            post({
                info!("POST /update_fixtures");
                let conn = Arc::clone(&conn);
                let context = Arc::clone(&search_fixtures_context);
                move |TypedHeader(Authorization(bearer)): TypedHeader<Authorization<Bearer>>,
                      body| fixtures::update_fixtures(bearer, body, conn, context)
            }),
        )
        .route(
            "/delete_fixtures",
            delete({
                info!("DELETE /delete_fixtures");
                let conn = Arc::clone(&conn);
                let context = Arc::clone(&search_fixtures_context);
                move |TypedHeader(Authorization(bearer)): TypedHeader<Authorization<Bearer>>,
                      Query(query)| {
                    fixtures::delete_fixtures(bearer, query, conn, context)
                }
            }),
        )
        .route(
            "/get_fixtures",
            get({
                info!("GET /get_fixtures");
                let conn = Arc::clone(&conn);
                move |Query(query)| fixtures::get_fixtures(query, conn)
            }),
        )
        .route(
            "/search_fixtures",
            get({
                info!("GET /search_fixtures");
                let context = Arc::clone(&search_fixtures_context);
                move |query: Query<HashMap<String, String>>| {
                    let keywords_str = query
                        .0
                        .get("keywords")
                        .map(|s| s.to_string())
                        .unwrap_or_default();
                    fixtures::search_fixtures(keywords_str, context)
                }
            }),
        )
        .route(
            "/insert_lending",
            post({
                info!("POST /insert_lending");
                let conn = Arc::clone(&conn);
                move |TypedHeader(Authorization(bearer)): TypedHeader<Authorization<Bearer>>,
                      body| lending::insert_lending(bearer, body, conn)
            }),
        )
        .route(
            "/update_lending",
            post({
                info!("POST /update_lending");
                let conn = Arc::clone(&conn);
                move |TypedHeader(Authorization(bearer)): TypedHeader<Authorization<Bearer>>,
                      body| lending::update_lending(bearer, body, conn)
            }),
        )
        .route(
            "/returned_lending",
            post({
                info!("POST /returned_lending");
                let conn = Arc::clone(&conn);
                move |TypedHeader(Authorization(bearer)): TypedHeader<Authorization<Bearer>>,
                      Query(query)| {
                    let now = Utc::now();
                    lending::returned_lending(bearer, query, now, conn)
                }
            }),
        )
        .route(
            "/get_lending_list",
            get({
                info!("GET /get_lending_list");
                let conn = Arc::clone(&conn);
                move || lending::get_lending_list(conn)
            }),
        )
        .route(
            "/get_lending",
            get({
                info!("GET /get_lending");
                let conn = Arc::clone(&conn);
                move |Query(query)| lending::get_one_lending(query, conn)
            }),
        )
        .route(
            "/get_is_lending",
            get({
                info!("GET /get_is_lending");
                let conn = Arc::clone(&conn);
                move |Query(query)| lending::get_is_lending(query, conn)
            }),
        )
        .route(
            "/insert_spot",
            post({
                info!("POST /insert_spot");
                let conn = Arc::clone(&conn);
                move |TypedHeader(Authorization(bearer)): TypedHeader<Authorization<Bearer>>,
                      body| spot::insert_spot(bearer, body, conn)
            }),
        )
        .route(
            "/update_spot",
            post({
                info!("POST /update_spot");
                let conn = Arc::clone(&conn);
                move |TypedHeader(Authorization(bearer)): TypedHeader<Authorization<Bearer>>,
                      body| spot::update_spot(bearer, body, conn)
            }),
        )
        .route(
            "/get_spot",
            get({
                info!("GET /get_spot");
                let conn = Arc::clone(&conn);
                move |Query(query)| spot::get_one_spot(query, conn)
            }),
        )
        .route(
            "/get_spot_list",
            get({
                info!("GET /get_spot_list");
                let conn = Arc::clone(&conn);
                move || spot::get_spot_list(conn)
            }),
        )
        .route(
            "/delete_spot",
            delete({
                info!("DELETE /delete_spot");
                let conn = Arc::clone(&conn);
                move |TypedHeader(Authorization(bearer)): TypedHeader<Authorization<Bearer>>,
                      Query(query)| spot::delte_spot(bearer, query, conn)
            }),
        )
        .route(
            "/insert_container",
            post({
                info!("POST /insert_container");
                let conn = Arc::clone(&conn);
                move |TypedHeader(Authorization(bearer)): TypedHeader<Authorization<Bearer>>,
                      body| container::insert_container(bearer, body, conn)
            }),
        )
        .route(
            "/gen_passtoken",
            post({
                info!("POST /gen_passtoken");
                let conn = Arc::clone(&conn);
                move |TypedHeader(Authorization(basic)): TypedHeader<Authorization<Basic>>| {
                    authentication::api_gen_passtoken(basic, conn)
                }
            }),
        )
        .layer(
            CorsLayer::new()
                .allow_methods([Method::GET, Method::POST, Method::DELETE])
                .allow_headers([CONTENT_TYPE, AUTHORIZATION])
                .allow_origin(Any),
        );

    // サーバーの実行
    axum::Server::bind(&bind)
        .serve(app.into_make_service())
        .await
        .map_err(|_| QrError::Serve)?;

    Ok(())
}

/// ダミー
pub async fn ping() -> &'static str {
    "pong"
}
