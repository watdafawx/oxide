use std::collections::HashMap;
use std::sync::Arc;
use tokio::sync::{Mutex, RwLock};
use tokio::task::yield_now;

/// Adds one to the counter. Read, give other tasks a chance to run, write back.
pub async fn bump(counter: &Mutex<usize>) {
    // TODO: lock (tokio Mutex), read the value, yield_now().await, store value + 1
    // The guard must stay alive over the .await, or updates get lost.
    todo!()
}

/// Spawns `tasks` tasks. Each calls bump `per_task` times. Returns the final count.
pub async fn count_concurrently(tasks: usize, per_task: usize) -> usize {
    // TODO: Arc<Mutex<usize>>, clone into each task, await every handle, read the result
    todo!()
}

/// A shared cache: many readers, occasional writers.
#[derive(Default)]
pub struct Cache {
    inner: RwLock<HashMap<String, String>>,
}

impl Cache {
    pub fn new() -> Self {
        Self::default()
    }

    pub async fn insert(&self, key: &str, value: &str) {
        // TODO: take the write lock and insert
        todo!()
    }

    pub async fn get(&self, key: &str) -> Option<String> {
        // TODO: take the read lock, return a clone of the value
        todo!()
    }

    pub async fn count(&self) -> usize {
        // TODO: read lock, number of entries
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
        let total = timeout(Duration::from_secs(5), count_concurrently(10, 20))
            .await
            .expect("timed out");
        assert_eq!(total, 200, "updates were lost: hold the lock over the whole read-modify-write");
    }

    #[tokio::test]
    async fn cache_insert_then_get() {
        let cache = Cache::new();
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
        let cache = Arc::new(Cache::new());
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
