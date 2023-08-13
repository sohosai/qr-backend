use anyhow::{Context, Result};
use uuid::Uuid;

pub async fn insert_equipment<'a, E>(conn: E, uuid: Uuid) -> Result<()>
where
    E: sqlx::Executor<'a, Database = sqlx::Postgres>,
{
    sqlx::query!("DELETE FROM equipment WHERE id = $1", uuid)
        .execute(conn)
        .await
        .context("Failed to delete equipment")?;

    Ok(())
}
