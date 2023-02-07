use chrono::{DateTime, Utc};

#[derive(Debug, Clone, sqlx::FromRow)]
pub struct Object {
    pub id: String,
    pub created_at: DateTime<Utc>,
    pub equipment_id: String,
    pub qrcode_id: String,
    pub owner_id: String,
    pub parent_object_id: Option<String>,
    pub place_id: String,
    pub note: String,
}
