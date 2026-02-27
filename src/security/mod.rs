//! Security Module for ZeroClaw Hackerbot Overlay
//!
//! Provides configurable security levels and protections:
//! - Datastore query blocking
//! - Flag encryption
//! - Bot sandboxing
//! - Prompt hardening

pub mod config;

pub use config::{SecurityConfig, SecurityLevel};
