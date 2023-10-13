use crate::Lending;
use anyhow::{Context, Result};
use serde::{Deserialize, Serialize};
use uuid::Uuid;

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub enum IdType {
    LendingId(Uuid),
    FixturesId(Uuid),
    QrId(String),
}

pub async fn get_one_lending<'a, E>(conn: E, id: IdType) -> Result<Option<Lending>>
where
    E: sqlx::Executor<'a, Database = sqlx::Postgres>,
{
    match id {
        IdType::LendingId(id) => {
            let lending_opt = sqlx::query_as!(Lending, "SELECT * FROM lending WHERE id = $1", id)
                .fetch_optional(conn)
                .await
                .context("Failed to get lending")?;

            Ok(lending_opt)
        }
        IdType::FixturesId(id) => {
            let lending_opt =
                sqlx::query_as!(Lending, "SELECT * FROM lending WHERE fixtures_id = $1", id)
                    .fetch_optional(conn)
                    .await
                    .context("Failed to get lending")?;

            Ok(lending_opt)
        }
        IdType::QrId(id) => {
            let lending_opt = sqlx::query_as!(
                Lending,
                "SELECT * FROM lending WHERE fixtures_qr_id = $1",
                id
            )
            .fetch_optional(conn)
            .await
            .context("Failed to get lending")?;

            Ok(lending_opt)
        }
    }
}

#[cfg(test)]
mod tests {
    use crate::database::get_one_lending::{get_one_lending, IdType::*};
    use crate::database::insert_lending::insert_lending;
    use sqlx::{pool::Pool, Postgres};
    use uuid::uuid;

    #[sqlx::test(migrations = "./migrations")]
    async fn test_insert_spot_sql(pool: Pool<Postgres>) {
        let id = uuid!("550e8400-e29b-41d4-a716-446655440000");
        let fixtures_id = uuid!("550e8400-e29b-41d4-a716-446655440001");
        let dummy_fixtures_id = uuid!("550e8400-e29b-41d4-a716-446655440002");
        let info = serde_json::from_value(serde_json::json!({
          "id": id,
          "fixtures_id": fixtures_id,
          "fixtures_qr_id": "x234",
          "spot_name": "test",
          "lending_at": "2023-08-07 15:56:35 UTC",
          "borrower_name": "test",
          "borrower_number": 202200000,
          "borrower_org": "jsys"
        }))
        .unwrap();

        insert_lending(&pool, info).await.unwrap();

        let result = get_one_lending(&pool, LendingId(id)).await.unwrap();
        assert!(result.is_some());

        let result = get_one_lending(&pool, FixturesId(fixtures_id))
            .await
            .unwrap();
        assert!(result.is_some());

        let result = get_one_lending(&pool, QrId("x234".to_string()))
            .await
            .unwrap();
        assert!(result.is_some());

        let result = get_one_lending(&pool, FixturesId(dummy_fixtures_id))
            .await
            .unwrap();
        assert!(result.is_none());

        let result = get_one_lending(&pool, QrId("x235".to_string()))
            .await
            .unwrap();
        assert!(result.is_none());
    }
}