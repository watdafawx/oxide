/// Given: the file name an episode is saved under. Pretend it asks the server.
pub async fn episode_file(id: u32) -> String {
    tokio::task::yield_now().await;
    format!("episode-{id}.mp3")
}

/// Given: runs `step` for every id, one after the other, and collects what it returns.
pub async fn for_each_episode(ids: Vec<u32>, step: impl AsyncFn(u32) -> String) -> Vec<String> {
    let mut out = Vec::new();
    for id in ids {
        out.push(step(id).await);
    }
    out
}

/// "daily: episode-1.mp3" for every id, built by an async closure passed to for_each_episode.
pub async fn labelled(ids: Vec<u32>, show: &str) -> Vec<String> {
    for_each_episode(ids, async |id| {
        let file = episode_file(id).await;
        format!("{show}: {file}")
    })
    .await
}

/// A labeller that keeps its own copy of `show` and can be called again and again.
pub fn labeller(show: String) -> impl AsyncFn(u32) -> String {
    async move |id| {
        let file = episode_file(id).await;
        format!("{show}: {file}")
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use std::time::Duration;
    use tokio::time::timeout;

    #[tokio::test]
    async fn labelled_awaits_inside_the_closure() {
        let out = timeout(Duration::from_secs(5), labelled(vec![1, 2], "daily")).await.expect("timed out");
        assert_eq!(out, vec!["daily: episode-1.mp3", "daily: episode-2.mp3"]);
    }

    #[tokio::test]
    async fn labeller_keeps_its_own_show_name() {
        let label = {
            let show = String::from("tech");
            labeller(show)
        }; // `show` is gone here, the closure owns its copy
        assert_eq!(label(3).await, "tech: episode-3.mp3");
        assert_eq!(label(4).await, "tech: episode-4.mp3");
    }
}
