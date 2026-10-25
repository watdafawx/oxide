use std::time::Duration;
use tokio::time::sleep;

/// Given: asks the mod server how big a jar is. Takes 300 ms of waiting.
pub async fn jar_size(id: u32) -> u64 {
    sleep(Duration::from_millis(300)).await;
    id as u64 * 1_000
}

/// Sizes of two mod jars, asked one after the other.
pub async fn jar_sizes_in_turn(a: u32, b: u32) -> (u64, u64) {
    // TODO: await jar_size(a), then await jar_size(b), and return both
    todo!()
}

/// Sizes of two mod jars, with both questions waiting at the same time.
pub async fn jar_sizes_together(a: u32, b: u32) -> (u64, u64) {
    // TODO: run both jar_size calls at the same time and return both results
    todo!()
}

#[cfg(test)]
mod tests {
    use super::*;
    use std::time::Instant;
    use tokio::time::timeout;

    #[tokio::test]
    async fn in_turn_returns_both_jar_sizes() {
        let start = Instant::now();
        let out = timeout(Duration::from_secs(5), jar_sizes_in_turn(3, 8)).await.expect("timed out");
        assert_eq!(out, (3_000, 8_000), "took {:?}", start.elapsed());
    }

    #[tokio::test]
    async fn together_returns_both_jar_sizes() {
        let out = timeout(Duration::from_secs(5), jar_sizes_together(4, 9)).await.expect("timed out");
        assert_eq!(out, (4_000, 9_000));
    }

    #[tokio::test]
    async fn together_overlaps_the_mod_server_waits() {
        let start = Instant::now();
        let out = timeout(Duration::from_secs(5), jar_sizes_together(1, 2)).await.expect("timed out");
        let took = start.elapsed();
        assert_eq!(out, (1_000, 2_000));
        assert!(
            took < Duration::from_millis(520),
            "took {took:?}: the two waits ran one after the other, use join!"
        );
    }
}
