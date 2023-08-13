use anyhow::{Context, Result};

/// 備品登録をする
pub async fn insert_spot<'a, E>(conn: E, info: crate::Spot) -> Result<()>
where
    E: sqlx::Executor<'a, Database = sqlx::Postgres>,
{
    let crate::Spot {
        name,
        area,
        building,
        floor,
        room,
    } = info;

    sqlx::query!(
        r#"
    INSERT INTO spot (
      name,
      area,
      building,
      floor,
      room
    ) VALUES ( $1, $2, $3, $4, $5 )"#,
        name,
        area.to_string(),
        building,
        floor.map(|u8| u8 as i32),
        room
    )
    .execute(conn)
    .await
    .context("Failed to insert to equipment")?;

    Ok(())
}

#[cfg(test)]
mod tests {
    use crate::database::insert_spot::insert_spot;
    use crate::Area;
    use crate::Spot;
    use sqlx::{pool::Pool, Postgres};
    #[sqlx::test(migrations = "./migrations")]
    async fn test_insert_spot_sql(pool: Pool<Postgres>) {
        let info = Spot {
            name: "test".to_string(),
            area: Area::Area3,
            building: Some("3C".to_string()),
            floor: Some(2),
            room: Some("coinsラウンジ".to_string()),
        };
        let res = insert_spot(&pool, info).await;
        assert!(res.is_ok());
    }
}
