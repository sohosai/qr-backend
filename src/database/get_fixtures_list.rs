use crate::{
    error_handling::{QrError, Result},
    Fixtures,
};

pub async fn get_fixtures_list<'a, E>(conn: E) -> Result<Vec<Fixtures>>
where
    E: sqlx::Executor<'a, Database = sqlx::Postgres>,
{
    let fixtures_lst = sqlx::query_as!(Fixtures, "SELECT * FROM fixtures")
        .fetch_all(conn)
        .await
        .map_err(|_| QrError::DatabaseGet("fixtures".to_string()))?;
    Ok(fixtures_lst)
}
