use tokio::runtime::Runtime;

/// Given: the launcher's status line for a mod, "Downloading mod-21.jar".
pub async fn announce(id: u32) -> String {
    format!("Downloading mod-{id}.jar")
}

/// The same status line, for plain (not async) code such as the launcher's settings screen.
pub fn announce_now(id: u32) -> String {
    // TODO: start a tokio runtime, and use it to run announce(id) to the end
    todo!()
}

#[cfg(test)]
mod tests {
    use super::*;

    // A plain #[test]: no tokio here, so announce_now has to start its own runtime.
    #[test]
    fn announce_now_runs_the_future_to_the_end() {
        assert_eq!(announce_now(21), "Downloading mod-21.jar");
        assert_eq!(announce_now(306), "Downloading mod-306.jar");
    }
}
