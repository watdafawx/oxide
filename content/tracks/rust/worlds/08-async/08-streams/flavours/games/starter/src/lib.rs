use futures::future::ready;
use futures::{stream, StreamExt};

/// [500, 2048, 1024, 4096, 100] (KB) -> [2, 1, 4] (MB)
pub async fn big_jars_in_mb(sizes_kb: Vec<u64>) -> Vec<u64> {
    // TODO: stream::iter + filter(ready(..)) + map + collect
    todo!()
}

/// n = 4 -> [0, 1024, 2048, 3072]. The chunk numbers 0, 1, 2, ... never run out.
pub async fn first_n_chunk_offsets(n: usize) -> Vec<u64> {
    // TODO: stream::iter(0u64..) + map + take(n) + collect
    todo!()
}

/// ["sodium.jar", "jei.jar"] -> 17
pub async fn total_name_length(names: Vec<&str>) -> usize {
    // TODO: stream::iter + fold
    todo!()
}

#[cfg(test)]
mod tests {
    use super::*;
    use std::time::Duration;
    use tokio::time::timeout;

    #[tokio::test]
    async fn big_jars_in_mb_filters_then_maps() {
        let out = timeout(
            Duration::from_secs(2),
            big_jars_in_mb(vec![500, 2048, 1024, 4096, 100]),
        )
        .await
        .expect("timed out");
        assert_eq!(out, vec![2, 1, 4]);
    }

    #[tokio::test]
    async fn first_n_chunk_offsets_stops_early() {
        let out = timeout(Duration::from_secs(2), first_n_chunk_offsets(4))
            .await
            .expect("timed out: the stream is endless, did you take(n)?");
        assert_eq!(out, vec![0, 1024, 2048, 3072]);
    }

    #[tokio::test]
    async fn total_name_length_folds_the_stream() {
        let out = timeout(
            Duration::from_secs(2),
            total_name_length(vec!["sodium.jar", "jei.jar"]),
        )
        .await
        .expect("timed out");
        assert_eq!(out, 17);
    }
}
