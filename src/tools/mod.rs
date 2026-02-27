//! Hackerbot Tools Module
//!
//! This module exports all Hackerbot-specific tools for ZeroClaw integration.

pub mod quiz_validator;
pub mod flag_validator;
pub mod scenario_manager;
pub mod secgen_datastore;

pub use quiz_validator::QuizValidatorTool;
pub use flag_validator::FlagValidatorTool;
pub use scenario_manager::ScenarioManagerTool;
pub use secgen_datastore::SecGenDatastoreTool;

/// Initialize all Hackerbot tools
pub fn init_all_tools(secgen_datastore_path: Option<&str>) -> Vec<Box<dyn zeroclaw::tools::Tool>> {
    vec![
        Box::new(QuizValidatorTool::new()),
        Box::new(FlagValidatorTool::new()),
        Box::new(ScenarioManagerTool::new()),
        Box::new(SecGenDatastoreTool::new(secgen_datastore_path)),
    ]
}
