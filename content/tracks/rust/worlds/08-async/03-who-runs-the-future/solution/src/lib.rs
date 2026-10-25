use tokio::runtime::Runtime;

/// Given: the status line for an episode, "Downloading episode-7.mp3".
pub async fn describe(id: u32) -> String {
    format!("Downloading episode-{id}.mp3")
}

/// The same status line, for plain (not async) code such as a settings screen.
pub fn describe_now(id: u32) -> String {
    let rt = Runtime::new().unwrap();
    rt.block_on(describe(id))
}

#[cfg(test)]
mod tests {
    use super::*;

    // A plain #[test]: no tokio here, so describe_now has to start its own runtime.
    #[test]
    fn describe_now_runs_the_future_to_the_end() {
        assert_eq!(describe_now(7), "Downloading episode-7.mp3");
        assert_eq!(describe_now(42), "Downloading episode-42.mp3");
    }
}
