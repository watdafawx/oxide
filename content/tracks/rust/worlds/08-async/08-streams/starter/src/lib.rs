use futures::future::ready;
use futures::{stream, StreamExt};

/// [1, 2, 3, 4] -> [4, 8]
pub async fn doubled_evens(nums: Vec<i32>) -> Vec<i32> {
    // TODO: stream::iter(nums), filter evens (closure returns ready(bool)), map, collect
    todo!()
}

/// n = 4 -> [1, 4, 9, 16]. The source stream is endless.
pub async fn first_n_squares(n: usize) -> Vec<u64> {
    // TODO: stream::iter(1u64..), map to squares, take(n), collect
    todo!()
}

/// ["a", "bb", "ccc"] -> 6
pub async fn total_length(words: Vec<&str>) -> usize {
    // TODO: turn words into a stream and fold it, adding each word's length (the closure returns a future)
    todo!()
}

#[cfg(test)]
mod tests {
    use super::*;
    use std::time::Duration;
    use tokio::time::timeout;

    #[tokio::test]
    async fn doubled_evens_filters_then_maps() {
        let out = timeout(Duration::from_secs(2), doubled_evens(vec![1, 2, 3, 4, 5, 6]))
            .await
            .expect("timed out");
        assert_eq!(out, vec![4, 8, 12]);
    }

    #[tokio::test]
    async fn first_n_squares_stops_early() {
        let out = timeout(Duration::from_secs(2), first_n_squares(5))
            .await
            .expect("timed out: the stream is endless, did you take(n)?");
        assert_eq!(out, vec![1, 4, 9, 16, 25]);
    }

    #[tokio::test]
    async fn total_length_folds_the_stream() {
        let out = timeout(Duration::from_secs(2), total_length(vec!["a", "bb", "ccc"]))
            .await
            .expect("timed out");
        assert_eq!(out, 6);
    }
}
