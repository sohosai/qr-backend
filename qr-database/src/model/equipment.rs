use chrono::{DateTime, Utc};

#[derive(Debug, Clone, sqlx::FromRow)]
pub struct Equipment {
    pub id: String,
    pub created_at: DateTime<Utc>,
    pub name: String,
}