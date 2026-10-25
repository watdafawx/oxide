/// The file name a mod is saved under: "mod-21.jar" for id 21.
pub async fn jar_name(id: u32) -> String {
    // TODO: build the text "mod-<id>.jar"
    todo!()
}

/// The launcher's status line: "Downloading mod-21.jar". Uses `jar_name`.
pub async fn announce(id: u32) -> String {
    // TODO: wait for jar_name(id), then put "Downloading " in front of it
    todo!()
}

#[cfg(test)]
mod tests {
    use super::*;

    #[tokio::test]
    async fn jar_name_names_the_file() {
        assert_eq!(jar_name(21).await, "mod-21.jar");
        assert_eq!(jar_name(306).await, "mod-306.jar");
    }

    #[tokio::test]
    async fn announce_awaits_the_jar_name() {
        assert_eq!(announce(21).await, "Downloading mod-21.jar");
    }
}
