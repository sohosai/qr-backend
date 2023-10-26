use crate::{
    error_handling::{QrError, Result},
    Lending,
};

/// 貸し出し情報のアップデートを行う
pub async fn update_lending<'a, E>(conn: E, new_info: Lending) -> Result<()>
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
    } = new_info;
    sqlx::query!(
        r#"UPDATE lending SET
            fixtures_id=$2,
            fixtures_qr_id=$3,
            spot_name=$4,
            lending_at=$5,
            returned_at=$6,
            borrower_name=$7,
            borrower_number=$8,
            borrower_org=$9
          WHERE id=$1"#,
        id,
        fixtures_id,
        fixtures_qr_id,
        spot_name,
        lending_at,
        returned_at,
        borrower_name,
        borrower_number,
        borrower_org
    )
    .execute(conn)
    .await
    .map_err(|_| QrError::DatabaseUpdate("lending".to_string()))?;

    Ok(())
}

#[cfg(test)]
mod tests {
    use crate::database::insert_lending::insert_lending;
    use crate::database::update_lending::update_lending;
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

        let new_info = serde_json::from_value(serde_json::json!({
          "id": id,
          "fixtures_id": fixtures_id,
          "fixtures_qr_id": "235",
          "spot_name": "test",
          "lending_at": "2023-08-16 15:56:35 UTC",
          "borrower_name": "test2",
          "borrower_number": 202200000,
          "borrower_org": "jsys"
        }))
        .unwrap();
        let res = update_lending(&pool, new_info).await;
        assert!(res.is_ok());
    }
}
