use anyhow::{Context, Result};
use chrono::{DateTime, Utc};
use uuid::Uuid;

/// 返却を行う
pub async fn returned_lending<'a, E>(conn: E, id: Uuid, returned_at: DateTime<Utc>) -> Result<()>
where
    E: sqlx::Executor<'a, Database = sqlx::Postgres>,
{
    sqlx::query!(
        r#"
    UPDATE lending SET returned_at=$1 WHERE id=$2"#,
        returned_at,
        id
    )
    .execute(conn)
    .await
    .context("Failed to returned")?;

    Ok(())
}

#[cfg(test)]
mod tests {
    use crate::database::get_one_lending::{get_one_lending, IdType};
    use crate::database::insert_lending::insert_lending;
    use crate::database::returned_lending::returned_lending;
    use chrono::Utc;
    use sqlx::{pool::Pool, Postgres};
    use uuid::uuid;
    #[sqlx::test(migrations = "./migrations")]
    async fn test_returned_lending(pool: Pool<Postgres>) {
        let id = uuid!("550e8400-e29b-41d4-a716-446655440000");
        let fixtures_id = uuid!("550e8400-e29b-41d4-a716-446655440001");
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
        let res = insert_lending(&pool, info).await;
        assert!(res.is_ok());

        let returned_at = Utc::now();
        let res = returned_lending(&pool, id, returned_at).await;
        assert!(res.is_ok());

        let res = get_one_lending(&pool, IdType::FixturesId(fixtures_id)).await;
        assert!(res.unwrap().is_none());
    }
}
