use anyhow::{Context, Result};
use axum::{
    routing::{get, post},
    Router,
};
use sqlx::postgres::PgPool;
use std::net::SocketAddr;
use std::sync::Arc;

/// 物品情報の登録を行うエンドポイントの定義
pub mod equipment;

/// サーバーの実体
/// データベースを起動してエントリポイントに応じて関数を呼び出す
pub async fn app(bind: SocketAddr) -> Result<()> {
    let database_url =
        std::env::var("DATABASE_URL").context("Environment variable not set: DATABASE_URL")?;

    // migrateファイルを適用
    let conn = Arc::new(PgPool::connect(&database_url).await?);
    crate::database::migrate(&mut conn.acquire().await?).await?;

    // pathと関数の実体の紐づけ
    let app = Router::new().route("/ping", get(ping)).route(
        "/insert_equipment",
        post({
            let conn = Arc::clone(&conn);
            move |body| equipment::insert_equipment(body, conn)
        }),
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
