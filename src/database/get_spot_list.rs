use crate::{
    error_handling::{QrError, Result},
    Spot,
};

pub async fn get_spot_list<'a, E>(conn: E) -> Result<Vec<Spot>>
where
    E: sqlx::Executor<'a, Database = sqlx::Postgres>,
{
    let spot_opt = sqlx::query_as!(Spot, "SELECT * FROM spot")
        .fetch_all(conn)
        .await
        .map_err(|_| QrError::DatabaseGet("spot".to_string()))?;

    Ok(spot_opt)
}

#[cfg(test)]
mod tests {
    use crate::database::get_spot_list::get_spot_list;
    use crate::database::insert_spot::insert_spot;
    use crate::Spot;
    use sqlx::{pool::Pool, Postgres};

    #[sqlx::test(migrations = "./migrations")]
    async fn test_insert_spot_sql(pool: Pool<Postgres>) {
        let info1: Spot = serde_json::from_value(serde_json::json!({
          "name": "test1",
          "area": "area3",
          "building": "3C棟",
        }))
        .unwrap();
        let info2: Spot = serde_json::from_value(serde_json::json!({
          "name": "test2",
          "area": "area3",
          "building": "3C棟",
        }))
        .unwrap();
        let info3: Spot = serde_json::from_value(serde_json::json!({
          "name": "test3",
          "area": "area3",
          "building": "3C棟",
        }))
        .unwrap();

        insert_spot(&pool, info1).await.unwrap();
        insert_spot(&pool, info2).await.unwrap();
        insert_spot(&pool, info3).await.unwrap();

        let result: Vec<Spot> = get_spot_list(&pool).await.unwrap();
        assert_eq!(result.len(), 3);
    }
}
