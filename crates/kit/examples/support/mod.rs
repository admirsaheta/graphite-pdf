use std::fs;
use std::path::{Path, PathBuf};
use std::time::{SystemTime, UNIX_EPOCH};

fn default_output_dir() -> PathBuf {
    Path::new(env!("CARGO_MANIFEST_DIR")).join("../../.artifacts/examples/kit")
}

pub fn output_path(example_name: &str) -> std::io::Result<PathBuf> {
    if let Ok(path) = std::env::var("GRAPHITEPDF_OUTPUT") {
        let path = PathBuf::from(path);
        if let Some(parent) = path.parent() {
            fs::create_dir_all(parent)?;
        }
        return Ok(path);
    }

    let dir = std::env::var_os("GRAPHITEPDF_OUTPUT_DIR")
        .map(PathBuf::from)
        .unwrap_or_else(default_output_dir);
    fs::create_dir_all(&dir)?;

    let stamp = SystemTime::now()
        .duration_since(UNIX_EPOCH)
        .unwrap_or_default()
        .as_millis();
    let pid = std::process::id();

    Ok(dir.join(format!("{example_name}-{stamp}-{pid}.pdf")))
}
