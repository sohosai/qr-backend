use anyhow::Result;
use axum::{
    extract::Query,
    routing::{get, post},
    Router,
};
use std::collections::HashMap;
use std::net::SocketAddr;
use std::sync::Arc;
use uuid::Uuid;

/// 物品情報の登録を行うエンドポイントの定義
pub mod equipment;

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
            "/insert_equipment",
            post({
                let conn = Arc::clone(&conn);
                move |body| equipment::insert_equipment(body, conn)
            }),
        )
        .route(
            "/delete_equipment",
            get({
                let conn = Arc::clone(&conn);
                move |query: Query<HashMap<String, String>>| {
                    let uuid_opt = match query.0.get("id") {
                        Some(uuid_str) => match Uuid::parse_str(uuid_str) {
                            Ok(uuid) => Some(uuid),
                            Err(_) => None,
                        },
                        None => None,
                    };
                    equipment::delete_equipment(uuid_opt, conn)
                }
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
