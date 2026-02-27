# ZeroClaw Hackerbot Overlay - SecGen Integration Guide

**Last Updated**: February 27, 2026  
**Version**: 1.0.0

---

## 🎯 Overview

This guide explains how to integrate the **ZeroClaw Hackerbot Overlay** with **SecGen** for AI-powered cybersecurity training with deterministic validation.

### What This Provides

- ✅ **AI-powered conversations** - LLM-based student interactions
- ✅ **Deterministic validation** - Quiz and flag checking NOT vulnerable to prompt injection
- ✅ **SecGen integration** - Reads randomized IPs, credentials, flags from SecGen datastore
- ✅ **No fork required** - ZeroClaw is a dependency, not a fork
- ✅ **Low maintenance** - ~10 hours/year vs 40+ hours for Ruby Hackerbot fork

### Architecture

```
SecGen VM Generation
    ↓
SecGen Datastore (JSON)
    ↓
ZeroClaw Hackerbot Overlay
    ├── quiz_validator (deterministic)
    ├── flag_validator (deterministic)
    ├── scenario_manager (deterministic)
    └── secgen_datastore (read-only)
    ↓
IRC Server ←→ Student IRC Client
```

---

## 📦 Installation

### Prerequisites

- SecGen installed and configured
- Rust 1.91+ (`curl --proto '=https' --tlsv1.2 -sSf https://sh.rustup.rs | sh`)
- IRC server (InspIRCd recommended)
- Ollama or other LLM provider

### Step 1: Clone the Overlay

```bash
cd /opt
git clone https://github.com/mission-deny-the-mission/zeroclaw-hackerbot-overlay.git
cd zeroclaw-hackerbot-overlay
```

### Step 2: Build

```bash
# Build release binary
cargo build --release

# Binary location
ls -lh target/release/zeroclaw-hackerbot
# Should be ~2.3MB
```

### Step 3: Install Binary

```bash
# Install to system path
sudo cp target/release/zeroclaw-hackerbot /usr/local/bin/

# Or keep in place and create symlink
sudo ln -s /opt/zeroclaw-hackerbot-overlay/target/release/zeroclaw-hackerbot /usr/local/bin/zeroclaw-hackerbot
```

### Step 4: Configure

```bash
# Create config directory
sudo mkdir -p /etc/zeroclaw-hackerbot

# Copy default config
cp config/hackerbot-default.toml /etc/zeroclaw-hackerbot/hackerbot.toml

# Edit for SecGen integration
sudo nano /etc/zeroclaw-hackerbot/hackerbot.toml
```

**Minimum SecGen configuration**:
```toml
[irc]
server = "localhost"
port = 6697
channel = "#hackerbot"
nickname = "Hackerbot"

[secgen]
datastore_path = "/var/lib/secgen/datastore.json"

[ollama]
host = "localhost"
port = 11434
model = "qwen3-vl:8b"
```

---

## 🔧 SecGen Integration

### How SecGen Generates Hackerbot Configs

SecGen's Hackerbot module generates:
1. **VM configuration** with randomized values
2. **Datastore JSON** with IPs, credentials, flags
3. **Lab sheets** with instructions

The overlay reads the **datastore JSON** to access these values.

### SecGen Datastore Format

SecGen generates a JSON file like:

```json
{
  "IP_addresses": ["172.16.0.2", "172.16.0.3", "172.16.0.4"],
  "accounts": [
    {"username": "student1", "password": "randompass123"},
    {"username": "admin", "password": "adminpass456"}
  ],
  "flags": [
    "SEC GEN{abc123def456}",
    "SEC GEN{789ghi012jkl}"
  ],
  "spoiler_admin_pass": "rootpass789"
}
```

The overlay's `secgen_datastore` tool can query this:

```
User: What's the target IP?
LLM calls: secgen_datastore(action="get", key="IP_addresses", index=0)
Result: "172.16.0.2"
```

### Puppet Integration

To deploy the overlay via SecGen's Puppet module:

1. **Add Puppet module** to SecGen:
   ```
   SecGen/modules/utilities/unix/zeroclaw_overlay/
   ├── manifests/
   │   ├── init.pp
   │   ├── install.pp
   │   ├── config.pp
   │   └── service.pp
   └── files/
       └── zeroclaw-hackerbot
   ```

2. **Update SecGen scenario** to include overlay:
   ```xml
   <utility module_path=".*/zeroclaw_overlay">
     <input into="irc_server_ip">
       <datastore access="2">IP_addresses</datastore>
     </input>
     <input into="hackerbot_configs">
       <datastore>hackerbot_instructions</datastore>
     </input>
   </utility>
   ```

---

## 🧪 Testing with SecGen VMs

### Test Scenario: hacker_vs_hackerbot_1

1. **Generate SecGen VM**:
   ```bash
   cd SecGen
   ruby secgen.rb run \
     --scenario scenarios/labs/response_and_investigation/hacker_vs_hackerbot_1.xml \
     --project hackerbot_test
   ```

2. **Start Overlay**:
   ```bash
   zeroclaw-hackerbot --config /etc/zeroclaw-hackerbot/hackerbot.toml
   ```

3. **Connect with IRC Client**:
   ```bash
   irssi -c localhost -p 6697 --tls --tls-noverify
   /join #hackerbot
   hello
   list
   ```

4. **Test Quiz Validation**:
   ```
   <student> answer -sS
   <Hackerbot> Correct! (similarity: 100.0%, edit distance: 0)
   ```

5. **Test Flag Validation**:
   ```
   <student> verify flag on 172.16.0.2
   <Hackerbot> Flag successfully captured! Flag: SEC GEN{abc123def456}
   ```

---

## 📊 Comparison: Ruby Hackerbot vs ZeroClaw Overlay

| Feature | Ruby Hackerbot | ZeroClaw Overlay |
|---------|---------------|------------------|
| **Language** | Ruby | Rust |
| **Binary Size** | Script + gems | 2.3MB single binary |
| **Memory Usage** | ~100MB | ~20MB |
| **Startup Time** | ~10s | ~0.1s |
| **LLM Support** | AIML pattern matching | Modern LLM (Ollama, etc.) |
| **Quiz Validation** | ✅ Deterministic | ✅ Deterministic |
| **Flag Validation** | ✅ Direct SSH | ✅ Direct SSH |
| **Maintenance** | ~40 hrs/year | ~10 hrs/year |
| **Updates** | Manual merge | `cargo update` |
| **Security** | Basic | ZeroClaw security model |

---

## 🔐 Security Considerations

### What's Secure

- ✅ **Quiz validation** - Deterministic string comparison (NOT LLM)
- ✅ **Flag validation** - Direct SSH + regex (NOT LLM)
- ✅ **Datastore access** - Read-only (NOT modifiable)
- ✅ **Tool allowlists** - Configured per personality

### What Requires Monitoring

- ⚠️ **LLM conversations** - Potential for hallucination
- ⚠️ **IRC communication** - Use TLS, authentication
- ⚠️ **SSH commands** - Sandboxed, allowlisted

### Best Practices

1. **Never trust LLM for validation** - Always use deterministic tools
2. **Use TLS for IRC** - Port 6697 with certificate verification
3. **Restrict tool access** - Use personality-based allowlists
4. **Log everything** - Audit trail for incidents
5. **Regular updates** - `cargo update zeroclaw` monthly

---

## 🚀 Deployment Options

### Option 1: Standalone Deployment

Deploy overlay separately from SecGen:

```bash
# Install on training server
sudo cp target/release/zeroclaw-hackerbot /usr/local/bin/

# Configure
sudo nano /etc/zeroclaw-hackerbot/hackerbot.toml

# Start as service
sudo systemctl enable zeroclaw-hackerbot
sudo systemctl start zeroclaw-hackerbot
```

**Pros**: Simple, independent of SecGen updates  
**Cons**: Manual coordination with SecGen VMs

### Option 2: Integrated with SecGen Puppet

Include overlay in SecGen's Puppet deployment:

```puppet
# In SecGen Puppet module
include zeroclaw_overlay::install
include zeroclaw_overlay::config
include zeroclaw_overlay::service
```

**Pros**: Automatic deployment with SecGen VMs  
**Cons**: Requires Puppet module development

### Option 3: Docker Deployment

Containerized deployment:

```dockerfile
FROM rust:1.91 as builder
WORKDIR /app
COPY . .
RUN cargo build --release

FROM debian:bookworm-slim
RUN apt-get update && apt-get install -y openssh-client
COPY --from=builder /app/target/release/zeroclaw-hackerbot /usr/local/bin/
CMD ["zeroclaw-hackerbot", "--config", "/etc/hackerbot.toml"]
```

**Pros**: Consistent environment, easy scaling  
**Cons**: Docker infrastructure required

---

## 📖 Migration from Ruby Hackerbot

### Step-by-Step Migration

1. **Document current Ruby Hackerbot setup**:
   ```bash
   # List current configs
   ls -la /opt/hackerbot/config/
   
   # Document IRC channels
   cat /opt/hackerbot/config/*.xml | grep -A5 "<channels>"
   ```

2. **Install overlay side-by-side**:
   ```bash
   # Install to different path
   sudo cp target/release/zeroclaw-hackerbot /usr/local/bin/zeroclaw-hackerbot-new
   
   # Use different IRC port initially
   # Ruby: 6667, Overlay: 6697
   ```

3. **Test with subset of students**:
   ```bash
   # Point test students to new bot
   irssi -c localhost -p 6697
   ```

4. **Validate functionality**:
   - [ ] Quiz answers validate correctly
   - [ ] Flag capture works
   - [ ] Scenario navigation works
   - [ ] All personalities available

5. **Switch production traffic**:
   ```bash
   # Update student documentation with new IRC port
   # Or configure overlay to use port 6667 (if stopping Ruby bot)
   ```

6. **Monitor and rollback if needed**:
   ```bash
   # Monitor logs
   journalctl -u zeroclaw-hackerbot -f
   
   # Rollback to Ruby bot if issues
   sudo systemctl stop zeroclaw-hackerbot
   sudo systemctl start hackerbot
   ```

---

## 🆘 Troubleshooting

### Overlay Won't Start

```bash
# Check binary
which zeroclaw-hackerbot
zeroclaw-hackerbot --version

# Check config
zeroclaw-hackerbot --config /etc/zeroclaw-hackerbot/hackerbot.toml --verbose

# Check logs
journalctl -u zeroclaw-hackerbot -n 100
```

### Can't Access SecGen Datastore

```bash
# Check datastore exists
ls -la /var/lib/secgen/datastore.json

# Check permissions
sudo chmod 644 /var/lib/secgen/datastore.json

# Validate JSON
jq . /var/lib/secgen/datastore.json
```

### Quiz Validation Not Working

```bash
# Enable debug logging
RUST_LOG=debug zeroclaw-hackerbot --config /etc/zeroclaw-hackerbot/hackerbot.toml

# Check tool is loaded
# Look for: "Loaded 4 Hackerbot tools" in logs

# Check tool calls
grep "quiz_validator" /var/log/zeroclaw-hackerbot/hackerbot.log
```

---

## 📚 Additional Resources

- **Overlay Repository**: https://github.com/mission-deny-the-mission/zeroclaw-hackerbot-overlay
- **ZeroClaw Documentation**: https://github.com/openagen/zeroclaw/tree/main/docs
- **SecGen Documentation**: https://github.com/cliffe/SecGen/blob/master/README.md
- **Hacktivity Platform**: https://hacktivity.co.uk/

---

## 🤝 Contributing

Found issues with SecGen integration? Please contribute:

1. Fork the overlay repository
2. Create `secgen/` branch
3. Add SecGen-specific features
4. Submit PR with testing notes

---

**Last Reviewed**: February 27, 2026  
**Next Review**: After first SecGen VM deployment
