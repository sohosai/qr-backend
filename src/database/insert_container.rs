use crate::{
    error_handling::{QrError, Result},
    Container,
};

/// 備品登録をする
pub async fn insert_container<'a, E>(conn: E, info: Container) -> Result<()>
where
    E: sqlx::Executor<'a, Database = sqlx::Postgres>,
{
    let Container {
        id,
        qr_id,
        qr_color,
        storage,
        description,
    } = info;

    sqlx::query!(
        r#"
    INSERT INTO container (
        id,
        qr_id,
        qr_color,
        storage,
        description
    ) VALUES ( $1, $2, $3, $4, $5 )"#,
        id,
        qr_id,
        qr_color.to_string(),
        storage.to_string(),
        description
    )
    .execute(conn)
    .await
    .map_err(|_| QrError::DatabaseAdd("container".to_string()))?;

    Ok(())
}

#[cfg(test)]
mod tests {
    use crate::database::insert_container::insert_container;
    use sqlx::{pool::Pool, Postgres};
    use uuid::uuid;
    #[sqlx::test(migrations = "./migrations")]
    async fn test_insert_container(pool: Pool<Postgres>) {
        let id = uuid!("550e8400-e29b-41d4-a716-446655440000");
        let info = serde_json::from_value(serde_json::json!({
          "id": id,
          "qr_id": "test",
          "qr_color": "red",
          "storage": "room101",
          "description": "test"
        }))
        .unwrap();
        let res = insert_container(&pool, info).await;
        assert!(res.is_ok());
    }
}
