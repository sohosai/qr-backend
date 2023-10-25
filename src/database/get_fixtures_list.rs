use crate::{
    error_handling::{QrError, Result},
    Fixtures, Stroge,
};
use uuid::Uuid;

/// 検索条件
/// クエリから上手く構築すると良い
#[derive(Debug, Clone)]
pub enum SelectInfo {
    Id(Uuid),
    QrId(String),
    Name(String),
    Description(String),
    Storage(Stroge),
    ParentId(String),
}

pub async fn get_fixtures_list<'a, E>(conn: E, select: SelectInfo) -> Result<Vec<Fixtures>>
where
    E: sqlx::Executor<'a, Database = sqlx::Postgres>,
{
    match select {
        SelectInfo::Id(id) => {
            let fixtures_lst =
                sqlx::query_as!(Fixtures, "SELECT * FROM fixtures WHERE id = $1", id)
                    .fetch_all(conn)
                    .await
                    .map_err(|_| QrError::DatabaseGet("fixtures".to_string()))?;
            Ok(fixtures_lst)
        }
        SelectInfo::QrId(id) => {
            let fixtures_lst =
                sqlx::query_as!(Fixtures, "SELECT * FROM fixtures WHERE qr_id = $1", id)
                    .fetch_all(conn)
                    .await
                    .map_err(|_| QrError::DatabaseGet("fixtures".to_string()))?;
            Ok(fixtures_lst)
        }
        SelectInfo::Name(name) => {
            let fixtures_lst =
                sqlx::query_as!(Fixtures, "SELECT * FROM fixtures WHERE name LIKE $1", name)
                    .fetch_all(conn)
                    .await
                    .map_err(|_| QrError::DatabaseGet("fixtures".to_string()))?;
            Ok(fixtures_lst)
        }
        SelectInfo::Description(text) => {
            let fixtures_lst = sqlx::query_as!(
                Fixtures,
                "SELECT * FROM fixtures WHERE description LIKE $1",
                text
            )
            .fetch_all(conn)
            .await
            .map_err(|_| QrError::DatabaseGet("fixtures".to_string()))?;
            Ok(fixtures_lst)
        }
        SelectInfo::Storage(storage) => {
            let fixtures_lst = sqlx::query_as!(
                Fixtures,
                "SELECT * FROM fixtures WHERE storage = $1",
                storage.to_string()
            )
            .fetch_all(conn)
            .await
            .map_err(|_| QrError::DatabaseGet("fixtures".to_string()))?;
            Ok(fixtures_lst)
        }
        SelectInfo::ParentId(id) => {
            let fixtures_lst =
                sqlx::query_as!(Fixtures, "SELECT * FROM fixtures WHERE parent_id = $1", id)
                    .fetch_all(conn)
                    .await
                    .map_err(|_| QrError::DatabaseGet("fixtures".to_string()))?;
            Ok(fixtures_lst)
        }
    }
}
