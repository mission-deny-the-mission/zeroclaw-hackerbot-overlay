//! SecGen Datastore Tool - Access SecGen Randomized Values
//!
//! This tool provides read-only access to SecGen datastore for accessing
//! randomized IPs, credentials, flags, and other scenario data.

use async_trait::async_trait;
use serde_json::json;
use std::collections::HashMap;
use std::path::PathBuf;

/// SecGen datastore access tool
pub struct SecGenDatastoreTool {
    datastore_path: Option<PathBuf>,
    cache: std::sync::RwLock<HashMap<String, serde_json::Value>>,
}

impl SecGenDatastoreTool {
    pub fn new(datastore_path: Option<&str>) -> Self {
        Self {
            datastore_path: datastore_path.map(PathBuf::from),
            cache: std::sync::RwLock::new(HashMap::new()),
        }
    }

    /// Query the SecGen datastore
    pub fn query(
        &self,
        key: &str,
        index: Option<usize>,
        field: Option<&str>,
    ) -> anyhow::Result<serde_json::Value> {
        // Try cache first
        {
            let cache = self.cache.read().map_err(|_| anyhow::anyhow!("Cache lock poisoned"))?;
            if let Some(cached) = cache.get(key) {
                return Ok(cached.clone());
            }
        }

        // Load from file
        let path = self.datastore_path.as_ref()
            .ok_or_else(|| anyhow::anyhow!("Datastore path not configured"))?;

        if !path.exists() {
            anyhow::bail!("SecGen datastore not found at {:?}", path);
        }

        let content = std::fs::read_to_string(path)?;
        let data: HashMap<String, serde_json::Value> = serde_json::from_str(&content)?;

        // Get value
        let value = data.get(key)
            .ok_or_else(|| anyhow::anyhow!("Key '{}' not found in datastore", key))?;

        // Handle array indexing
        let result = if let Some(idx) = index {
            let array = value.as_array()
                .ok_or_else(|| anyhow::anyhow!("Key '{}' is not an array", key))?;
            
            let item = array.get(idx)
                .ok_or_else(|| anyhow::anyhow!("Index {} out of bounds for '{}'", idx, key))?;

            // Handle field access
            if let Some(field_name) = field {
                let obj = item.as_object()
                    .ok_or_else(|| anyhow::anyhow!("Item at index {} is not an object", idx))?;
                
                obj.get(field_name)
                    .ok_or_else(|| anyhow::anyhow!("Field '{}' not found", field_name))?
                    .clone()
            } else {
                item.clone()
            }
        } else if let Some(field_name) = field {
            // Field access on first array element
            let array = value.as_array()
                .ok_or_else(|| anyhow::anyhow!("Key '{}' is not an array", key))?;
            
            let first = array.first()
                .ok_or_else(|| anyhow::anyhow!("Array '{}' is empty", key))?;

            let obj = first.as_object()
                .ok_or_else(|| anyhow::anyhow!("First item is not an object"))?;

            obj.get(field_name)
                .ok_or_else(|| anyhow::anyhow!("Field '{}' not found", field_name))?
                .clone()
        } else {
            value.clone()
        };

        // Cache result
        {
            let mut cache = self.cache.write().map_err(|_| anyhow::anyhow!("Cache lock poisoned"))?;
            cache.insert(key.to_string(), result.clone());
        }

        Ok(result)
    }

    /// Get all available keys in datastore
    pub fn list_keys(&self) -> anyhow::Result<Vec<String>> {
        let path = self.datastore_path.as_ref()
            .ok_or_else(|| anyhow::anyhow!("Datastore path not configured"))?;

        let content = std::fs::read_to_string(path)?;
        let data: HashMap<String, serde_json::Value> = serde_json::from_str(&content)?;

        Ok(data.keys().cloned().collect())
    }
}

impl Default for SecGenDatastoreTool {
    fn default() -> Self {
        Self::new(None)
    }
}

#[async_trait]
impl zeroclaw::tools::Tool for SecGenDatastoreTool {
    fn name(&self) -> &str {
        "secgen_datastore"
    }

    fn description(&self) -> &str {
        "Query SecGen datastore for randomized values (IPs, accounts, flags). \
         Use 'list' to see available keys, 'get' to retrieve values with optional index/field."
    }

    fn parameters_schema(&self) -> serde_json::Value {
        json!({
            "type": "object",
            "properties": {
                "action": {
                    "type": "string",
                    "enum": ["list", "get"],
                    "description": "Action: 'list' shows keys, 'get' retrieves value"
                },
                "key": {
                    "type": "string",
                    "description": "Datastore key (for 'get' action)"
                },
                "index": {
                    "type": "integer",
                    "description": "Array index (optional, for 'get')"
                },
                "field": {
                    "type": "string",
                    "description": "Object field (optional, for 'get')"
                }
            },
            "required": ["action"]
        })
    }

    async fn execute(&self, args: serde_json::Value) -> anyhow::Result<zeroclaw::tools::ToolResult> {
        let action = args
            .get("action")
            .and_then(|v| v.as_str())
            .ok_or_else(|| anyhow::anyhow!("Missing 'action' parameter"))?;

        match action {
            "list" => {
                let keys = self.list_keys()?;
                Ok(zeroclaw::tools::ToolResult {
                    success: true,
                    output: format!("Available datastore keys:\n{}", keys.join("\n")),
                    error: None,
                })
            }
            "get" => {
                let key = args
                    .get("key")
                    .and_then(|v| v.as_str())
                    .ok_or_else(|| anyhow::anyhow!("'get' requires 'key' parameter"))?;

                let index = args.get("index").and_then(|v| v.as_u64()).map(|i| i as usize);
                let field = args.get("field").and_then(|v| v.as_str());

                let value = self.query(key, index, field)?;

                Ok(zeroclaw::tools::ToolResult {
                    success: true,
                    output: format!("{}: {}", key, value),
                    error: None,
                })
            }
            _ => Ok(zeroclaw::tools::ToolResult {
                success: false,
                output: String::new(),
                error: Some(format!("Unknown action: {}", action)),
            }),
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use std::fs;
    use tempfile::TempDir;

    fn create_test_datastore(dir: &TempDir, content: &str) -> PathBuf {
        let path = dir.path().join("datastore.json");
        fs::write(&path, content).unwrap();
        path
    }

    #[test]
    fn test_query_ip_addresses() {
        let dir = TempDir::new().unwrap();
        let path = create_test_datastore(&dir, r#"{
            "IP_addresses": ["172.16.0.2", "172.16.0.3"],
            "flags": ["SEC GEN{flag1}"]
        }"#);

        let tool = SecGenDatastoreTool::new(Some(path.to_str().unwrap()));
        
        let result = tool.query("IP_addresses", Some(0), None).unwrap();
        assert_eq!(result.as_str().unwrap(), "172.16.0.2");
    }

    #[test]
    fn test_query_account_field() {
        let dir = TempDir::new().unwrap();
        let path = create_test_datastore(&dir, r#"{
            "accounts": [{"username": "student1", "password": "pass123"}]
        }"#);

        let tool = SecGenDatastoreTool::new(Some(path.to_str().unwrap()));
        
        let result = tool.query("accounts", Some(0), Some("username")).unwrap();
        assert_eq!(result.as_str().unwrap(), "student1");
    }

    #[test]
    fn test_list_keys() {
        let dir = TempDir::new().unwrap();
        let path = create_test_datastore(&dir, r#"{
            "IP_addresses": [],
            "accounts": [],
            "flags": []
        }"#);

        let tool = SecGenDatastoreTool::new(Some(path.to_str().unwrap()));
        let keys = tool.list_keys().unwrap();
        
        assert!(keys.contains(&"IP_addresses".to_string()));
        assert!(keys.contains(&"accounts".to_string()));
        assert!(keys.contains(&"flags".to_string()));
    }
}
