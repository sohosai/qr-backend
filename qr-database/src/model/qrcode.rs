use chrono::{DateTime, Utc};

#[derive(Debug, Clone, sqlx::Type)]
#[sqlx(type_name = "user_role")]
#[sqlx(rename_all = "snake_case")]
pub enum QRCodeStatus {
    Printed,
    AwaitingPrinting,
    Destroyed,
}

#[derive(Debug, Clone, sqlx::FromRow)]
pub struct QRCode {
    pub id: String,
    pub created_at: DateTime<Utc>,
    pub status: QRCodeStatus,
}
