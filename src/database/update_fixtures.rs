use crate::Fixtures;
use anyhow::{Context, Result};

pub async fn update_fixtures<'a, E>(conn: E, new_info: Fixtures) -> Result<()>
where
    E: sqlx::Executor<'a, Database = sqlx::Postgres>,
{
    let Fixtures {
        id,
        created_at,
        qr_id,
        qr_color,
        name,
        description,
        model_number,
        storage,
        usage,
        usage_season,
        note,
        parent_id,
    } = new_info;

    sqlx::query!(
        r#"
    UPDATE fixtures SET
        created_at=$2,
        qr_id=$3,
        qr_color=$4,
        name=$5,
        description=$6,
        model_number=$7,
        storage=$8,
        usage=$9,
        usage_season=$10,
        note=$11,
        parent_id=$12
    WHERE id=$1"#,
        id,
        created_at,
        qr_id,
        qr_color.to_string(),
        name,
        description,
        model_number,
        storage.to_string(),
        usage,
        usage_season,
        note,
        parent_id,
    )
    .execute(conn)
    .await
    .context("Failed to update to fixtures")?;

    Ok(())
}

#[cfg(test)]
mod tests {
    use crate::database::get_one_fixtures::get_one_fixtures;
    use crate::database::insert_fixtures::insert_fixtures;
    use crate::database::update_fixtures::update_fixtures;
    use crate::Fixtures;
    use sqlx::{pool::Pool, Postgres};
    use uuid::uuid;
    #[sqlx::test(migrations = "./migrations")]
    async fn test_update_fixtures(pool: Pool<Postgres>) {
        let uuid = uuid!("550e8400-e29b-41d4-a716-446655440000");
        let info: Fixtures = serde_json::from_value(serde_json::json!({
          "id": uuid,
          "qr_id": "test",
          "created_at": "2023-08-07 15:56:35 UTC",
          "qr_color":"red",
          "name":"テスト物品",
          "description":"テスト説明",
          "storage": "room101",
          "usage": "無い",
          "note": "DBを確認",
          "parent_id": "null"
        }))
        .unwrap();

        insert_fixtures(&pool, info).await.unwrap();

        let new_info: Fixtures = serde_json::from_value(serde_json::json!({
          "id": uuid,
          "qr_id": "test2",
          "created_at": "2023-08-07 15:56:35 UTC",
          "qr_color":"red",
          "name":"テスト物品",
          "description":"テスト説明",
          "storage": "room102",
          "usage": "無い",
          "note": "DBを確認",
          "parent_id": "null"
        }))
        .unwrap();

        update_fixtures(&pool, new_info).await.unwrap();

        let result = get_one_fixtures(&pool, uuid).await.unwrap().unwrap();
        assert_eq!(result.qr_id, "test2".to_string())
    }
}
