use crate::error_handling::{QrError, Result};

pub async fn delete_spot<'a, E>(conn: E, name: &str) -> Result<()>
where
    E: sqlx::Executor<'a, Database = sqlx::Postgres>,
{
    sqlx::query!("DELETE FROM spot WHERE name = $1", name)
        .execute(conn)
        .await
        .map_err(|_| QrError::DatabaseDelete("spot".to_string()))?;

    Ok(())
}

#[cfg(test)]
mod tests {
    use crate::database::delete_spot::delete_spot;
    use crate::database::get_one_spot::get_one_spot;
    use crate::database::insert_spot::insert_spot;
    use crate::Spot;
    use sqlx::{pool::Pool, Postgres};

    #[sqlx::test(migrations = "./migrations")]
    async fn test_insert_spot_sql(pool: Pool<Postgres>) {
        let name = "test1";
        let info: Spot = serde_json::from_value(serde_json::json!({
          "name": name,
          "area": "area1",
          "building": "3C"
        }))
        .unwrap();

        insert_spot(&pool, info).await.unwrap();
        let result = get_one_spot(&pool, name).await;
        assert!(result.is_ok());

        delete_spot(&pool, name).await.unwrap();
        let result = get_one_spot(&pool, name).await;
        assert!(result.is_err());
    }
}
