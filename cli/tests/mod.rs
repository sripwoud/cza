// Test modules
mod basic_tests;
mod integration_tests;

// Common test utilities
use std::sync::Once;

static INIT: Once = Once::new();

pub fn init() {
    INIT.call_once(|| {
        // Initialize test environment
        env_logger::try_init().ok();
    });
}

// Test helpers
pub mod helpers {
    use std::env;
    use std::path::PathBuf;

    pub fn cargo_bin_path() -> PathBuf {
        let mut path = env::current_exe().unwrap();
        path.pop(); // Remove test executable name
        if path.ends_with("deps") {
            path.pop();
        }
        path.join("cza")
    }

    pub fn with_temp_dir<F>(test: F)
    where
        F: FnOnce(&std::path::Path),
    {
        let temp_dir = tempfile::TempDir::new().unwrap();
        test(temp_dir.path());
    }

    pub fn create_test_template_registry() -> String {
        r#"
[templates.test-template]
name = "Test Template"
description = "A template for testing"
repository = "https://github.com/test/test-template"
frameworks = ["test"]
"#
        .to_string()
    }
}
