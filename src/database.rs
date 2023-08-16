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
/// 貸し出し中の物品の情報を取得する
pub mod get_lending_list;
/// 物品の取得を行う関数を提供する
pub mod get_one_fixtures;
/// 物品登録を行う関数を提供する
pub mod insert_fixtures;
/// 貸出情報の登録を行う関数を提供する
pub mod insert_lending;
/// 地点登録を行う関数を提供する
pub mod insert_spot;
/// 返却処理を行う関数を提供する
pub mod returned_lending;
/// 物品情報の更新をする関数を提供する
pub mod update_fixtures;
/// 貸出情報の更新を行う関数を提供する
pub mod update_lending;
/// 地点情報の変更を行う関数を提供する
pub mod update_spot;

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
