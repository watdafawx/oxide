use std::fs;
use std::path::Path;

/// Skipped when staging quest content for the bundle (build output the quests leave behind).
const SKIP: &[&str] = &["target", "build", ".gradle", "node_modules"];

fn stage(from: &Path, to: &Path) -> std::io::Result<()> {
    fs::create_dir_all(to)?;
    for entry in fs::read_dir(from)? {
        let entry = entry?;
        let name = entry.file_name();
        let src = entry.path();
        let dst = to.join(&name);
        if entry.file_type()?.is_dir() {
            if !SKIP.iter().any(|s| name == *s) {
                stage(&src, &dst)?;
            }
        } else {
            let (sm, dm) = (entry.metadata()?, fs::metadata(&dst).ok());
            let same = dm.is_some_and(|d| d.len() == sm.len() && d.modified().ok() >= sm.modified().ok());
            if !same {
                fs::copy(&src, &dst)?;
            }
        }
    }
    Ok(())
}

fn main() {
    // `bundle.resources` can't exclude files, so the bundle maps this clean copy instead of `content/tracks`.
    let src = Path::new("../content/tracks");
    let dst = Path::new("bundle-content/content/tracks");
    if std::env::var("PROFILE").as_deref() == Ok("release") {
        println!("cargo:rerun-if-changed=../content/tracks");
        let _ = fs::remove_dir_all(dst);
        stage(src, dst).expect("stage content for bundle");
    } else {
        // Dev builds read quests from the repo (see `content_dir`), so they skip the copy, and editing a
        // quest no longer reruns this script and rebuilds the app. The folder only has to exist for tauri-build.
        println!("cargo:rerun-if-changed=build.rs");
        fs::create_dir_all(dst).expect("bundle-content folder");
    }
    tauri_build::build()
}
