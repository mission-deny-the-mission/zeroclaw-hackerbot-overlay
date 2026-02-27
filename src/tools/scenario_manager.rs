//! Scenario Manager Tool - Cybersecurity Training Scenarios
//!
//! Manages cybersecurity training scenarios including navigation,
//! progress tracking, and scenario state management.

use async_trait::async_trait;
use serde::{Deserialize, Serialize};
use serde_json::json;
use std::collections::HashMap;
use std::sync::Arc;
use tokio::sync::Mutex;

/// Default training scenarios
fn default_scenarios() -> Vec<Scenario> {
    vec![
        Scenario {
            index: 0,
            title: "Initial Reconnaissance".to_string(),
            prompt: "Begin by gathering information about the target system. What services are running? What ports are open? Use nmap to scan the target.".to_string(),
            flag_id: "flag_1".to_string(),
            hint: "Try using nmap with the -sS flag for a stealthy SYN scan...".to_string(),
        },
        Scenario {
            index: 1,
            title: "Service Enumeration".to_string(),
            prompt: "Now that you've identified open ports, enumerate the services running on the target. What versions are they running?".to_string(),
            flag_id: "flag_2".to_string(),
            hint: "Use nmap with -sV to detect service versions...".to_string(),
        },
        Scenario {
            index: 2,
            title: "Initial Access".to_string(),
            prompt: "Gain initial access to the system using the credentials you've discovered or vulnerabilities you've identified.".to_string(),
            flag_id: "flag_3".to_string(),
            hint: "Check for weak credentials or misconfigured services...".to_string(),
        },
    ]
}

#[derive(Debug, Clone, Serialize, Deserialize)]
struct Scenario {
    index: usize,
    title: String,
    prompt: String,
    flag_id: String,
    hint: String,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
struct UserState {
    current_scenario: usize,
    completed_scenarios: Vec<usize>,
}

/// Scenario manager tool for training navigation
pub struct ScenarioManagerTool {
    scenarios: Vec<Scenario>,
    user_states: Arc<Mutex<HashMap<String, UserState>>>,
}

impl ScenarioManagerTool {
    pub fn new() -> Self {
        Self {
            scenarios: default_scenarios(),
            user_states: Arc::new(Mutex::new(HashMap::new())),
        }
    }

    fn list_scenarios(&self, user_state: &UserState) -> String {
        let mut output = String::from("Available cybersecurity training scenarios:\n\n");
        
        for scenario in &self.scenarios {
            let current = if scenario.index == user_state.current_scenario {
                " [CURRENT]"
            } else {
                ""
            };
            let completed = if user_state.completed_scenarios.contains(&scenario.index) {
                " ✓"
            } else {
                ""
            };
            output.push_str(&format!(
                "{}. {}{}{}\n",
                scenario.index + 1,
                scenario.title,
                current,
                completed
            ));
        }

        output.push_str(&format!(
            "\nProgress: {}/{} completed",
            user_state.completed_scenarios.len(),
            self.scenarios.len()
        ));

        output
    }

    fn goto_scenario(&self, user: &str, index: usize) -> anyhow::Result<String> {
        if index == 0 || index > self.scenarios.len() {
            anyhow::bail!(
                "Invalid scenario number. Must be between 1 and {}",
                self.scenarios.len()
            );
        }

        let scenario = &self.scenarios[index - 1];
        Ok(format!(
            "Jumped to scenario {}: {}\n\n{}",
            index, scenario.title, scenario.prompt
        ))
    }

    fn next_scenario(&self, user: &str) -> anyhow::Result<String> {
        let state = self.user_states
            .try_lock()
            .map_err(|_| anyhow::anyhow!("Failed to acquire state lock"))?
            .get(user)
            .cloned()
            .unwrap_or_else(|| UserState {
                current_scenario: 0,
                completed_scenarios: vec![],
            });
        
        if state.current_scenario >= self.scenarios.len() - 1 {
            return Ok("You're already at the final scenario. Great job!".to_string());
        }

        let new_index = state.current_scenario + 1;
        let scenario = &self.scenarios[new_index];

        Ok(format!(
            "Moving to scenario {}: {}\n\n{}",
            new_index + 1,
            scenario.title,
            scenario.prompt
        ))
    }

    fn previous_scenario(&self, user: &str) -> anyhow::Result<String> {
        let state = self.user_states
            .try_lock()
            .map_err(|_| anyhow::anyhow!("Failed to acquire state lock"))?
            .get(user)
            .cloned()
            .unwrap_or_else(|| UserState {
                current_scenario: 0,
                completed_scenarios: vec![],
            });
        
        if state.current_scenario == 0 {
            return Ok("You're already at the first scenario.".to_string());
        }

        let new_index = state.current_scenario - 1;
        let scenario = &self.scenarios[new_index];

        Ok(format!(
            "Going back to scenario {}: {}\n\n{}",
            new_index + 1,
            scenario.title,
            scenario.prompt
        ))
    }
}

impl Default for ScenarioManagerTool {
    fn default() -> Self {
        Self::new()
    }
}

#[async_trait]
impl zeroclaw::tools::Tool for ScenarioManagerTool {
    fn name(&self) -> &str {
        "scenario_manager"
    }

    fn description(&self) -> &str {
        "Cybersecurity training scenario manager. USE THIS for: 'list' (show scenarios), \
         'goto N' (jump to scenario N), 'next' (next scenario), 'previous' (previous scenario). \
         Do NOT use cron or other tools for these commands."
    }

    fn parameters_schema(&self) -> serde_json::Value {
        json!({
            "type": "object",
            "properties": {
                "command": {
                    "type": "string",
                    "enum": ["list", "goto", "next", "previous"],
                    "description": "Command: 'list' shows scenarios, 'goto N' jumps to scenario N, 'next'/'previous' navigate"
                },
                "scenario_index": {
                    "type": "integer",
                    "description": "Scenario number for 'goto' command (1-based)"
                },
                "user": {
                    "type": "string",
                    "description": "Username (defaults to message sender)"
                }
            },
            "required": ["command"]
        })
    }

    async fn execute(&self, args: serde_json::Value) -> anyhow::Result<zeroclaw::tools::ToolResult> {
        let command = args
            .get("command")
            .and_then(|v| v.as_str())
            .ok_or_else(|| anyhow::anyhow!("Missing 'command' parameter"))?;

        let user = args
            .get("user")
            .and_then(|v| v.as_str())
            .unwrap_or("default");

        let result = match command {
            "list" => {
                let state = self.user_states
                    .lock()
                    .await
                    .get(user)
                    .cloned()
                    .unwrap_or_else(|| UserState {
                        current_scenario: 0,
                        completed_scenarios: vec![],
                    });
                self.list_scenarios(&state)
            }
            "goto" => {
                let index = args
                    .get("scenario_index")
                    .and_then(|v| v.as_u64())
                    .ok_or_else(|| anyhow::anyhow!("'goto' requires 'scenario_index' parameter"))?;
                
                self.goto_scenario(user, index as usize)?
            }
            "next" => self.next_scenario(user)?,
            "previous" => self.previous_scenario(user)?,
            _ => {
                return Ok(zeroclaw::tools::ToolResult {
                    success: false,
                    output: String::new(),
                    error: Some(format!("Unknown command: {}", command)),
                });
            }
        };

        Ok(zeroclaw::tools::ToolResult {
            success: true,
            output: result,
            error: None,
        })
    }
}
