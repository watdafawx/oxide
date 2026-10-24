use std::time::Duration;
use tokio::task::JoinHandle;
use tokio::time::sleep;

/// Given: asks the mod server how big a jar is. Takes 300 ms of waiting.
pub async fn jar_size(id: u32) -> u64 {
    sleep(Duration::from_millis(300)).await;
    id as u64 * 1_000
}

/// Starts a task that builds the file name "<slug>-<version>.jar" and hands back its handle.
pub fn spawn_jar_name(slug: String, version: u32) -> JoinHandle<String> {
    tokio::spawn(async move { format!("{slug}-{version}.jar") })
}

/// One task per mod jar, all waiting at the same time. Sizes come back in the order of `ids`.
pub async fn all_jar_sizes(ids: Vec<u32>) -> Vec<u64> {
    let mut handles = Vec::new();
    for id in ids {
        handles.push(tokio::spawn(jar_size(id)));
    }
    let mut sizes = Vec::new();
    for handle in handles {
        sizes.push(handle.await.unwrap());
    }
    sizes
}

#[cfg(test)]
mod tests {
    use super::*;
    use std::time::Instant;
    use tokio::time::timeout;

    #[tokio::test]
    async fn spawn_jar_name_runs_in_its_own_task() {
        let handle = spawn_jar_name(String::from("sodium"), 6);
        let name = timeout(Duration::from_secs(5), handle).await.expect("timed out").unwrap();
        assert_eq!(name, "sodium-6.jar");
    }

    #[tokio::test]
    async fn all_jar_sizes_spawns_first_then_awaits() {
        let start = Instant::now();
        let sizes = timeout(Duration::from_secs(5), all_jar_sizes(vec![5, 1, 4, 2, 3]))
            .await
            .expect("timed out");
        let took = start.elapsed();
        assert_eq!(sizes, vec![5_000, 1_000, 4_000, 2_000, 3_000], "keep the order of ids");
        assert!(
            took < Duration::from_millis(800),
            "took {took:?}: spawn every task first, then await the handles"
        );
    }
}
