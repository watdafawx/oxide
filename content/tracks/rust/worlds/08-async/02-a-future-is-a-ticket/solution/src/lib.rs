/// The file name an episode is saved under: "episode-7.mp3" for id 7.
pub async fn episode_file(id: u32) -> String {
    format!("episode-{id}.mp3")
}

/// The status line: "Downloading episode-7.mp3". Uses `episode_file`.
pub async fn describe(id: u32) -> String {
    let file = episode_file(id).await;
    format!("Downloading {file}")
}

#[cfg(test)]
mod tests {
    use super::*;

    #[tokio::test]
    async fn episode_file_names_the_file() {
        assert_eq!(episode_file(7).await, "episode-7.mp3");
        assert_eq!(episode_file(120).await, "episode-120.mp3");
    }

    #[tokio::test]
    async fn describe_awaits_the_file_name() {
        assert_eq!(describe(7).await, "Downloading episode-7.mp3");
    }
}
