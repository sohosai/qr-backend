use crate::{
    database::get_one_lending::*,
    error_handling::{QrError, Result},
    Lending,
};

/// 備品登録をする
pub async fn insert_lending<'a, E>(conn: E, info: Lending) -> Result<()>
where
    E: sqlx::Executor<'a, Database = sqlx::Postgres> + Clone,
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

    // 物品IDとQR IDを元に二重貸し出しにならないかを確認する
    let is_lending1 = get_one_lending(conn.clone(), IdType::FixturesId(fixtures_id))
        .await
        .is_ok();
    let is_lending2 = get_one_lending(conn.clone(), IdType::QrId(fixtures_qr_id.clone()))
        .await
        .is_ok();

    if !is_lending1 && !is_lending2 {
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
        .map_err(|_| QrError::DatabaseUpdate("lending".to_string()))?;
    } else {
        return Err(QrError::DatabaseUpdate("spot".to_string()));
    }
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
        let id2 = uuid!("550e8400-e29b-41d4-a716-446655440001");
        let fixtures_id = uuid!("550e8400-e29b-41d4-a716-446655440002");
        let fixtures_id2 = uuid!("550e8400-e29b-41d4-a716-446655440003");
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

        // 同じIDでもう一度やってみる
        let info = serde_json::from_value(serde_json::json!({
          "id": id,
          "fixtures_id": fixtures_id2,
          "fixtures_qr_id": "x235",
          "spot_name": "test",
          "lending_at": "2023-08-07 15:56:35 UTC",
          "borrower_name": "test",
          "borrower_number": 202200000,
          "borrower_org": "jsys"
        }))
        .unwrap();
        let res2 = insert_lending(&pool, info).await;
        assert!(res2.is_err());

        // 同じFixtures IDでもう一度やってみる
        let info = serde_json::from_value(serde_json::json!({
          "id": id2,
          "fixtures_id": fixtures_id,
          "fixtures_qr_id": "x236",
          "spot_name": "test",
          "lending_at": "2023-08-07 15:56:35 UTC",
          "borrower_name": "test",
          "borrower_number": 202200000,
          "borrower_org": "jsys"
        }))
        .unwrap();
        let res2 = insert_lending(&pool, info).await;
        assert!(res2.is_err());

        // IDとFixtures IDを変えるがQR IDは同じにしてみる
        let info = serde_json::from_value(serde_json::json!({
          "id": id2,
          "fixtures_id": fixtures_id2,
          "fixtures_qr_id": "x234",
          "spot_name": "test",
          "lending_at": "2023-08-07 15:56:35 UTC",
          "borrower_name": "test",
          "borrower_number": 202200000,
          "borrower_org": "jsys"
        }))
        .unwrap();
        let res2 = insert_lending(&pool, info).await;
        assert!(res2.is_err());

        // 全てのIDを変えてみる
        let info = serde_json::from_value(serde_json::json!({
          "id": id2,
          "fixtures_id": fixtures_id2,
          "fixtures_qr_id": "x237",
          "spot_name": "test",
          "lending_at": "2023-08-07 15:56:35 UTC",
          "borrower_name": "test",
          "borrower_number": 202200000,
          "borrower_org": "jsys"
        }))
        .unwrap();
        let res2 = insert_lending(&pool, info).await;
        assert!(res2.is_ok());
    }
}
