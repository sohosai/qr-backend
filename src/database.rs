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

/// 物品登録を行う関数を提供する
pub mod insert_equipment;

/// migrationファイルを適用する
pub async fn migrate<'a, A>(conn: A) -> Result<()>
where
    A: sqlx::Acquire<'a, Database = sqlx::Postgres>,
{
    sqlx::migrate!("./migrations")
        .run(conn)
        .await
        .context("Failed to run migrations")
}
