use chrono::{DateTime, Utc};

#[derive(Debug, Clone, sqlx::FromRow)]
pub struct Place {
    pub id: String,
    pub created_at: DateTime<Utc>,
}