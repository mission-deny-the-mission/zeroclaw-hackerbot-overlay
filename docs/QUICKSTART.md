# ZeroClaw Hackerbot Overlay - Quick Start Guide

**Get up and running in 5 minutes!**

---

## 🚀 Prerequisites

- ✅ Rust 1.91+ (`curl --proto '=https' --tlsv1.2 -sSf https://sh.rustup.rs | sh`)
- ✅ Ollama running (`ollama serve`)
- ✅ IRC server (Docker: `docker run -d -p 6697:6697 inspircd/inspircd-docker`)

---

## 📦 Build

```bash
# Clone the overlay
cd zeroclaw-hackerbot-overlay

# Build (uses build script)
./build.sh

# Or manually
cargo build --release
```

Binary will be at: `target/release/zeroclaw-hackerbot`

---

## ⚙️ Configure

```bash
# Create config directory
mkdir -p ~/.zeroclaw

# Copy default config
cp config/hackerbot-default.toml ~/.zeroclaw/hackerbot.toml

# Edit for your environment
nano ~/.zeroclaw/hackerbot.toml
```

**Minimum changes**:
```toml
[irc]
server = "localhost"     # Your IRC server
port = 6697              # TLS port

[secgen]
datastore_path = "/var/lib/secgen/datastore.json"  # If using SecGen
```

---

## ▶️ Run

```bash
# Start the bot
./target/release/zeroclaw-hackerbot --config ~/.zeroclaw/hackerbot.toml
```

You should see:
```
ZeroClaw Hackerbot v1.0.0
Starting cybersecurity training bot...
Configuration loaded from /home/user/.zeroclaw/hackerbot.toml
Loaded 4 Hackerbot tools
Connecting to IRC server localhost:6697
IRC channel listening for messages...
```

---

## 🧪 Test

Connect with an IRC client:

```bash
# Install irssi if needed
sudo apt install irssi

# Connect
irssi -c localhost -p 6697 --tls --tls-noverify

# Join channel
/join #hackerbot

# Test commands
hello
list
goto 1
help
```

Expected responses:
```
<hackerbot> Welcome to Red Team training!
<hackerbot> Available cybersecurity training scenarios:
            1. Initial Reconnaissance [CURRENT]
            2. Service Enumeration
            3. Initial Access
```

---

## 🔧 Troubleshooting

### "Connection refused" on port 6697

```bash
# Check IRC server is running
docker ps | grep inspircd

# Start if needed
docker run -d -p 6697:6697 inspircd/inspircd-docker
```

### "Ollama connection failed"

```bash
# Check Ollama is running
ollama ps

# Start if needed
ollama serve

# Pull model if missing
ollama pull qwen3-vl:8b
```

### "SecGen datastore not found"

```bash
# Check if using SecGen
grep datastore_path ~/.zeroclaw/hackerbot.toml

# If not using SecGen, comment it out or set to empty
# datastore_path = ""
```

---

## 📖 Next Steps

1. **Read the full README**: `README.md`
2. **Review security model**: `docs/SECURITY.md`
3. **Check maintenance guide**: `docs/MAINTENANCE.md`
4. **Customize scenarios**: Edit `src/tools/scenario_manager.rs`

---

## 🆘 Help

- **Issues**: [GitHub Issues](https://github.com/mission-deny-the-mission/zeroclaw-hackerbot-overlay/issues)
- **Docs**: `docs/` directory
- **ZeroClaw**: https://github.com/zeroclaw-labs/zeroclaw

---

**Happy hacking! 🎯**
