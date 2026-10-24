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
    // TODO: call for_each_mod with an async closure that awaits jar_name(id)
    // and puts "[<loader>] " in front of it; await the result
    let _ = (ids, loader);
    todo!()
}

/// A tagger that keeps its own copy of `loader` and can be called again and again.
pub fn tagger(loader: String) -> impl AsyncFn(u32) -> String {
    // TODO: return an async closure that moves `loader` in and builds the same tag as above
    let _ = loader;
    async |_id: u32| -> String { todo!() }
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
