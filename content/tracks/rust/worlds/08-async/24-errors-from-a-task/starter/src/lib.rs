use tokio::task::JoinSet;

/// Any error that may cross from one task to another. `?` turns parse and join errors into it.
pub type Error = Box<dyn std::error::Error + Send + Sync>;

/// Given: asks the server for an episode's size, as text. Episode 13's answer is broken.
pub async fn size_text(id: u32) -> String {
    tokio::task::yield_now().await;
    if id == 13 { "big".to_string() } else { format!("{}", id * 1_000) }
}

/// Asks for every size in its own task and adds them up. Any broken answer makes the whole total an Err.
pub async fn total_size(ids: Vec<u32>) -> Result<u64, Error> {
    // TODO: spawn one task per id into a JoinSet; each awaits size_text(id), parses it
    // with ?, and ends with an Ok whose types are spelled out
    // TODO: add up the results as they come out, letting both kinds of error through with ?
    let _ = (ids, JoinSet::<Result<u64, Error>>::new());
    todo!()
}

#[cfg(test)]
mod tests {
    use super::*;
    use std::time::Duration;
    use tokio::time::timeout;

    #[tokio::test]
    async fn total_size_adds_every_episode() {
        let total = timeout(Duration::from_secs(5), total_size(vec![1, 2, 4]))
            .await
            .expect("timed out")
            .expect("all three sizes are fine");
        assert_eq!(total, 7_000);
    }

    #[tokio::test]
    async fn a_broken_size_is_an_error() {
        let r = timeout(Duration::from_secs(5), total_size(vec![1, 13, 2])).await.expect("timed out");
        let e = r.expect_err("episode 13 answered \"big\", so the total must be an Err");
        assert!(e.to_string().contains("invalid digit"), "the parse error should come through, got: {e}");
    }
}
