/// Splits a download of `total` bytes into `parts` equal parts, in a task of its own.
/// `parts == 0` makes the division panic inside the task: then give back None.
pub async fn part_size(total: u64, parts: u64) -> Option<u64> {
    let handle = tokio::spawn(async move { total / parts });
    handle.await.ok()
}

#[cfg(test)]
mod tests {
    use super::*;
    use std::time::Duration;
    use tokio::time::timeout;

    #[tokio::test]
    async fn part_size_survives_a_panic() {
        let ok = timeout(Duration::from_secs(5), part_size(9_000, 3)).await.expect("timed out");
        assert_eq!(ok, Some(3_000));
        // The panic message printed below is expected: the task panics, the test does not.
        let bad = timeout(Duration::from_secs(5), part_size(9_000, 0)).await.expect("timed out");
        assert_eq!(bad, None, "a panicking task must give None, not crash the caller");
    }
}
