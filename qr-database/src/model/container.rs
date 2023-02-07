use chrono::{DateTime, Utc};

#[derive(Debug, Clone, sqlx::FromRow)]
pub struct Container {
    pub id: String,
    pub created_at: DateTime<Utc>,
    pub stored_place_id: String
}