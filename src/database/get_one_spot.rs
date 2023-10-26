use crate::{
    error_handling::{QrError, Result},
    Spot,
};

pub async fn get_one_spot<'a, E>(conn: E, name: &str) -> Result<Spot>
where
    E: sqlx::Executor<'a, Database = sqlx::Postgres>,
{
    let spot_opt = sqlx::query_as!(Spot, "SELECT * FROM spot WHERE name = $1", name)
        .fetch_optional(conn)
        .await
        .map_err(|_| QrError::DatabaseGet("spot".to_string()))?;
    if let Some(spot) = spot_opt {
        Ok(spot)
    } else {
        Err(QrError::DatabaseNotFound(name.to_string()))
    }
}

#[cfg(test)]
mod tests {
    use crate::database::get_one_spot::get_one_spot;
    use crate::database::insert_spot::insert_spot;
    use crate::Spot;
    use sqlx::{pool::Pool, Postgres};

    #[sqlx::test(migrations = "./migrations")]
    async fn test_insert_spot_sql(pool: Pool<Postgres>) {
        let info: Spot = serde_json::from_value(serde_json::json!({
          "name": "test1",
          "area": "area3",
          "building": "3C棟",
          "note": null,
        }))
        .unwrap();

        insert_spot(&pool, info).await.unwrap();
        let result = get_one_spot(&pool, "test1").await;
        assert!(result.is_ok());

        let result = get_one_spot(&pool, "test2").await;
        assert!(result.is_err());
    }
}
