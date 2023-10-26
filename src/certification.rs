use crate::error_handling::{QrError, Result};
use chrono::{DateTime, Duration, Utc};
use rand::{
    distributions::{Alphanumeric, DistString},
    Rng,
};
use serde::{Deserialize, Serialize};
use std::env;
use uuid::Uuid;

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq, sqlx::FromRow)]
pub struct Passtoken {
    /// 認証用の一時トークン
    pub token: String,
    /// トークンに付与された権限
    pub role: Role,
    /// 生成された日時
    pub created_at: DateTime<Utc>,
    /// トークンの期限
    pub limit_days: i32,
}

/// トークンに与えられる権限情報
/// 本来は`CREATE TYPE role AS ENUM`などの形で定義したい。
/// しかしsqlx v0.6以降できないらしく、DBにはtextで保存して変換をこちらで行うこととする。
/// そのため、文字列に変換する`Display`トレイトと文字列から変換する`FromStr`トレイトを実装している。
/// 参考：<https://github.com/launchbadge/sqlx/issues/1920>
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq)]
#[serde(rename_all = "snake_case")]
pub enum Role {
    /// 管理者権限 全ての操作ができる
    Administrator,
    /// 物品の登録や貸出、情報の更新、閲覧ができる
    /// 物品の廃棄などの戻すことのできない破壊的な変更は許されない
    EquipmentManager,
    /// 一般ユーザー
    /// - 物品情報の閲覧
    /// - 貸し出し情報の閲覧
    /// などの情報の閲覧のみ可能
    General,
}

impl std::fmt::Display for Role {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        match self {
            Role::Administrator => write!(f, "administrator")?,
            Role::EquipmentManager => write!(f, "equipment_manager")?,
            Role::General => write!(f, "general")?,
        };
        Ok(())
    }
}

impl From<std::string::String> for Role {
    fn from(item: String) -> Self {
        match item.as_str() {
            "administrator" => Role::Administrator,
            "equipment_manager" => Role::EquipmentManager,
            "general" => Role::General,
            _ => panic!("Undefined role: {item}"),
        }
    }
}

impl Passtoken {
    pub fn new(role: Role, limit: usize) -> Self {
        let now = Utc::now();
        let token = gen_token();
        Passtoken {
            token,
            role,
            created_at: now,
            limit_days: limit as i32,
        }
    }
    /// 有効期間内かどうかを検査する
    pub fn check_valid(&self) -> bool {
        let now = Utc::now();
        let d = self.created_at + Duration::days(self.limit_days as i64);
        // トークンの有効期限が現在時刻より大きければ有効
        d > now
    }
}

/// Bearer認証用のトークンをランダムに生成する
fn gen_token() -> String {
    let mut rng = rand::thread_rng();
    let token_length: usize = rng.gen_range(200..300);
    let uuid = Uuid::new_v4();
    uuid.to_string() + &Alphanumeric.sample_string(&mut rng, token_length)
}

/// 環境変数にあるマスターキーや有効期間などをもとに生成する
pub fn gen_passtoken(role: Role, key: &str) -> Result<Passtoken> {
    match role {
        Role::Administrator => {
            let pass = env::var("ADMINISTRATOR_PASS_KEY")
                .map_err(|_| QrError::Environment("ADMINISTRATOR_PASS_KEY".to_string()))?;
            let limit_days = env::var("ADMINISTRATOR_LIMIT_DAYS")
                .map_err(|_| QrError::Environment("ADMINISTRATOR_LIMIT_DAYS".to_string()))?
                .parse::<usize>()
                .map_err(|_| QrError::Environment("ADMINISTRATOR_LIMIT_DAYS".to_string()))?;
            if key == pass {
                Ok(Passtoken::new(role, limit_days))
            } else {
                Err(QrError::Authorized)
            }
        }
        Role::EquipmentManager => {
            let pass = env::var("EQUIPMENT_MANAGER_PASS_KEY")
                .map_err(|_| QrError::Environment("EQUIPMENT_MANAGER_PASS_KEY".to_string()))?;
            let limit_days = env::var("EQUIPMENT_MANAGER_LIMIT_DAYS")
                .map_err(|_| QrError::Environment("EQUIPMENT_MANAGER_LIMIT_DAYS".to_string()))?
                .parse::<usize>()
                .map_err(|_| QrError::Environment("EQUIPMENT_MANAGER_LIMIT_DAYS".to_string()))?;
            if key == pass {
                Ok(Passtoken::new(role, limit_days))
            } else {
                Err(QrError::Authorized)
            }
        }
        Role::General => {
            let pass = env::var("GENERAL_PASS_KEY")
                .map_err(|_| QrError::Environment("GENERAL_PASS_KEY".to_string()))?;
            let limit_days = env::var("GENERAL_LIMIT_DAYS")
                .map_err(|_| QrError::Environment("GENERAL_LIMIT_DAYS".to_string()))?
                .parse::<usize>()
                .map_err(|_| QrError::Environment("GENERAL_LIMIT_DAYS".to_string()))?;
            if key == pass {
                Ok(Passtoken::new(role, limit_days))
            } else {
                Err(QrError::Authorized)
            }
        }
    }
}

pub async fn insert_passtoken<'a, E>(conn: E, passtoken: Passtoken) -> Result<()>
where
    E: sqlx::Executor<'a, Database = sqlx::Postgres>,
{
    let Passtoken {
        token,
        role,
        created_at,
        limit_days,
    } = passtoken;

    sqlx::query!(
        r#"
    INSERT INTO passtoken (
        token,
        role,
        created_at,
        limit_days
    ) VALUES ( $1, $2, $3, $4 )"#,
        token,
        role.to_string(),
        created_at,
        limit_days
    )
    .execute(conn)
    .await
    .map_err(|_| QrError::DatabaseAdd("passtoken".to_string()))?;

    Ok(())
}

/// トークンを検査し、有効であればそれに結びついているロールを返す
pub async fn get_role<'a, E>(conn: E, token: &str) -> Result<Role>
where
    E: sqlx::Executor<'a, Database = sqlx::Postgres>,
{
    let passtoken_opt: Option<Passtoken> =
        sqlx::query_as!(Passtoken, "SELECT * FROM passtoken WHERE token = $1", token)
            .fetch_optional(conn)
            .await
            .map_err(|_| QrError::DatabaseGet("passtoken".to_string()))?;
    if let Some(passtoken) = passtoken_opt {
        if passtoken.check_valid() {
            Ok(passtoken.role)
        } else {
            Err(QrError::Authorized)
        }
    } else {
        Err(QrError::DatabaseNotFound(token.to_string()))
    }
}
