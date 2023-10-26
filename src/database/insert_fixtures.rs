use crate::{
    error_handling::{QrError, Result},
    Fixtures,
};

/// 備品登録をする
pub async fn insert_fixtures<'a, E>(conn: E, info: Fixtures) -> Result<()>
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
    } = info;

    sqlx::query!(
        r#"
    INSERT INTO fixtures (
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
        parent_id
    ) VALUES ( $1, $2, $3, $4, $5, $6, $7, $8, $9, $10, $11, $12 )"#,
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
    .map_err(|_| QrError::DatabaseAdd("fixtures".to_string()))?;

    Ok(())
}
