//! 資材管理システムqrのバックエンド
//!

use chrono::{DateTime, Utc};
use serde::{Deserialize, Serialize};
use uuid::Uuid;

// とりあえず後で実装しそうなものをちょっとだけ用意しておく
/// サーバーの実体
pub mod app;
/// データベース周りのモジュール
pub mod database;

/// 備品情報のデータ。
/// 必要な構成要素はこちらを参照<https://scrapbox.io/jsys/QR_2023_Design_Doc>
///
/// 具体的なデータはこれ:
/// <https://docs.google.com/spreadsheets/d/1PttDAxejyimvIQp-RKmAnYzVVEUaBb611Zgp4bUiO0I/edit#gid=0>
#[derive(Debug, Clone, Serialize, Deserialize, sqlx::FromRow)]
pub struct Equipment {
    /// 備品を識別する一意のID
    pub id: Uuid,
    /// 作成日時の記録
    pub created_at: DateTime<Utc>,
    /// 貼られているQRコードに対応するID
    /// QRコードの更新でここの値が変わることがありうる
    pub qr_id: String,
    /// QRコードに貼られた色
    pub qr_color: QrColor,
    /// 物品名
    pub name: String,
    /// 説明
    pub description: Option<String>,
    /// 型番
    pub model_number: Option<String>,
    /// 保管場所
    pub storage: Stroge,
    /// 使用用途
    pub usage: Option<String>,
    /// 使用時期（当日使うかどうか、など）
    pub usage_season: Option<String>,
    /// 備考
    pub note: String,
    /// 親物品ID
    /// 収納ケースにもIDを振っているので、これを参照する
    pub parent_id: String,
}

/// QRコードに貼られている色
/// 本来はqr_colorを`CREATE TYPE qr_color AS ENUM`などの形で定義したい。
/// しかしsqlx v0.6以降できないらしく、DBにはtextで保存して変換をこちらで行うこととする。
/// そのため、文字列に変換する`Display`トレイトと文字列から変換する`FromStr`トレイトを実装している。
/// 参考：<https://github.com/launchbadge/sqlx/issues/1920>
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum QrColor {
    Red,
    Orange,
    Brown,
    LightBlue,
    Blue,
    Green,
    Yellow,
    Purple,
    Pink,
}

impl std::fmt::Display for QrColor {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        match self {
            QrColor::Red => write!(f, "red")?,
            QrColor::Orange => write!(f, "orange")?,
            QrColor::Brown => write!(f, "brown")?,
            QrColor::LightBlue => write!(f, "light_blue")?,
            QrColor::Blue => write!(f, "blue")?,
            QrColor::Green => write!(f, "green")?,
            QrColor::Yellow => write!(f, "yellow")?,
            QrColor::Purple => write!(f, "purple")?,
            QrColor::Pink => write!(f, "pink")?,
        };
        Ok(())
    }
}

#[derive(Debug, PartialEq, Eq)]
pub struct ParseQrColorError;

impl std::str::FromStr for QrColor {
    type Err = ParseQrColorError;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        match s {
            "red" => Ok(QrColor::Red),
            "orange" => Ok(QrColor::Orange),
            "brown" => Ok(QrColor::Brown),
            "light_blue" => Ok(QrColor::LightBlue),
            "blue" => Ok(QrColor::Blue),
            "green" => Ok(QrColor::Green),
            "yellow" => Ok(QrColor::Yellow),
            "purple" => Ok(QrColor::Purple),
            "pink" => Ok(QrColor::Pink),
            _ => Err(ParseQrColorError),
        }
    }
}

/// 保管場所の教室等の情報
/// 本来はstorageを`CREATE TYPE storage AS ENUM`などの形で定義したい。
/// しかしsqlx v0.6以降できないらしく、DBにはtextで保存して変換をこちらで行うこととする。
/// そのため、文字列に変換する`Display`トレイトと文字列から変換する`FromStr`トレイトを実装している。
/// 参考：<https://github.com/launchbadge/sqlx/issues/1920>
#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum Stroge {
    /// 101という部屋
    Room101,
    /// 102という部屋
    Room102,
    /// 206という教室
    Room206,
}

impl std::fmt::Display for Stroge {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        match self {
            Stroge::Room101 => write!(f, "room101")?,
            Stroge::Room102 => write!(f, "room102")?,
            Stroge::Room206 => write!(f, "room206")?,
        };
        Ok(())
    }
}

#[derive(Debug, PartialEq, Eq)]
pub struct ParseStrogeError;

impl std::str::FromStr for Stroge {
    type Err = ParseStrogeError;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        match s {
            "room101" => Ok(Stroge::Room101),
            "room102" => Ok(Stroge::Room102),
            "room206" => Ok(Stroge::Room206),
            _ => Err(ParseStrogeError),
        }
    }
}
