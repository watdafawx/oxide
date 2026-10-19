use std::collections::HashMap;
use std::sync::Arc;
use tokio::sync::{Mutex, RwLock};
use tokio::task::yield_now;

/// Adds one to the steps-done counter. Read, give other tasks a chance to run, write back.
pub async fn record_step(steps: &Mutex<usize>) {
    // TODO: lock (tokio Mutex), read the value, yield_now().await, store value + 1
    // The guard must stay alive over the .await, or updates get lost.
    todo!()
}

/// Spawns `tasks` tasks. Each calls record_step `steps_each` times. Returns the final count.
pub async fn finish_downloads(tasks: usize, steps_each: usize) -> usize {
    // TODO: Arc<Mutex<usize>>, clone into each task, await every handle, read the result
    todo!()
}

/// Which version of each mod is installed: many readers, occasional writers.
#[derive(Default)]
pub struct VersionCache {
    inner: RwLock<HashMap<String, String>>,
}

impl VersionCache {
    pub fn new() -> Self {
        Self::default()
    }

    /// Records the installed version of a mod (replaces an older one).
    pub async fn insert(&self, mod_id: &str, version: &str) {
        // TODO: write lock, then insert
        todo!()
    }

    /// The installed version, if any.
    pub async fn get(&self, mod_id: &str) -> Option<String> {
        // TODO: read lock, then get + cloned
        todo!()
    }

    /// How many mods are recorded.
    pub async fn count(&self) -> usize {
        // TODO: read lock, then len
        todo!()
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use std::time::Duration;
    use tokio::time::timeout;

    #[tokio::test(flavor = "multi_thread", worker_threads = 4)]
    async fn counter_reaches_the_full_total() {
        let total = timeout(Duration::from_secs(5), finish_downloads(10, 20))
            .await
            .expect("timed out");
        assert_eq!(total, 200, "updates were lost: hold the lock over the whole read-modify-write");
    }

    #[tokio::test]
    async fn cache_insert_then_get() {
        let cache = VersionCache::new();
        timeout(Duration::from_secs(2), async {
            assert_eq!(cache.get("jei").await, None);
            cache.insert("jei", "1.21").await;
            cache.insert("jei", "1.21.1").await;
            assert_eq!(cache.get("jei").await.as_deref(), Some("1.21.1"));
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
                cache.insert(&format!("mod{i}"), "1.0").await;
            }));
        }
        for h in handles {
            timeout(Duration::from_secs(2), h).await.expect("timed out").unwrap();
        }
        assert_eq!(cache.count().await, 8);
        assert_eq!(cache.get("mod3").await.as_deref(), Some("1.0"));
    }
}
