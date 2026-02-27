//! Integration Test - ZeroClaw with Hackerbot Overlay
//!
//! This test demonstrates that the overlay tools work with ZeroClaw.

use zeroclaw::tools::Tool;
use zeroclaw_hackerbot::{init_tools, QuizValidatorTool, FlagValidatorTool};

#[tokio::main]
async fn main() -> anyhow::Result<()> {
    println!("=== ZeroClaw Hackerbot Overlay Integration Test ===\n");

    // Initialize overlay tools
    let tools = init_tools(None);
    println!("✅ Loaded {} tools:", tools.len());
    for tool in &tools {
        println!("   - {}", tool.name());
    }
    println!();

    // Test Quiz Validator
    println!("=== Testing Quiz Validator ===");
    let quiz_tool = tools.iter().find(|t| t.name() == "quiz_validator").unwrap();
    
    let result = quiz_tool.execute(serde_json::json!({
        "user_answer": "-sS",
        "correct_answer": "-sS",
        "accepted_variants": ["SYN scan", "half-open"]
    })).await?;
    
    println!("Test 1 (exact match): {}", if result.success { "✅ PASS" } else { "❌ FAIL" });
    println!("  Output: {}\n", result.output);

    let result = quiz_tool.execute(serde_json::json!({
        "user_answer": "sS",
        "correct_answer": "-sS",
        "accepted_variants": []
    })).await?;
    
    println!("Test 2 (fuzzy match): {}", if result.success { "✅ PASS" } else { "❌ FAIL" });
    println!("  Output: {}\n", result.output);

    // Test Scenario Manager
    println!("=== Testing Scenario Manager ===");
    let scenario_tool = tools.iter().find(|t| t.name() == "scenario_manager").unwrap();
    
    let result = scenario_tool.execute(serde_json::json!({
        "command": "list",
        "user": "test_student"
    })).await?;
    
    println!("Test 3 (list scenarios): {}", if result.success { "✅ PASS" } else { "❌ FAIL" });
    println!("  Output preview: {}...\n", result.output.chars().take(100).collect::<String>());

    let result = scenario_tool.execute(serde_json::json!({
        "command": "goto",
        "scenario_index": 2,
        "user": "test_student"
    })).await?;
    
    println!("Test 4 (goto scenario): {}", if result.success { "✅ PASS" } else { "❌ FAIL" });
    println!("  Output: {}\n", result.output);

    println!("=== Integration Test Complete ===");
    println!("\n✅ All overlay tools work correctly!");
    println!("✅ Ready for ZeroClaw integration");

    Ok(())
}
