use std::collections::HashMap;
use tokio::sync::RwLock;

/// Feed name -> title, shared by every task: read often, written now and then.
pub struct TitleCache {
    inner: RwLock<HashMap<String, String>>,
}

impl TitleCache {
    pub fn new() -> TitleCache {
        TitleCache { inner: RwLock::new(HashMap::new()) }
    }

    /// Records the title of a feed (replaces an older one).
    pub async fn insert(&self, feed: &str, title: &str) {
        // TODO: take the write lock and insert owned copies of feed and title
        let _ = (feed, title);
        todo!()
    }

    /// The title of a feed, if it is known.
    pub async fn get(&self, feed: &str) -> Option<String> {
        // TODO: take the read lock and give back an owned copy of the title, if there is one
        let _ = feed;
        todo!()
    }

    /// How many feeds are known.
    pub async fn count(&self) -> usize {
        // TODO: take the read lock and give back how many entries the map has
        todo!()
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use std::sync::Arc;
    use std::time::Duration;
    use tokio::time::timeout;

    #[tokio::test]
    async fn cache_insert_then_get() {
        let cache = TitleCache::new();
        timeout(Duration::from_secs(2), async {
            assert_eq!(cache.get("daily").await, None);
            cache.insert("daily", "The Daily").await;
            cache.insert("daily", "The Daily Show").await;
            assert_eq!(cache.get("daily").await.as_deref(), Some("The Daily Show"));
            assert_eq!(cache.count().await, 1);
        })
        .await
        .expect("timed out");
    }

    #[tokio::test(flavor = "multi_thread", worker_threads = 4)]
    async fn cache_is_shared_between_tasks() {
        let cache = Arc::new(TitleCache::new());
        let mut handles = Vec::new();
        for i in 0..8 {
            let cache = Arc::clone(&cache);
            handles.push(tokio::spawn(async move {
                cache.insert(&format!("feed{i}"), "x").await;
            }));
        }
        for h in handles {
            timeout(Duration::from_secs(2), h).await.expect("timed out").unwrap();
        }
        assert_eq!(cache.count().await, 8);
        assert_eq!(cache.get("feed3").await.as_deref(), Some("x"));
    }
}
