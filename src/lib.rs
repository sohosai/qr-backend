//! 資材管理システムqrのバックエンド
//!

use core::panic;

use chrono::{DateTime, Utc};
use serde::{Deserialize, Serialize};
use uuid::Uuid;

// とりあえず後で実装しそうなものをちょっとだけ用意しておく
/// サーバーの実体
pub mod app;
/// データベース周りのモジュール
pub mod database;
/// 検索エンジン周りのモジュール
pub mod search_engine;

/// 備品情報のデータ。
/// 必要な構成要素はこちらを参照<https://scrapbox.io/jsys/QR_2023_Design_Doc>
///
/// 具体的なデータはこれ:
/// <https://docs.google.com/spreadsheets/d/1PttDAxejyimvIQp-RKmAnYzVVEUaBb611Zgp4bUiO0I/edit#gid=0>
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq, sqlx::FromRow)]
pub struct Fixtures {
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
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq)]
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

impl From<std::string::String> for QrColor {
    fn from(item: String) -> Self {
        match item.as_str() {
            "red" => QrColor::Red,
            "orange" => QrColor::Orange,
            "brown" => QrColor::Brown,
            "light_blue" => QrColor::LightBlue,
            "blue" => QrColor::Blue,
            "green" => QrColor::Green,
            "yellow" => QrColor::Yellow,
            "purple" => QrColor::Purple,
            "pink" => QrColor::Pink,
            _ => panic!(),
        }
    }
}

/// 保管場所の教室等の情報
/// 本来はstorageを`CREATE TYPE storage AS ENUM`などの形で定義したい。
/// しかしsqlx v0.6以降できないらしく、DBにはtextで保存して変換をこちらで行うこととする。
/// そのため、文字列に変換する`Display`トレイトと文字列から変換する`FromStr`トレイトを実装している。
/// 参考：<https://github.com/launchbadge/sqlx/issues/1920>
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq)]
#[serde(rename_all = "snake_case")]
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

impl From<std::string::String> for Stroge {
    fn from(item: String) -> Self {
        match item.as_str() {
            "room101" => Stroge::Room101,
            "room102" => Stroge::Room102,
            "room206" => Stroge::Room206,
            _ => panic!(),
        }
    }
}

/// 貸し出した物品を持っていく地点などの情報
/// DBに保管して参照できるようにする
#[derive(Debug, Clone, Serialize, Deserialize, sqlx::FromRow)]
pub struct Spot {
    /// 人が聞いて認識できるような場所につけられた名前。
    /// 入力で使われることを想定。
    pub name: String,
    /// 第三エリアなどの大まかな範囲を与える。
    pub area: Area,
    /// 建物の名称。
    /// 建物ではないところで使う可能性もあるのでOption型。
    /// enumで新たにbuilding型を定義するべきかは迷い中。
    pub building: Option<String>,
    /// 階数。
    /// 建物ではないところで使う可能性もあるのでOption型。
    pub floor: Option<i32>,
    /// 部屋の番号や名前など。
    /// 建物ではないところで使う可能性もあるのでOption型。
    pub room: Option<String>,
}

/// 大まかな範囲を与える区分。
/// 学内の使われる範囲を細かすぎず網羅的にカバーできるべき。
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum Area {
    /// 第一エリア
    Area1,
    /// 第二エリア
    Area2,
    /// 第三エリア
    Area3,
    /// 中央図書館
    CenterLibrary,
    /// 石の広場
    IshiSquare,
    /// 医学エリア
    Igaku,
    /// 体育芸術エリア
    Taigei,
    /// 春日エリア
    Kasuga,
    /// 一の矢
    Ichinoya,
    /// 平砂
    Hirasuna,
    /// 追越
    Oikoshi,
}

impl std::fmt::Display for Area {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        match self {
            Area::Area1 => write!(f, "area1")?,
            Area::Area2 => write!(f, "area2")?,
            Area::Area3 => write!(f, "area3")?,
            Area::CenterLibrary => write!(f, "center_library")?,
            Area::IshiSquare => write!(f, "ishi_square")?,
            Area::Igaku => write!(f, "igaku")?,
            Area::Taigei => write!(f, "taigei")?,
            Area::Kasuga => write!(f, "kasuga")?,
            Area::Ichinoya => write!(f, "ichinoya")?,
            Area::Hirasuna => write!(f, "hirasuna")?,
            Area::Oikoshi => write!(f, "oikoshi")?,
        };
        Ok(())
    }
}

impl From<std::string::String> for Area {
    fn from(item: String) -> Self {
        match item.as_str() {
            "area1" => Area::Area1,
            "area2" => Area::Area2,
            "area3" => Area::Area3,
            "center_library" => Area::CenterLibrary,
            "ishi_square" => Area::IshiSquare,
            "igaku" => Area::Igaku,
            "taigei" => Area::Taigei,
            "kasuga" => Area::Taigei,
            "ichinoya" => Area::Ichinoya,
            "hirasuna" => Area::Hirasuna,
            "okioshi" => Area::Oikoshi,
            _ => panic!(),
        }
    }
}

/// 貸し出した物品を持っていく地点などの情報
/// DBに保管して参照できるようにする
#[derive(Debug, Clone, Serialize, Deserialize, sqlx::FromRow)]
pub struct Lending {
    /// 貸し出しに振る一意のID
    pub id: Uuid,
    /// 貸し出した物品のID
    pub fixtures_id: Uuid,
    /// 貸し出した物品のQR ID
    pub fixtures_qr_id: String,
    /// 貸し出して持っていく地点の名称
    /// Spot型のnameフィールドと一致する必要がある
    pub spot_name: String,
    /// 貸し出し日時の記録
    pub lending_at: DateTime<Utc>,
    /// 返却日時の記録
    /// これの値で貸し出し中かどうかも判定できる
    pub returned_at: Option<DateTime<Utc>>,
    /// 借りた人の名前
    pub borrower_name: String,
    /// 借りた人の学籍番号
    pub borrower_number: i32,
    /// 借りた人の所属組織
    pub borrower_org: Option<String>,
}

/// 物品を保管しているコンテナの情報
#[derive(Debug, Clone, Serialize, Deserialize, sqlx::FromRow)]
pub struct Container {
    /// コンテナに振る一意のID
    pub id: Uuid,
    /// QRコードに振られたID
    pub qr_id: String,
    /// QRコードに貼られた色
    pub qr_color: QrColor,
    /// 保管されている部屋
    pub storage: Stroge,
    /// 見た目や分類などを説明するテキスト
    pub description: String,
}
