use chrono::{DateTime, Utc};

#[derive(Debug, Clone, sqlx::Type)]
#[sqlx(type_name = "user_role")]
#[sqlx(rename_all = "snake_case")]
pub enum UserRole {
    Administrator,
    EquipmentManager,
    General
}

#[derive(Debug, Clone, sqlx::FromRow)]
pub struct User {
  pub id: String,
  pub created_at: DateTime<Utc>,
  pub email: String,
  pub role: UserRole,
}