//! Postgresを使う。
//! DBとRUstとの接続は[sqlx](https://github.com/launchbadge/sqlx)というライブラリを使う。
//!
//! cliツールもあり、これは
//! ```sh
//! cargo install sqlx-cli
//! ```
//! を行うことでインストールできる。
//!
//! PostgresのURLは`DATABASE_URL`に設定する。
//!

use anyhow::{Context, Result};
use sqlx::{pool::Pool, postgres::PgPool, Postgres};

/// 物品削除を行う関数を提供する
pub mod delete_fixtures;
/// 物品登録を行う関数を提供する
pub mod insert_fixtures;
/// 地点登録を行う関数を提供する
pub mod insert_spot;

/// migrationファイルを適用する
pub async fn migrate<'a, A>(conn: A) -> Result<()>
where
    A: sqlx::Acquire<'a, Database = sqlx::Postgres>,
{
    // migrationsフォルダにあるsqlファイルを全て実行する
    sqlx::migrate!("./migrations")
        .run(conn)
        .await
        .context("Failed to run migrations")
}

/// poolを生成する
pub async fn create_pool() -> Result<Pool<Postgres>> {
    let database_url =
        std::env::var("DATABASE_URL").context("Environment variable not set: DATABASE_URL")?;
    let conn = PgPool::connect(&database_url).await?;

    Ok(conn)
}
