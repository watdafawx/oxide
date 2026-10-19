/// "Downloading sodium.jar" for jar = "sodium.jar".
pub async fn announce_download(jar: &str) -> String {
    // TODO: format!("Downloading {jar}")
    todo!()
}

/// Adds two file sizes (in bytes).
pub async fn total_size(a: u64, b: u64) -> u64 {
    // TODO: a + b
    todo!()
}

#[cfg(test)]
mod tests {
    use super::*;

    #[tokio::test]
    async fn announce_download_names_the_jar() {
        assert_eq!(announce_download("sodium.jar").await, "Downloading sodium.jar");
    }

    #[tokio::test]
    async fn total_size_adds_two_files() {
        assert_eq!(total_size(2_000, 40).await, 2_040);
        assert_eq!(total_size(0, 0).await, 0);
    }
}
