//! ZeroClaw Hackerbot Overlay
//! 
//! This library provides cybersecurity training capabilities for ZeroClaw.
//! It adds deterministic quiz validation, flag verification, and SecGen integration
//! WITHOUT requiring a ZeroClaw fork.

pub mod tools;
pub mod security;

pub use tools::{
    QuizValidatorTool,
    FlagValidatorTool,
    ScenarioManagerTool,
    SecGenDatastoreTool,
};
pub use security::{SecurityConfig, SecurityLevel};

/// Overlay version
pub const VERSION: &str = env!("CARGO_PKG_VERSION");

/// Initialize all Hackerbot tools with security configuration
pub fn init_tools(secgen_datastore_path: Option<&str>, security: &SecurityConfig) -> Vec<Box<dyn zeroclaw::tools::Tool>> {
    vec![
        Box::new(QuizValidatorTool::new()),
        Box::new(FlagValidatorTool::new(security.clone())),
        Box::new(ScenarioManagerTool::new()),
        Box::new(SecGenDatastoreTool::new(secgen_datastore_path, security.clone())),
    ]
}
