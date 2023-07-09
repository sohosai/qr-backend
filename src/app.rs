use anyhow::{Context, Result};
use sqlx::{postgres::PgConnection, Connection};
use std::net::SocketAddr;
use warp::Filter;

/// サーバーの実体
/// データベースを起動してエントリポイントに応じて関数を呼び出す
pub async fn app(bind: SocketAddr) -> Result<()> {
    let database_url =
        std::env::var("DATABASE_URL").context("Environment variable not set: DATABASE_URL")?;

    // migrateファイルを適用
    let mut conn = PgConnection::connect(&database_url).await?;
    crate::database::migrate(&mut conn).await?;

    // とりあえずダミー
    warp::serve(warp::path("ping").map(|| String::from("ping")))
        .run(bind)
        .await;
    Ok(())
}
