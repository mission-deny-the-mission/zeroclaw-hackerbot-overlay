//! Flag Validator Tool - Deterministic Flag Verification
//!
//! This tool verifies flag capture from student machines using direct SSH/CLI commands.
//! **NOT vulnerable to prompt injection** - executes deterministic code with regex validation.

use async_trait::async_trait;
use chrono::{DateTime, Utc};
use serde::{Deserialize, Serialize};
use serde_json::json;
use tokio::process::Command;

/// Flag validator tool for deterministic flag verification
pub struct FlagValidatorTool {
    /// SSH timeout in seconds
    timeout_secs: u64,
    /// Default flag path pattern
    default_flag_path: String,
}

/// Flag validation result
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct FlagResult {
    pub captured: bool,
    pub flag_value: String,
    pub message: String,
    pub timestamp: DateTime<Utc>,
    pub target_ip: String,
}

impl FlagValidatorTool {
    pub fn new() -> Self {
        Self {
            timeout_secs: 30,
            default_flag_path: "/root/flag.txt".to_string(),
        }
    }

    /// Verify flag capture from a target machine
    pub async fn verify_flag(
        &self,
        target_ip: &str,
        flag_path: Option<&str>,
        expected_pattern: Option<&str>,
    ) -> anyhow::Result<FlagResult> {
        let flag_path = flag_path.unwrap_or(&self.default_flag_path);
        
        // Build SSH command deterministically
        let output = Command::new("ssh")
            .args(&[
                "-o", "StrictHostKeyChecking=no",
                "-o", "ConnectTimeout=10",
                &format!("root@{}", target_ip),
                &format!("cat {}", flag_path),
            ])
            .output()
            .await?;

        let flag_content = String::from_utf8_lossy(&output.stdout).trim().to_string();

        // Validate flag format with regex (deterministic!)
        let flag_regex = regex::Regex::new(r"SEC GEN\{[a-f0-9]+\}")?;
        
        let captured = if let Some(pattern) = expected_pattern {
            let custom_regex = regex::Regex::new(pattern)?;
            custom_regex.is_match(&flag_content)
        } else {
            flag_regex.is_match(&flag_content)
        };

        let message = if captured {
            "Flag successfully captured!".to_string()
        } else {
            format!("Flag not found or invalid format. Content: {}", 
                   if flag_content.is_empty() { "(empty)" } else { "(see logs)" })
        };

        Ok(FlagResult {
            captured,
            flag_value: if captured { flag_content } else { String::new() },
            message,
            timestamp: Utc::now(),
            target_ip: target_ip.to_string(),
        })
    }

    /// Verify command output matches expected pattern
    pub async fn verify_command_output(
        &self,
        target_ip: &str,
        command: &str,
        expected_pattern: &str,
    ) -> anyhow::Result<CommandResult> {
        let output = Command::new("ssh")
            .args(&[
                "-o", "StrictHostKeyChecking=no",
                "-o", "ConnectTimeout=10",
                &format!("root@{}", target_ip),
                command,
            ])
            .output()
            .await?;

        let output_str = String::from_utf8_lossy(&output.stdout);
        let regex = regex::Regex::new(expected_pattern)?;
        
        let matches = regex.is_match(&output_str);

        Ok(CommandResult {
            matches,
            output: output_str.to_string(),
            exit_code: output.status.code().unwrap_or(-1),
            timestamp: Utc::now(),
        })
    }
}

/// Command execution result
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct CommandResult {
    pub matches: bool,
    pub output: String,
    pub exit_code: i32,
    pub timestamp: DateTime<Utc>,
}

impl Default for FlagValidatorTool {
    fn default() -> Self {
        Self::new()
    }
}

#[async_trait]
impl zeroclaw::tools::Tool for FlagValidatorTool {
    fn name(&self) -> &str {
        "flag_validator"
    }

    fn description(&self) -> &str {
        "Verify flag capture from student machines using deterministic SSH commands. \
         Connects to target machine, reads flag file, validates format with regex. \
         NOT vulnerable to prompt injection - pure deterministic validation."
    }

    fn parameters_schema(&self) -> serde_json::Value {
        json!({
            "type": "object",
            "properties": {
                "target_ip": {
                    "type": "string",
                    "description": "IP address of target machine to check"
                },
                "flag_path": {
                    "type": "string",
                    "description": "Path to flag file (default: /root/flag.txt)"
                },
                "expected_pattern": {
                    "type": "string",
                    "description": "Regex pattern for valid flag (default: SEC GEN{...})"
                }
            },
            "required": ["target_ip"]
        })
    }

    async fn execute(&self, args: serde_json::Value) -> anyhow::Result<zeroclaw::tools::ToolResult> {
        let target_ip = args
            .get("target_ip")
            .and_then(|v| v.as_str())
            .ok_or_else(|| anyhow::anyhow!("Missing 'target_ip' parameter"))?;

        let flag_path = args.get("flag_path").and_then(|v| v.as_str());
        let expected_pattern = args.get("expected_pattern").and_then(|v| v.as_str());

        let result = self.verify_flag(target_ip, flag_path, expected_pattern).await?;

        Ok(zeroclaw::tools::ToolResult {
            success: result.captured,
            output: format!(
                "{}\nFlag: {} | Time: {}",
                result.message,
                if result.flag_value.is_empty() { "(not captured)" } else { &result.flag_value },
                result.timestamp.format("%Y-%m-%d %H:%M:%S UTC")
            ),
            error: None,
        })
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_flag_regex_valid() {
        let regex = regex::Regex::new(r"SEC GEN\{[a-f0-9]+\}").unwrap();
        
        assert!(regex.is_match("SEC GEN{abc123}"));
        assert!(regex.is_match("SEC GEN{deadbeef}"));
        assert!(regex.is_match("SEC GEN{1234567890abcdef}"));
    }

    #[test]
    fn test_flag_regex_invalid() {
        let regex = regex::Regex::new(r"SEC GEN\{[a-f0-9]+\}").unwrap();
        
        assert!(!regex.is_match("FLAG{abc123}"));
        assert!(!regex.is_match("SEC GEN{xyz123}"));  // Invalid hex
        assert!(!regex.is_match("SEC GEN{}"));  // Empty
    }
}
