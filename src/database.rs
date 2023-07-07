//! Postgresを使う。
//! DBとRUstとの接続は[sqlx](https://github.com/launchbadge/sqlx)というライブラリを使う。
//!
//! cliツールもあり、これは
//! ```sh
//! cargo install sqlx-cli
//! ```
//! を行うことでインストールできる。
//!
//! PostgresのURLは`QR_DATABASE_URL`に設定する。
//!

use anyhow::{Context, Result};

/// migrationファイルを適用する
pub async fn migrate<'a, A>(conn: A) -> Result<()>
where
    A: sqlx::Acquire<'a, Database = sqlx::Postgres>,
{
    sqlx::migrate!("src/database/migrations")
        .run(conn)
        .await
        .context("Failed to run migrations")
}

/// 備品登録をする
pub async fn insert_equipment(_info: crate::Equipment) -> Result<()> {
    todo!()
}
