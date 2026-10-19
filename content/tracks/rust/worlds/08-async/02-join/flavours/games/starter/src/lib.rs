use std::time::Duration;
use tokio::time::sleep;

/// Waits 300 ms (pretend: a network call), then returns the mod's size in KB. Given.
pub async fn fetch_size(mod_id: u32) -> u32 {
    sleep(Duration::from_millis(300)).await;
    mod_id * 100
}

/// "Queued sodium.jar" for jar = "sodium.jar". Given.
pub async fn queue_line(jar: &str) -> String {
    format!("Queued {jar}")
}

/// Two queue lines one after the other, joined by a space:
/// "Queued sodium.jar Queued sodium.jar"
pub async fn queue_twice(jar: &str) -> String {
    // TODO: await queue_line twice, join the results with a space
    todo!()
}

/// Returns both sizes. The two waits must overlap.
pub async fn fetch_both(a: u32, b: u32) -> (u32, u32) {
    // TODO: tokio::join!
    todo!()
}

/// Fetches both sizes and adds them.
pub async fn total_kb(a: u32, b: u32) -> u32 {
    // TODO: join! both fetches, then add
    todo!()
}

#[cfg(test)]
mod tests {
    use super::*;
    use std::time::Instant;

    #[tokio::test]
    async fn fetch_both_returns_both_sizes() {
        let out = tokio::time::timeout(Duration::from_secs(5), fetch_both(3, 8))
            .await
            .expect("timed out");
        assert_eq!(out, (300, 800));
    }

    #[tokio::test]
    async fn fetch_both_overlaps_the_waits() {
        let start = Instant::now();
        let out = tokio::time::timeout(Duration::from_secs(5), fetch_both(1, 2))
            .await
            .expect("timed out");
        let took = start.elapsed();
        assert_eq!(out, (100, 200));
        assert!(
            took < Duration::from_millis(520),
            "took {took:?}: the two waits ran one after another, use join!"
        );
    }

    #[tokio::test]
    async fn queue_twice_awaits_both_lines() {
        assert_eq!(
            queue_twice("iris.jar").await,
            "Queued iris.jar Queued iris.jar"
        );
    }

    #[tokio::test]
    async fn total_kb_adds_both_sizes() {
        let start = Instant::now();
        let out = tokio::time::timeout(Duration::from_secs(5), total_kb(10, 20))
            .await
            .expect("timed out");
        assert_eq!(out, 3_000);
        assert!(start.elapsed() < Duration::from_millis(520), "use join! so the waits overlap");
    }
}
