use anyhow::{Context, Result};

/// 備品登録をする
pub async fn insert_equipment<'a, E>(conn: E, info: crate::Equipment) -> Result<()>
where
    E: sqlx::Executor<'a, Database = sqlx::Postgres>,
{
    let crate::Equipment {
        id,
        created_at,
        qr_id,
        qr_color,
        name,
        descripiton,
        model_number,
        storage,
        usage,
        usage_season,
        note,
        parent_id,
    } = info;

    sqlx::query!(
        r#"
    INSERT INTO equipment (
        id,
        created_at,
        qr_id,
        qr_color,
        name,
        descripiton,
        model_number,
        storage,
        usage,
        usage_season,
        note,
        parent_id
    ) VALUES ( $1, $2, $3, $4, $5, $6, $7, $8, $9, $10, $11, $12 )
    "#,
        id,
        created_at,
        qr_id,
        qr_color,
        name,
        descripiton,
        model_number,
        storage,
        usage,
        usage_season,
        note,
        parent_id,
    )
    .execute(conn)
    .await
    .context("Failed to insert to equipment")?;

    Ok(())
}
