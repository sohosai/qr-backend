use anyhow::{Context, Result};
use axum::{
    extract::Json,
    http::StatusCode,
    routing::{get, post},
    Router,
};
use sqlx::{pool::Pool, postgres::PgPool, postgres::Postgres};
use std::net::SocketAddr;
use std::sync::Arc;

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
            move |body| insert_equipment(body, conn)
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

/// 備品情報の登録を行うエンドポイント
/// - https://github.com/sohosai/qr-backend/issues/11
pub async fn insert_equipment(
    Json(equipment): Json<crate::Equipment>,
    conn: Arc<Pool<Postgres>>,
) -> StatusCode {
    match crate::database::insert_equipment::insert_equipment(&*conn, equipment).await {
        Ok(()) => StatusCode::ACCEPTED,
        _ => StatusCode::BAD_REQUEST,
    }
}
