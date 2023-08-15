use crate::Fixtures;
use anyhow::{Context, Result};
use uuid::Uuid;

pub async fn get_one_fixtures<'a, E>(conn: E, uuid: Uuid) -> Result<Option<Fixtures>>
where
    E: sqlx::Executor<'a, Database = sqlx::Postgres>,
{
    let fixtures_opt = sqlx::query_as!(Fixtures, "SELECT * FROM fixtures WHERE id = $1", uuid)
        .fetch_optional(conn)
        .await
        .context("Failed to get fixtures")?;

    Ok(fixtures_opt)
}

#[cfg(test)]
mod tests {
    use crate::database::get_one_fixtures::get_one_fixtures;
    use crate::database::insert_fixtures::insert_fixtures;
    use crate::Fixtures;
    use sqlx::{pool::Pool, Postgres};
    use uuid::uuid;

    #[sqlx::test(migrations = "./migrations")]
    async fn test_insert_spot_sql(pool: Pool<Postgres>) {
        let uuid = uuid!("550e8400-e29b-41d4-a716-446655440000");
        let dummy_uuid = uuid!("550e8400-e29b-41d4-a716-446655440001");
        let info: Fixtures = serde_json::from_value(serde_json::json!({
          "id": uuid,
          "qr_id": "test",
          "created_at": "2023-08-07 15:56:35 UTC",
          "qr_color":"red",
          "name":"テスト物品",
          "description":"テスト説明",
          "storage": "Room101",
          "usage": "無い",
          "note": "DBを確認",
          "parent_id": "null"
        }))
        .unwrap();

        insert_fixtures(&pool, info).await.unwrap();
        let result: Option<Fixtures> = get_one_fixtures(&pool, uuid).await.unwrap();
        assert!(result.is_some());

        let result: Option<Fixtures> = get_one_fixtures(&pool, dummy_uuid).await.unwrap();
        assert!(result.is_none());
    }
}
