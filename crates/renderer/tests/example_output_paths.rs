#[path = "../examples/support/mod.rs"]
mod support;

use std::path::{Path, PathBuf};
use std::sync::{LazyLock, Mutex};

static ENV_LOCK: LazyLock<Mutex<()>> = LazyLock::new(|| Mutex::new(()));

struct EnvState {
    output: Option<String>,
    output_dir: Option<PathBuf>,
}

impl EnvState {
    fn capture() -> Self {
        Self {
            output: std::env::var("GRAPHITEPDF_OUTPUT").ok(),
            output_dir: std::env::var_os("GRAPHITEPDF_OUTPUT_DIR").map(PathBuf::from),
        }
    }

    fn restore(self) {
        match self.output {
            Some(value) => unsafe { std::env::set_var("GRAPHITEPDF_OUTPUT", value) },
            None => unsafe { std::env::remove_var("GRAPHITEPDF_OUTPUT") },
        }

        match self.output_dir {
            Some(value) => unsafe { std::env::set_var("GRAPHITEPDF_OUTPUT_DIR", value) },
            None => unsafe { std::env::remove_var("GRAPHITEPDF_OUTPUT_DIR") },
        }
    }
}

fn with_env_reset(test: impl FnOnce()) {
    let _guard = ENV_LOCK.lock().expect("env lock should not be poisoned");
    let saved = EnvState::capture();
    unsafe {
        std::env::remove_var("GRAPHITEPDF_OUTPUT");
        std::env::remove_var("GRAPHITEPDF_OUTPUT_DIR");
    }

    test();
    saved.restore();
}

#[test]
fn defaults_to_workspace_artifacts_directory() {
    with_env_reset(|| {
        let path = support::output_path("pipeline").expect("default output path should resolve");

        assert!(path.starts_with(support::workspace_path(".artifacts/examples/renderer")));
        assert!(
            path.file_name()
                .and_then(|name| name.to_str())
                .is_some_and(|name| name.starts_with("pipeline-") && name.ends_with(".pdf"))
        );
        assert!(path.parent().is_some_and(Path::exists));
    });
}

#[test]
fn honors_explicit_output_file_path() {
    with_env_reset(|| {
        let exact = std::env::temp_dir()
            .join("graphitepdf-task8")
            .join(format!("renderer-example-{}.pdf", std::process::id()));
        unsafe {
            std::env::set_var("GRAPHITEPDF_OUTPUT", &exact);
        }

        let path =
            support::output_path("pipeline").expect("explicit output file path should resolve");

        assert_eq!(path, exact);
        assert!(path.parent().is_some_and(Path::exists));
    });
}

#[test]
fn honors_output_directory_override() {
    with_env_reset(|| {
        let dir = std::env::temp_dir().join(format!(
            "graphitepdf-task8-renderer-dir-{}",
            std::process::id()
        ));
        unsafe {
            std::env::set_var("GRAPHITEPDF_OUTPUT_DIR", &dir);
        }

        let path = support::output_path("pipeline").expect("output dir override should resolve");

        assert!(path.starts_with(&dir));
        assert_eq!(
            path.extension().and_then(|value| value.to_str()),
            Some("pdf")
        );
        assert!(
            path.parent()
                .is_some_and(|parent| parent == dir && parent.exists())
        );
    });
}
