use crate::{
    error_handling::{QrError, Result},
    Spot,
};

/// 情報のアップデートを行う
pub async fn update_spot<'a, E>(conn: E, new_info: Spot) -> Result<()>
where
    E: sqlx::Executor<'a, Database = sqlx::Postgres>,
{
    let Spot {
        name,
        area,
        building,
        floor,
        room,
        note,
    } = new_info;
    sqlx::query!(
        r#"UPDATE spot SET area=$2, building=$3, floor=$4, room=$5, note=$6 WHERE name=$1"#,
        name,
        area.to_string(),
        building,
        floor.map(|i| i as i32),
        room,
        note
    )
    .execute(conn)
    .await
    .map_err(|_| QrError::DatabaseUpdate("spot".to_string()))?;

    Ok(())
}

#[cfg(test)]
mod tests {
    use crate::database::insert_spot::insert_spot;
    use crate::database::update_spot::update_spot;
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
            note: None,
        };
        let res = insert_spot(&pool, info).await;
        assert!(res.is_ok());

        let new_info = Spot {
            name: "test".to_string(),
            area: Area::Area3,
            building: Some("3C".to_string()),
            floor: Some(1),
            room: Some("coins計算機室".to_string()),
            note: None,
        };
        let res = update_spot(&pool, new_info).await;
        assert!(res.is_ok());
    }
}
