/// Splits a jar of `total` bytes over `mirrors` equal parts, one per mirror, in a task of its own.
/// `mirrors == 0` makes the division panic inside the task: then give back None.
pub async fn bytes_per_mirror(total: u64, mirrors: u64) -> Option<u64> {
    let handle = tokio::spawn(async move { total / mirrors });
    handle.await.ok()
}

#[cfg(test)]
mod tests {
    use super::*;
    use std::time::Duration;
    use tokio::time::timeout;

    #[tokio::test]
    async fn bytes_per_mirror_survives_a_panic() {
        let ok = timeout(Duration::from_secs(5), bytes_per_mirror(9_000, 3)).await.expect("timed out");
        assert_eq!(ok, Some(3_000));
        // The panic message printed below is expected: the task panics, the test does not.
        let bad = timeout(Duration::from_secs(5), bytes_per_mirror(9_000, 0)).await.expect("timed out");
        assert_eq!(bad, None, "a panicking task must give None, not crash the caller");
    }
}
