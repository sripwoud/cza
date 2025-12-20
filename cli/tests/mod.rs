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
