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
    let dst = Path::new("bundle-content/tracks");
    println!("cargo:rerun-if-changed=../content/tracks");
    let _ = fs::remove_dir_all(dst);
    stage(src, dst).expect("stage content for bundle");
    tauri_build::build()
}
