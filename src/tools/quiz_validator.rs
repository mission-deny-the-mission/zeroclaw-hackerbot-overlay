//! Quiz Validator Tool - Deterministic Answer Validation
//!
//! This tool validates student quiz answers using deterministic string comparison
//! and fuzzy matching. **NOT vulnerable to prompt injection** - runs as pure Rust code.

use async_trait::async_trait;
use serde::{Deserialize, Serialize};
use serde_json::json;
use strsim::levenshtein;

/// Quiz validator tool for deterministic answer validation
pub struct QuizValidatorTool {
    /// Minimum similarity threshold (0.0 - 1.0)
    similarity_threshold: f64,
    /// Maximum Levenshtein distance for fuzzy matching
    max_edit_distance: usize,
}

/// Quiz validation result
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct QuizResult {
    pub correct: bool,
    pub message: String,
    pub points: u32,
    pub similarity: f64,
    pub edit_distance: usize,
}

impl QuizValidatorTool {
    pub fn new() -> Self {
        Self {
            similarity_threshold: 0.8,
            max_edit_distance: 3,
        }
    }

    /// Validate a quiz answer deterministically
    pub fn validate(&self, user_answer: &str, correct_answer: &str, accepted_variants: &[&str]) -> QuizResult {
        let normalized_user = user_answer.trim().to_lowercase();
        let normalized_correct = correct_answer.trim().to_lowercase();

        // Exact match
        if normalized_user == normalized_correct {
            return QuizResult {
                correct: true,
                message: "Correct! Well done.".to_string(),
                points: 100,
                similarity: 1.0,
                edit_distance: 0,
            };
        }

        // Check accepted variants
        for variant in accepted_variants {
            if normalized_user == variant.to_lowercase() {
                return QuizResult {
                    correct: true,
                    message: "Correct! (Accepted variant)".to_string(),
                    points: 100,
                    similarity: 1.0,
                    edit_distance: 0,
                };
            }
        }

        // Fuzzy matching
        let edit_distance = levenshtein(&normalized_user, &normalized_correct);
        let max_len = normalized_user.len().max(normalized_correct.len());
        let similarity = if max_len == 0 {
            1.0
        } else {
            1.0 - (edit_distance as f64 / max_len as f64)
        };

        // Accept if similarity is high enough OR edit distance is small
        if similarity >= self.similarity_threshold || edit_distance <= self.max_edit_distance {
            QuizResult {
                correct: true,
                message: "Correct! (Fuzzy match accepted)".to_string(),
                points: 100,
                similarity,
                edit_distance,
            }
        } else {
            QuizResult {
                correct: false,
                message: "Incorrect. Try again or ask for a hint.".to_string(),
                points: 0,
                similarity,
                edit_distance,
            }
        }
    }
}

impl Default for QuizValidatorTool {
    fn default() -> Self {
        Self::new()
    }
}

#[async_trait]
impl zeroclaw::tools::Tool for QuizValidatorTool {
    fn name(&self) -> &str {
        "quiz_validator"
    }

    fn description(&self) -> &str {
        "Validate quiz answers with deterministic fuzzy matching. \
         Use this to check if a student's answer is correct. \
         NOT vulnerable to prompt injection - uses pure string comparison."
    }

    fn parameters_schema(&self) -> serde_json::Value {
        json!({
            "type": "object",
            "properties": {
                "user_answer": {
                    "type": "string",
                    "description": "The student's answer to validate"
                },
                "correct_answer": {
                    "type": "string",
                    "description": "The correct answer"
                },
                "accepted_variants": {
                    "type": "array",
                    "items": {"type": "string"},
                    "description": "Alternative acceptable answers"
                }
            },
            "required": ["user_answer", "correct_answer"]
        })
    }

    async fn execute(&self, args: serde_json::Value) -> anyhow::Result<zeroclaw::tools::ToolResult> {
        let user_answer = args
            .get("user_answer")
            .and_then(|v| v.as_str())
            .ok_or_else(|| anyhow::anyhow!("Missing 'user_answer' parameter"))?;

        let correct_answer = args
            .get("correct_answer")
            .and_then(|v| v.as_str())
            .ok_or_else(|| anyhow::anyhow!("Missing 'correct_answer' parameter"))?;

        let accepted_variants: Vec<&str> = args
            .get("accepted_variants")
            .and_then(|v| v.as_array())
            .map(|arr| arr.iter().filter_map(|v| v.as_str()).collect())
            .unwrap_or_default();

        let result = self.validate(user_answer, correct_answer, &accepted_variants);

        Ok(zeroclaw::tools::ToolResult {
            success: result.correct,
            output: format!(
                "{} (similarity: {:.1}%, edit distance: {})",
                result.message,
                result.similarity * 100.0,
                result.edit_distance
            ),
            error: None,
        })
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_exact_match() {
        let validator = QuizValidatorTool::new();
        let result = validator.validate("-sS", "-sS", &[]);
        
        assert!(result.correct);
        assert_eq!(result.similarity, 1.0);
        assert_eq!(result.edit_distance, 0);
    }

    #[test]
    fn test_case_insensitive() {
        let validator = QuizValidatorTool::new();
        let result = validator.validate("-ss", "-sS", &[]);
        
        assert!(result.correct);
        assert_eq!(result.similarity, 1.0);
    }

    #[test]
    fn test_accepted_variant() {
        let validator = QuizValidatorTool::new();
        let result = validator.validate(
            "SYN scan",
            "-sS",
            &["SYN scan", "half-open scan"]
        );
        
        assert!(result.correct);
    }

    #[test]
    fn test_fuzzy_match() {
        let validator = QuizValidatorTool::new();
        let result = validator.validate("-sS scan", "-sS", &[]);
        
        // Should accept due to small edit distance
        assert!(result.correct);
        assert!(result.edit_distance > 0);
    }

    #[test]
    fn test_incorrect_answer() {
        let validator = QuizValidatorTool::new();
        let result = validator.validate("completely wrong", "-sS", &[]);
        
        assert!(!result.correct);
        assert_eq!(result.points, 0);
    }
}
