use std::time::Duration;
use tokio::task::JoinHandle;
use tokio::time::sleep;

/// Given: asks the server how big an episode is. Takes 300 ms of waiting.
pub async fn episode_size(id: u32) -> u64 {
    sleep(Duration::from_millis(300)).await;
    id as u64 * 1_000
}

/// Starts a task that builds the file name "<show>-<id>.mp3" and hands back its handle.
pub fn spawn_file_name(show: String, id: u32) -> JoinHandle<String> {
    tokio::spawn(async move { format!("{show}-{id}.mp3") })
}

/// One task per episode, all waiting at the same time. Sizes come back in the order of `ids`.
pub async fn all_sizes(ids: Vec<u32>) -> Vec<u64> {
    let mut handles = Vec::new();
    for id in ids {
        handles.push(tokio::spawn(episode_size(id)));
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
    async fn spawn_file_name_runs_in_its_own_task() {
        let handle = spawn_file_name(String::from("daily"), 3);
        let name = timeout(Duration::from_secs(5), handle).await.expect("timed out").unwrap();
        assert_eq!(name, "daily-3.mp3");
    }

    #[tokio::test]
    async fn all_sizes_spawns_first_then_awaits() {
        let start = Instant::now();
        let sizes = timeout(Duration::from_secs(5), all_sizes(vec![5, 1, 4, 2, 3]))
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
