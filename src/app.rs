use anyhow::{Context, Result};
use sqlx::postgres::PgPool;
use std::net::SocketAddr;
use warp::Filter;

/// サーバーの実体
/// データベースを起動してエントリポイントに応じて関数を呼び出す
pub async fn app(bind: SocketAddr) -> Result<()> {
    let database_url =
        std::env::var("DATABASE_URL").context("Environment variable not set: DATABASE_URL")?;

    // migrateファイルを適用
    let conn = PgPool::connect(&database_url).await?;
    crate::database::migrate(&mut conn.acquire().await?).await?;
    warp::serve(ping().or(insert_equipment(&conn)))
        .run(bind)
        .await;
    Ok(())
}

/// ダミー
pub fn ping() -> impl Filter<Extract = impl warp::Reply, Error = warp::Rejection> + Clone {
    warp::path("ping")
        .and(warp::get())
        .map(|| String::from("ping"))
}

/// 備品情報の登録を行うエンドポイント
/// - https://github.com/sohosai/qr-backend/issues/11
pub fn insert_equipment<'a, E>(
    conn: E,
) -> impl Filter<Extract = impl warp::Reply, Error = warp::Rejection> + Clone + 'a
where
    E: sqlx::Executor<'a, Database = sqlx::Postgres> + Clone,
{
    warp::path("insert_equipment")
        .and(warp::post())
        .and(warp::body::json::<crate::Equipment>())
        .and_then(
            |equipment: crate::Equipment| async {
                if true {
                    Ok(String::new())
                } else {
                    Err(warp::reject::reject())
                }
            },
            // クロージャの外部にある`conn`を参照してしまっているので
            // ```
            // expected a closure that implements the `Fn` trait, but this closure only implements `FnOnce`
            // ```
            // というエラーが出てダメ
            // どうすれば良いのかわからず停止
            /*async {
                match crate::database::insert_equipment::insert_equipment(conn, equipment).await {
                    Ok(()) => Ok(String::new()),//Ok(warp::http::StatusCode::ACCEPTED),
                    _ => Err(warp::reject::reject()),
                }
            }*/
        )
}
