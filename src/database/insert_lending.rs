use crate::Lending;
use anyhow::{Context, Result};

/// 備品登録をする
pub async fn insert_lending<'a, E>(conn: E, info: Lending) -> Result<()>
where
    E: sqlx::Executor<'a, Database = sqlx::Postgres>,
{
    let Lending {
        id,
        fixtures_id,
        fixtures_qr_id,
        spot_name,
        lending_at,
        returned_at,
        borrower_name,
        borrower_number,
        borrower_org,
    } = info;

    sqlx::query!(
        r#"
    INSERT INTO lending (
    id,
    fixtures_id,
    fixtures_qr_id,
    spot_name,
    lending_at,
    returned_at,
    borrower_name,
    borrower_number,
    borrower_org
    ) VALUES ( $1, $2, $3, $4, $5, $6, $7, $8, $9 )"#,
        id,
        fixtures_id,
        fixtures_qr_id,
        spot_name,
        lending_at,
        returned_at,
        borrower_name,
        borrower_number as i32,
        borrower_org
    )
    .execute(conn)
    .await
    .context("Failed to insert to fixtures")?;

    Ok(())
}

#[cfg(test)]
mod tests {
    use crate::database::insert_lending::insert_lending;
    use sqlx::{pool::Pool, Postgres};
    use uuid::uuid;
    #[sqlx::test(migrations = "./migrations")]
    async fn test_insert_spot_sql(pool: Pool<Postgres>) {
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
    }
}
