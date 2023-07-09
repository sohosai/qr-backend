//! 資材管理システムqrのバックエンド
//!

use chrono::{serde::ts_milliseconds, Utc};
use serde::{Deserialize, Serialize};
use uuid::Uuid;

// とりあえず後で実装しそうなものをちょっとだけ用意しておく
/// サーバーの実体
pub mod app;
/// データベース周りのモジュール
pub mod database;

#[derive(Debug, Clone, Copy, Serialize, Deserialize)]
#[serde(transparent)]
pub struct DateTime(#[serde(with = "ts_milliseconds")] pub chrono::DateTime<Utc>);

impl DateTime {
    pub fn from_utc(utc: chrono::DateTime<Utc>) -> Self {
        DateTime(utc)
    }

    pub fn into_utc(self) -> chrono::DateTime<Utc> {
        self.0
    }
}

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
    pub created_at: DateTime,
    /// 貼られているQRコードに対応するID
    /// QRコードの更新でここの値が変わることがありうる
    pub qr_id: String,
    /// QRコードに貼られた色
    pub qr_color: QrColor,
    /// 物品名
    pub name: String,
    /// 説明
    pub descripiton: Option<String>,
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
#[derive(Debug, Clone, Serialize, Deserialize, sqlx::Type)]
#[serde(rename_all = "snake_case")]
#[sqlx(type_name = "qr_color", rename_all = "snake_case")]
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

/// 保管場所の教室等の情報
#[derive(Debug, Clone, Serialize, Deserialize, sqlx::Type)]
#[serde(rename_all = "snake_case")]
#[sqlx(type_name = "stroge", rename_all = "snake_case")]
pub enum Stroge {
    /// 101という部屋
    Room101,
    /// 102という部屋
    Room102,
    /// 206という教室
    Room206,
}
