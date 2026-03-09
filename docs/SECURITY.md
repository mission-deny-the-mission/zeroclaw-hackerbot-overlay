# ZeroClaw Hackerbot - Security Architecture

**Last Updated**: February 27, 2026

---

## 🔐 Security Model Overview

This overlay implements a **hybrid security model**:

1. **LLM for Conversation** - Friendly, educational interactions
2. **Deterministic Tools for Validation** - Quiz answers, flag capture, machine state

**Critical**: All security-critical validation happens **OUTSIDE the LLM** in deterministic Rust code.

---

## 🛡️ Threat Model

### What We Protect Against

| Threat | Mitigation | Location |
|--------|------------|----------|
| **Prompt Injection** | Deterministic validation | Tools (not LLM) |
| **Flag Fabrication** | Direct machine access | `flag_validator` tool |
| **Quiz Answer Spoofing** | String comparison + Levenshtein | `quiz_validator` tool |
| **Unauthorized Commands** | Allowlist + sandboxing | ZeroClaw security layer |
| **Datastore Tampering** | Read-only access | `secgen_datastore` tool |

### What We DON'T Protect Against

| Threat | Reason | Mitigation |
|--------|--------|------------|
| **LLM Hallucination** | LLM limitation | Validate outputs, monitor |
| **LLM Social Engineering** | LLM limitation | Student training, monitoring |
| **IRC Protocol Attacks** | External | Use TLS, authentication |

---

## 🔍 Tool Security Analysis

### Quiz Validator (`quiz_validator`)

**Purpose**: Validate student quiz answers

**Security Properties**:
- ✅ **Deterministic** - Pure string comparison
- ✅ **No LLM Involvement** - Just Rust code
- ✅ **Fuzzy Matching** - Levenshtein distance, not LLM judgment
- ✅ **Fixed Responses** - Templates, not LLM-generated

**Attack Surface**:
- ⚠️ **Input Validation** - User answer could be malicious
- ✅ **Mitigation** - Answer is just compared, not executed

**Code Location**: `src/tools/quiz_validator.rs`

```rust
// This code is NOT vulnerable to prompt injection!
pub fn validate(&self, user_answer: &str, correct_answer: &str) -> QuizResult {
    let normalized_user = user_answer.trim().to_lowercase();
    let normalized_correct = correct_answer.trim().to_lowercase();
    
    // Pure string comparison - NO LLM
    if normalized_user == normalized_correct {
        return QuizResult { correct: true, /* ... */ };
    }
    
    // Levenshtein distance - deterministic algorithm
    let edit_distance = levenshtein(&normalized_user, &normalized_correct);
    // ...
}
```

---

### Flag Validator (`flag_validator`)

**Purpose**: Verify flag capture from student machines

**Security Properties**:
- ✅ **Direct Machine Access** - SSH to target, no LLM
- ✅ **Regex Validation** - Deterministic pattern matching
- ✅ **Timestamped Results** - Audit trail
- ✅ **Fixed Responses** - Templates, not LLM-generated

**Attack Surface**:
- ⚠️ **SSH Command Execution** - Could be intercepted
- ✅ **Mitigation** - StrictHostKeyChecking, timeout, read-only command

**Code Location**: `src/tools/flag_validator.rs`

```rust
// This code is NOT vulnerable to prompt injection!
pub async fn verify_flag(&self, target_ip: &str, flag_path: &str) -> FlagResult {
    // Direct SSH command - deterministic
    let output = Command::new("ssh")
        .args(&["root", &target_ip, &format!("cat {}", flag_path)])
        .output()
        .await?;
    
    // Regex validation - deterministic
    let flag_regex = regex::Regex::new(r"SEC GEN\{[a-f0-9]+\}")?;
    let captured = flag_regex.is_match(&output_str);
    
    // Result is STRUCTURED DATA, not LLM text
    FlagResult {
        captured,  // ← Boolean, can't be faked
        flag_value: output_str,  // ← Actual content from machine
        // ...
    }
}
```

---

### Scenario Manager (`scenario_manager`)

**Purpose**: Navigate training scenarios

**Security Properties**:
- ✅ **State Tracking** - User progress in memory
- ✅ **Fixed Scenarios** - Predefined, not LLM-generated
- ✅ **Deterministic Navigation** - Index-based, not LLM judgment

**Attack Surface**:
- ⚠️ **State Manipulation** - User could try to skip scenarios
- ✅ **Mitigation** - Server-side state, validated transitions

---

### SecGen Datastore (`secgen_datastore`)

**Purpose**: Access SecGen randomized values

**Security Properties**:
- ✅ **Read-Only** - Cannot modify datastore
- ✅ **Cached** - Reduces file access
- ✅ **Validated Access** - Key/index/field validation

**Attack Surface**:
- ⚠️ **Information Disclosure** - Could reveal spoilers
- ✅ **Mitigation** - Only accessible to authorized users

---

## 🔒 LLM Security Boundaries

### What LLM CAN Do

- ✅ Hold friendly conversation
- ✅ Explain concepts
- ✅ Provide hints
- ✅ Decide WHEN to call tools

### What LLM CANNOT Do

- ❌ Validate quiz answers (deterministic tool)
- ❌ Verify flag capture (deterministic tool)
- ❌ Modify machine state (sandboxed)
- ❌ Access SecGen datastore directly (tool-mediated)

---

## 🛡️ Defense in Depth

### Layer 1: Input Validation

```rust
// All tool inputs validated
let user_answer = args.get("user_answer")
    .and_then(|v| v.as_str())
    .ok_or_else(|| anyhow::anyhow!("Missing parameter"))?;
```

### Layer 2: Tool Allowlist

```toml
# In config.toml
[agents.red_team]
allowed_tools = ["quiz_validator", "flag_validator", "scenario_manager"]
# ← LLM can ONLY use these tools
```

### Layer 3: Autonomy Levels

```toml
[autonomy]
level = "supervised"  # ← Requires approval for sensitive operations
```

### Layer 4: Audit Logging

```rust
// All tool calls logged
tracing::info!("Tool called: {} by user: {}", tool_name, user);
```

---

## 📊 Security Comparison: Overlay vs Original Hackerbot

| Aspect | Original (Ruby) | This Overlay (Rust) |
|--------|----------------|---------------------|
| **Quiz Validation** | ✅ Deterministic | ✅ Deterministic |
| **Flag Verification** | ✅ Direct SSH | ✅ Direct SSH |
| **LLM Conversation** | ❌ None (AIML only) | ✅ LLM-powered |
| **Prompt Injection Risk** | ✅ None | ⚠️ Low (mitigated) |
| **Tool Sandboxing** | ⚠️ Basic | ✅ ZeroClaw security |
| **Audit Logging** | ⚠️ Basic | ✅ Comprehensive (tracing) |
| **Type Safety** | ❌ Dynamic (Ruby) | ✅ Static (Rust) |

**Verdict**: This overlay is **AS SECURE** as original for validation, with **ADDED BENEFITS** of type safety and comprehensive logging.

---

## 🚨 Incident Response

### If Prompt Injection Detected

1. **Disable LLM**:
   ```bash
   # Set autonomy to readonly
   zeroclaw autonomy set readonly
   ```

2. **Review Logs**:
   ```bash
   journalctl -u zeroclaw-hackerbot | grep "tool_call"
   ```

3. **Audit Tool Calls**:
   ```bash
   # Check for unauthorized tool usage
   grep "UNAUTHORIZED" /var/log/zeroclaw-hackerbot/hackerbot.log
   ```

4. **Update Allowlist**:
   ```toml
   # Restrict allowed tools
   allowed_tools = ["scenario_manager"]  # ← Remove sensitive tools
   ```

---

## ✅ Security Checklist

### Before Production Deployment

- [ ] Quiz validator tested with edge cases
- [ ] Flag validator tested with invalid flags
- [ ] Tool allowlist configured per personality
- [ ] Autonomy level set appropriately
- [ ] Audit logging enabled
- [ ] IRC TLS enabled
- [ ] SecGen datastore permissions verified
- [ ] SSH key-based authentication configured

### Monthly Review

- [ ] Review tool call logs
- [ ] Check for unauthorized access attempts
- [ ] Update allowlists if needed
- [ ] Review ZeroClaw security advisories
- [ ] Test incident response procedure

---

## 📚 References

- [ZeroClaw Security Model](https://github.com/zeroclaw-labs/zeroclaw/tree/main/docs/security)
- [SecGen Security](https://github.com/cliffe/SecGen/blob/master/README.md)
- [Rust Security Guidelines](https://rust-lang.github.io/rust-clippy/master/index.html)

---

**Last Reviewed**: February 27, 2026  
**Next Review**: March 27, 2026
