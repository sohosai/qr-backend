use crate::Fixtures;
use anyhow::Result;
use meilisearch_sdk::client::*;
use serde::{de::DeserializeOwned, Deserialize, Serialize};
use std::env;
use uuid::Uuid;

/// 抽象化された検索コンテキスト
#[derive(Clone)]
pub struct Context {
    client: Client,
    index: String,
    primary_key: String,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct SearchResult<T> {
    pub data: T,
    pub ranking: Option<f64>,
}

impl Context {
    /// コンテキストを新しく作成
    pub fn new(index: &str, primary_key: &str) -> Self {
        let master_key = env::var("MEILI_MASTER_KEY").expect("MEILI_MASTER_KEY is not defined");
        let url = env::var("MEILI_URL").expect("MEILI_URL is not defined");
        Context {
            client: Client::new(url, Some(master_key)),
            index: index.to_string(),
            primary_key: primary_key.to_string(),
        }
    }

    /// 情報を追加または更新
    pub async fn add_or_replace_documents<T: Serialize>(&self, documents: &[T]) -> Result<()> {
        let client = &self.client;
        let index = client.index(&self.index);
        let task = index
            .add_documents(documents, Some(&self.primary_key))
            .await?;
        client.wait_for_task(task, None, None).await?;
        Ok(())
    }

    /// 削除する
    pub async fn delete_documents<T>(&self, keys: &[T]) -> Result<()>
    where
        T: std::fmt::Display + Serialize + std::fmt::Debug,
    {
        let client = &self.client;
        let index = client.index(&self.index);
        let task = index.delete_documents(keys).await?;
        client.wait_for_task(task, None, None).await?;
        Ok(())
    }

    /// 単語に対して検索をし、結果とランキングスコアのペアを返す
    pub async fn search<T: DeserializeOwned + 'static + Clone>(
        &self,
        keyword: &str,
    ) -> Result<Vec<SearchResult<T>>> {
        let index = self.client.index(&self.index);
        let list = index
            .search()
            .with_query(keyword)
            .with_limit(1000)
            .execute::<T>()
            .await?
            .hits
            .iter()
            .map(|res| SearchResult {
                data: res.result.clone(),
                ranking: res.ranking_score,
            })
            .collect::<Vec<_>>();
        Ok(list)
    }
}

/// 物品情報についての検索コンテキストなど
pub struct SearchFixtures {
    context: Context,
}

impl SearchFixtures {
    pub fn new() -> Self {
        SearchFixtures {
            context: Context::new("index", "id"),
        }
    }
    pub async fn add_or_replace(&self, lst: &[Fixtures]) -> Result<()> {
        self.context.add_or_replace_documents(lst).await
    }

    /// 削除する
    pub async fn delete(&self, keys: &[Uuid]) -> Result<()> {
        self.context.delete_documents(keys).await
    }

    /// 複数の単語について検索
    /// くっつけて重複削除
    pub async fn search(&self, keywords: &[String]) -> Result<Vec<SearchResult<Fixtures>>> {
        let mut lst = Vec::new();
        for keyword in keywords.iter() {
            let mut res = self.context.search::<Fixtures>(keyword).await?;
            lst.append(&mut res);
        }
        lst.sort_by_key(|f| f.data.id);
        lst.dedup_by_key(|f| f.data.id);
        Ok(lst)
    }
}
