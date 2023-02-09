use crate::config::Config;
use anyhow::Result;
use chrono::{DateTime, Utc};
use qr_context::{TimeContext, UserRepository};
use qr_database::{command, model, query};
use qr_domain::{
    email::EmailAddress,
    role::Role,
    user::{User, UserId, UserName},
};
use sqlx::postgres::{PgPool, PgPoolOptions};
use std::convert::TryInto;

mod authentication;
pub use authentication::Authentication;

#[derive(Debug, Clone)]
pub struct App {
    pub pool: PgPool,
    pub config: Config,
}

impl App {
    pub async fn new(config: Config) -> Result<Self> {
        let pool = PgPoolOptions::new()
            .max_connections(config.max_database_connections)
            .connect(&config.postgres_uri)
            .await?;
        Ok(App { pool, config })
    }
}

#[async_trait::async_trait]
impl UserRepository for App {
    async fn create_user(&self, user: User) -> Result<()> {
        let User {
            id,
            created_at,
            name,
            email,
            role,
        } = user;
        let UserName { first, last } = name;
        let user = model::user::User {
            id: id.into(),
            created_at,
            first_name: first.into(),
            last_name: last.into(),
            email: email.into(),
            role: match role {
                Role::Administrator => model::user::UserRole::Administrator,
                Role::EquipmentManager => model::user::UserRole::EquipmentManager,
                Role::General => model::user::UserRole::General,
            },
        };
        command::insert_user(&self.pool, user).await
    }
}
