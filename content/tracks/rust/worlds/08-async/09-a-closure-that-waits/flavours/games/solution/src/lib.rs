/// Given: the file name a mod is saved under. Pretend it asks the mod server.
pub async fn jar_name(id: u32) -> String {
    tokio::task::yield_now().await;
    format!("mod-{id}.jar")
}

/// Given: runs `step` for every id, one after the other, and collects what it returns.
pub async fn for_each_mod(ids: Vec<u32>, step: impl AsyncFn(u32) -> String) -> Vec<String> {
    let mut out = Vec::new();
    for id in ids {
        out.push(step(id).await);
    }
    out
}

/// "[Fabric] mod-1.jar" for every id, built by an async closure passed to for_each_mod.
pub async fn tagged(ids: Vec<u32>, loader: &str) -> Vec<String> {
    for_each_mod(ids, async |id| {
        let file = jar_name(id).await;
        format!("[{loader}] {file}")
    })
    .await
}

/// A tagger that keeps its own copy of `loader` and can be called again and again.
pub fn tagger(loader: String) -> impl AsyncFn(u32) -> String {
    async move |id| {
        let file = jar_name(id).await;
        format!("[{loader}] {file}")
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use std::time::Duration;
    use tokio::time::timeout;

    #[tokio::test]
    async fn tagged_awaits_inside_the_closure() {
        let out = timeout(Duration::from_secs(5), tagged(vec![1, 2], "Fabric")).await.expect("timed out");
        assert_eq!(out, vec!["[Fabric] mod-1.jar", "[Fabric] mod-2.jar"]);
    }

    #[tokio::test]
    async fn tagger_keeps_its_own_loader_name() {
        let label = {
            let loader = String::from("Forge");
            tagger(loader)
        }; // `loader` is gone here, the closure owns its copy
        assert_eq!(label(3).await, "[Forge] mod-3.jar");
        assert_eq!(label(4).await, "[Forge] mod-4.jar");
    }
}
