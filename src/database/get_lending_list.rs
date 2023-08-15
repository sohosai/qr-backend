use crate::Lending;
use anyhow::{Context, Result};

/// 今貸し出されている物品の一覧を取得
pub async fn get_lending_list<'a, E>(conn: E) -> Result<Vec<Lending>>
where
    E: sqlx::Executor<'a, Database = sqlx::Postgres>,
{
    let list = sqlx::query_as!(Lending, "SELECT * FROM lending WHERE returned_at IS NULL")
        .fetch_all(conn)
        .await
        .context("Failed to get lending")?;

    Ok(list)
}

#[cfg(test)]
mod tests {
    use crate::database::get_lending_list::get_lending_list;
    use crate::database::insert_lending::insert_lending;
    use crate::database::returned_lending::returned_lending;
    use crate::Lending;
    use chrono::Utc;
    use sqlx::{pool::Pool, Postgres};
    use uuid::uuid;

    #[sqlx::test(migrations = "./migrations")]
    async fn test_insert_spot_sql(pool: Pool<Postgres>) {
        let info: Lending = serde_json::from_value(serde_json::json!({
          "id": "550e8400-e29b-41d4-a716-446655440000",
          "fixtures_id":  "550e8400-e29b-41d4-a716-446655440001",
          "spot_name": "test1",
          "lending_at": "2023-08-07 15:56:35 UTC",
          "borrower_name": "test",
          "borrower_number": 202200000,
          "borrower_org": "jsys"
        }))
        .unwrap();
        insert_lending(&pool, info).await.unwrap();

        let info: Lending = serde_json::from_value(serde_json::json!({
          "id": "550e8400-e29b-41d4-a716-446655440002",
          "fixtures_id":  "550e8400-e29b-41d4-a716-446655440003",
          "spot_name": "test2",
          "lending_at": "2023-08-07 15:56:35 UTC",
          "borrower_name": "test",
          "borrower_number": 202200000,
          "borrower_org": "jsys"
        }))
        .unwrap();
        insert_lending(&pool, info).await.unwrap();

        let info: Lending = serde_json::from_value(serde_json::json!({
          "id": "550e8400-e29b-41d4-a716-446655440004",
          "fixtures_id":  "550e8400-e29b-41d4-a716-446655440004",
          "spot_name": "test3",
          "lending_at": "2023-08-07 15:56:35 UTC",
          "borrower_name": "test",
          "borrower_number": 202200000,
          "borrower_org": "jsys"
        }))
        .unwrap();
        insert_lending(&pool, info).await.unwrap();

        let result = get_lending_list(&pool).await.unwrap();
        assert_eq!(result.len(), 3);

        returned_lending(
            &pool,
            uuid!("550e8400-e29b-41d4-a716-446655440004"),
            Utc::now(),
        )
        .await
        .unwrap();

        let result = get_lending_list(&pool).await.unwrap();
        assert_eq!(result.len(), 2);
    }
}
