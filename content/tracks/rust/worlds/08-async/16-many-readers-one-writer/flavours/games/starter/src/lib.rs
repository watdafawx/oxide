use std::collections::HashMap;
use tokio::sync::RwLock;

/// Mod id -> installed version, shared by every download task: read often, written now and then.
pub struct VersionCache {
    inner: RwLock<HashMap<String, String>>,
}

impl VersionCache {
    pub fn new() -> VersionCache {
        VersionCache { inner: RwLock::new(HashMap::new()) }
    }

    /// Records the installed version of a mod (replaces an older one).
    pub async fn insert(&self, mod_id: &str, version: &str) {
        // TODO: take the write lock and insert owned copies of mod_id and version
        let _ = (mod_id, version);
        todo!()
    }

    /// The installed version of a mod, if any.
    pub async fn get(&self, mod_id: &str) -> Option<String> {
        // TODO: take the read lock and give back an owned copy of the version, if there is one
        let _ = mod_id;
        todo!()
    }

    /// How many mods are recorded.
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
        let cache = VersionCache::new();
        timeout(Duration::from_secs(2), async {
            assert_eq!(cache.get("sodium").await, None);
            cache.insert("sodium", "0.5.8").await;
            cache.insert("sodium", "0.6.1").await;
            assert_eq!(cache.get("sodium").await.as_deref(), Some("0.6.1"));
            assert_eq!(cache.count().await, 1);
        })
        .await
        .expect("timed out");
    }

    #[tokio::test(flavor = "multi_thread", worker_threads = 4)]
    async fn cache_is_shared_between_tasks() {
        let cache = Arc::new(VersionCache::new());
        let mut handles = Vec::new();
        for i in 0..8 {
            let cache = Arc::clone(&cache);
            handles.push(tokio::spawn(async move {
                cache.insert(&format!("mod{i}"), "x").await;
            }));
        }
        for h in handles {
            timeout(Duration::from_secs(2), h).await.expect("timed out").unwrap();
        }
        assert_eq!(cache.count().await, 8);
        assert_eq!(cache.get("mod3").await.as_deref(), Some("x"));
    }
}
