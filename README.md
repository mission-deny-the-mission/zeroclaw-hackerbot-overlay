# ZeroClaw Hackerbot Overlay

**Cybersecurity training bot for ZeroClaw - SecGen Hackerbot replacement**

This is an **overlay/plugin** for [ZeroClaw](https://github.com/zeroclaw-labs/zeroclaw) that adds cybersecurity training capabilities. **No fork required!**

## 🎯 Features

- ✅ **Deterministic Quiz Validation** - Fuzzy matching, NOT vulnerable to prompt injection
- ✅ **Deterministic Flag Verification** - Direct machine checking, NOT LLM-based
- ✅ **SecGen Datastore Integration** - Access randomized IPs, credentials, flags
- ✅ **Multi-Personality Support** - Red Team, Blue Team, Researcher, Instructor
- ✅ **IRC Communication** - TLS-enabled IRC for secure communication
- ✅ **Training Scenarios** - Progressive cybersecurity exercises

## 🔐 Security Architecture

**Critical**: All validation happens **OUTSIDE the LLM** in deterministic Rust code:

```
User Input → LLM (conversation) → Tool Call → Deterministic Validation → Result
                                    ↑
                                    └── NO LLM INVOLVEMENT HERE!
```

- ✅ Quiz answers: Validated with string comparison + Levenshtein distance
- ✅ Flag capture: Validated with regex matching against actual machine state
- ✅ Machine checking: Direct SSH/CLI commands, deterministic output parsing

## 🔗 SecGen Integration

This overlay is designed to integrate with **SecGen** (Security Scenario Generator):

- ✅ **SecGen datastore access** - Read randomized IPs, credentials, flags
- ✅ **Deterministic validation** - Compatible with SecGen's security model
- ✅ **Low maintenance** - ~10 hours/year vs 40+ hours for Ruby Hackerbot
- ✅ **No fork required** - ZeroClaw is a dependency

**See**: [`docs/SECGN_INTEGRATION.md`](docs/SECGN_INTEGRATION.md) for complete integration guide.

**SecGen Documentation**: [`SecGen/docs/ZEROCLOW_INTEGRATION.md`](../SecGen/docs/ZEROCLOW_INTEGRATION.md)

## 🚀 Quick Start

### Prerequisites

- Rust 1.91+ (`rustup install stable`)
- ZeroClaw dependencies (Ollama, IRC server)
- SecGen environment (for datastore access)

### Build

```bash
# Clone the overlay repository
git clone https://github.com/mission-deny-the-mission/zeroclaw-hackerbot-overlay.git
cd zeroclaw-hackerbot-overlay

# Build release binary
cargo build --release

# Binary location
./target/release/zeroclaw-hackerbot
```

### Configuration

1. **Copy default config**:
   ```bash
   cp config/hackerbot-default.toml ~/.zeroclaw/hackerbot.toml
   ```

2. **Edit for your environment**:
   ```toml
   [irc]
   server = "localhost"
   port = 6697  # TLS port
   channel = "#hackerbot"
   nickname = "Hackerbot"
   
   [secgen]
   datastore_path = "/var/lib/secgen/datastore.json"
   
   [ollama]
   host = "localhost"
   port = 11434
   model = "qwen3-vl:8b"
   ```

### Run

```bash
# Start with config
./target/release/zeroclaw-hackerbot --config ~/.zeroclaw/hackerbot.toml

# Or use environment variables
ZEROCLOW_HACKERBOT_IRC_SERVER=localhost \
ZEROCLOW_HACKERBOT_IRC_PORT=6697 \
./target/release/zeroclaw-hackerbot
```

## 📖 Usage

### IRC Commands

Connect to the IRC channel and use these commands (no slash prefix):

```
hello              - Greeting and introduction
list               - List available training scenarios
goto <N>           - Jump to scenario N
next               - Move to next scenario
previous           - Go back to previous scenario
answer <text>      - Submit quiz answer
ready              - Execute demonstration
help               - Show help
personalities      - List available personalities
switch <name>      - Change personality
```

### Example Session

```irc
<Student> hello
<Hackerbot> Welcome to Red Team training! I'm your offensive security specialist.
            Current scenario: Network Reconnaissance
            Use 'list' to see all scenarios.

<Student> list
<Hackerbot> Available scenarios:
            1. Network Reconnaissance [CURRENT]
            2. Service Enumeration
            3. Initial Access
            Progress: 0/3 completed

<Student> goto 2
<Hackerbot> Jumped to scenario 2: Service Enumeration
            Now enumerate the services running on the target...
```

## 🛠️ Tools

### Quiz Validator

Validates student quiz answers with fuzzy matching:

```rust
// Deterministic validation - NO LLM involved!
let result = quiz_validator.validate(
    user_answer: "-sS",
    correct_answer: "-sS",
    accepted_variants: vec!["SYN scan", "half-open"]
);

assert!(result.correct);
assert_eq!(result.points, 100);
```

### Flag Validator

Verifies flag capture from student machines:

```rust
// Direct machine check - deterministic!
let result = flag_validator.verify(
    target_ip: "172.16.0.2",
    flag_path: "/root/flag.txt",
    expected_pattern: r"SEC GEN\{[a-f0-9]+\}"
).await?;

assert!(result.captured);
```

### SecGen Datastore

Access SecGen randomized values:

```rust
// Query SecGen datastore
let ip = datastore.query("IP_addresses", Some(0))?;
let user = datastore.query("accounts", Some(0), Some("username"))?;
let flag = datastore.query("flags", Some(0))?;
```

## 📁 Project Structure

```
zeroclaw-hackerbot-overlay/
├── Cargo.toml              # Dependencies (ZeroClaw as library!)
├── README.md               # This file
├── src/
│   ├── main.rs             # Entry point
│   ├── lib.rs              # Library exports
│   └── tools/
│       ├── mod.rs          # Tool registry
│       ├── quiz_validator.rs    # Deterministic quiz checking
│       ├── flag_validator.rs    # Deterministic flag checking
│       ├── scenario_manager.rs  # Scenario navigation
│       └── secgen_datastore.rs  # SecGen integration
├── config/
│   └── hackerbot-default.toml  # Default configuration
├── docs/
│   ├── SECURITY.md         # Security architecture
│   ├── MAINTENANCE.md      # Maintenance guide
│   └── DEPLOYMENT.md       # Deployment guide
└── tests/
    └── integration.rs      # Integration tests
```

## 🔧 Maintenance

### Updating ZeroClaw

```bash
# Update ZeroClaw dependency
cargo update zeroclaw

# Run tests
cargo test

# Build and deploy
cargo build --release
```

**No merge conflicts!** ZeroClaw updates are automatic via Cargo.

### Adding New Features

1. **Add tool** in `src/tools/your_tool.rs`
2. **Register** in `src/tools/mod.rs`
3. **Update config** to allow new tool
4. **Test** with `cargo test`

Your code stays separate from ZeroClaw - **no conflicts!**

## 📊 Comparison: Overlay vs Fork

| Aspect | This Overlay | Traditional Fork |
|--------|-------------|------------------|
| **Setup** | `cargo add zeroclaw` | Clone + maintain fork |
| **Updates** | `cargo update` | Manual merge (4-8 hrs) |
| **Conflicts** | None | Every update |
| **Security Patches** | Automatic | Manual merge required |
| **Your Code** | Separate repo | Mixed with ZeroClaw |
| **Maintenance/Year** | ~10 hours | ~40+ hours |

## 🎯 Security Considerations

### What's Secure

- ✅ Quiz validation (deterministic string matching)
- ✅ Flag verification (direct machine access)
- ✅ Command execution (sandboxed, allowlisted)
- ✅ Datastore access (read-only, authenticated)

### What's NOT Secure

- ⚠️ LLM conversation (potential prompt injection)
- ⚠️ **Mitigation**: Critical operations use deterministic tools

### Best Practices

1. **Never trust LLM for validation** - always use tools
2. **Allowlist commands** - restrict shell access
3. **Audit tool outputs** - verify before acting
4. **Log everything** - audit trail for incidents

## 🤝 Contributing

1. Fork this repository (not ZeroClaw!)
2. Create feature branch
3. Add tests
4. Submit PR

## 📄 License

GPL-3.0 (compatible with SecGen and ZeroClaw)

## 🙏 Acknowledgments

- [ZeroClaw](https://github.com/zeroclaw-labs/zeroclaw) - AI agent platform
- [SecGen](https://github.com/cliffe/SecGen) - Original Hackerbot
- [Hacktivity](https://hacktivity.co.uk/) - Cybersecurity training platform

---

**Built with ❤️ for cybersecurity education**
