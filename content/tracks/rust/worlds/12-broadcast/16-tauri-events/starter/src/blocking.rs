//! Heavy work from async code: the feed must keep flowing while photos are cut.

#![allow(unused_imports)]

use crate::*;

/// Run blocking or CPU-heavy work on tokio's blocking threads and wait for it, without holding up other tasks.
pub async fn off_the_runtime<T: Send + 'static>(work: impl FnOnce() -> T + Send + 'static) -> T {
    tokio::task::spawn_blocking(work).await.expect("the work panicked")
}
