use crate::Fixtures;
use anyhow::Result;
use meilisearch_sdk::client::*;
use serde::{de::DeserializeOwned, Deserialize, Serialize};

/// 抽象化された検索コンテキスト
#[derive(Clone)]
pub struct Context {
    client: Client,
    index: String,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct SearchResult<T> {
    pub data: T,
    pub ranking: Option<f64>,
}

impl Context {
    /// コンテキストを新しく作成
    pub fn new(index: &str) -> Self {
        Context {
            client: Client::new("http://localhost:7700", Some("masterKey")),
            index: index.to_string(),
        }
    }

    /// 情報を追加する
    pub async fn add_documents<T: Serialize>(&self, documents: &[T]) -> Result<()> {
        let client = &self.client;
        let index = client.index(&self.index);
        let task = index.add_documents(documents, None).await?;
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
            context: Context::new("index"),
        }
    }
    pub async fn add(&self, lst: &[Fixtures]) -> Result<()> {
        self.context.add_documents(lst).await
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
