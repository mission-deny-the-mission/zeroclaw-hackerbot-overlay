# ZeroClaw Hackerbot Overlay - Maintenance Guide

**Last Updated**: February 27, 2026  
**Maintenance Model**: Overlay/Extension (NO FORK)

---

## 📊 Maintenance Overview

This overlay uses ZeroClaw as a **library dependency**, not a fork. This means:

- ✅ **ZeroClaw updates are automatic** via `cargo update`
- ✅ **No merge conflicts** - your code stays separate
- ✅ **Security patches inherited automatically** from ZeroClaw
- ✅ **Your code is independent** - version and release separately

---

## 🔄 Updating ZeroClaw

### Regular Updates (Monthly)

```bash
# Navigate to overlay directory
cd zeroclaw-hackerbot-overlay

# Update ZeroClaw dependency
cargo update zeroclaw

# Review what changed
cargo update zeroclaw --dry-run

# Build and test
cargo build --release
cargo test

# Deploy if tests pass
sudo systemctl restart zeroclaw-hackerbot
```

**Time Required**: 5-10 minutes  
**Risk**: Low (ZeroClaw has comprehensive tests)

---

### Breaking API Changes (1-2 times per year)

When ZeroClaw makes breaking changes:

```bash
# Update
cargo update zeroclaw

# Build will fail with clear error messages
cargo build 2>&1 | grep "error"

# Fix your code based on compiler errors
# Example: if Tool trait changed:
# - Update trait implementation
# - Fix method signatures

# Test thoroughly
cargo test --all

# Deploy
cargo build --release
sudo systemctl restart zeroclaw-hackerbot
```

**Time Required**: 2-8 hours (depending on change scope)  
**Risk**: Medium (test thoroughly before deploying)

---

## 🛠️ Adding New Features

### Adding a New Tool

1. **Create tool file**:
   ```bash
   touch src/tools/your_new_tool.rs
   ```

2. **Implement Tool trait**:
   ```rust
   use async_trait::async_trait;
   
   pub struct YourTool { /* ... */ }
   
   #[async_trait]
   impl zeroclaw::tools::Tool for YourTool {
       fn name(&self) -> &str { "your_tool" }
       fn description(&self) -> &str { "..." }
       fn parameters_schema(&self) -> serde_json::Value { /* ... */ }
       async fn execute(&self, args: serde_json::Value) -> anyhow::Result<zeroclaw::tools::ToolResult> {
           /* Your implementation */
       }
   }
   ```

3. **Register in mod.rs**:
   ```rust
   pub mod your_new_tool;
   pub use your_new_tool::YourTool;
   ```

4. **Add to init function**:
   ```rust
   pub fn init_tools() -> Vec<Box<dyn zeroclaw::tools::Tool>> {
       vec![
           // ... existing tools ...
           Box::new(YourTool::new()),
       ]
   }
   ```

5. **Test**:
   ```bash
   cargo test your_new_tool
   cargo build --release
   ```

**Time Required**: 1-4 hours  
**Risk**: Low (isolated to your code)

---

## 🧪 Testing Strategy

### Before Any Update

```bash
# Run all tests
cargo test

# Run with coverage (optional)
cargo tarpaulin --out Html

# Build release
cargo build --release

# Integration test (if you have SecGen environment)
./target/release/zeroclaw-hackerbot --config test-config.toml
```

### Test Checklist

- [ ] All unit tests pass
- [ ] Quiz validator tests pass
- [ ] Flag validator tests pass
- [ ] Scenario manager tests pass
- [ ] SecGen datastore tests pass
- [ ] Release build succeeds
- [ ] Binary size is acceptable (< 20MB)

---

## 📦 Deployment

### Production Deployment

```bash
# Build release
cargo build --release

# Stop service
sudo systemctl stop zeroclaw-hackerbot

# Backup current binary
sudo cp /usr/local/bin/zeroclaw-hackerbot /usr/local/bin/zeroclaw-hackerbot.bak

# Install new binary
sudo cp target/release/zeroclaw-hackerbot /usr/local/bin/

# Restart service
sudo systemctl start zeroclaw-hackerbot

# Check status
sudo systemctl status zeroclaw-hackerbot

# Monitor logs
journalctl -u zeroclaw-hackerbot -f
```

### Rollback (if needed)

```bash
# Stop service
sudo systemctl stop zeroclaw-hackerbot

# Restore backup
sudo cp /usr/local/bin/zeroclaw-hackerbot.bak /usr/local/bin/zeroclaw-hackerbot

# Restart
sudo systemctl start zeroclaw-hackerbot
```

---

## 🔍 Monitoring

### Health Checks

```bash
# Check service status
systemctl status zeroclaw-hackerbot

# Check logs
journalctl -u zeroclaw-hackerbot --since "1 hour ago"

# Check IRC connection
echo "PING test" | nc localhost 6697

# Check Ollama connection
curl http://localhost:11434/api/tags
```

### Alert Conditions

Monitor for:
- ❌ Service not running
- ❌ High memory usage (> 100MB)
- ❌ IRC connection failures
- ❌ Ollama connection failures
- ❌ SecGen datastore access errors

---

## 🐛 Troubleshooting

### Build Failures After Update

```bash
# See what changed
cargo update zeroclaw --dry-run

# Check ZeroClaw changelog
# https://github.com/openagen/zeroclaw/blob/main/CHANGELOG.md

# Fix compiler errors
# (Compiler will tell you exactly what broke)
cargo build 2>&1 | grep "error"
```

### Runtime Errors

```bash
# Check logs
journalctl -u zeroclaw-hackerbot -n 100

# Enable debug logging
RUST_LOG=debug zeroclaw-hackerbot

# Check dependencies
systemctl status ollama
systemctl status inspircd
```

### SecGen Datastore Issues

```bash
# Check datastore exists
ls -la /var/lib/secgen/datastore.json

# Check permissions
sudo chmod 644 /var/lib/secgen/datastore.json

# Validate JSON
jq . /var/lib/secgen/datastore.json
```

---

## 📅 Maintenance Schedule

### Weekly
- [ ] Check logs for errors
- [ ] Monitor resource usage

### Monthly
- [ ] Run `cargo update zeroclaw`
- [ ] Review ZeroClaw changelog
- [ ] Run full test suite

### Quarterly
- [ ] Review security advisories
- [ ] Update dependencies
- [ ] Review and update documentation

### Annually
- [ ] Major version upgrade planning
- [ ] Architecture review
- [ ] Performance optimization

---

## 🆘 Getting Help

### Resources
- **ZeroClaw Docs**: https://github.com/openagen/zeroclaw/tree/main/docs
- **ZeroClaw Issues**: https://github.com/openagen/zeroclaw/issues
- **This Overlay Issues**: [Your repo issues]

### When to Open Issue
- ZeroClaw API breaks without deprecation warning
- Security vulnerability in ZeroClaw
- Feature request for ZeroClaw core

### When NOT to Open Issue
- Your tool has bugs (fix in your code)
- Configuration issues (check docs)
- Deployment issues (check deployment guide)

---

## 💡 Best Practices

### DO
- ✅ Run tests before deploying
- ✅ Keep ZeroClaw updated monthly
- ✅ Monitor logs regularly
- ✅ Backup before upgrades
- ✅ Use release builds in production

### DON'T
- ❌ Modify ZeroClaw source directly
- ❌ Skip testing after updates
- ❌ Deploy without backup
- ❌ Ignore deprecation warnings
- ❌ Run debug builds in production

---

## 📊 Comparison: This Overlay vs Fork

| Task | This Overlay | Traditional Fork |
|------|-------------|------------------|
| **Monthly Update** | 5 min (`cargo update`) | 4-8 hours (merge conflicts) |
| **Security Patch** | Automatic | Manual merge required |
| **Adding Features** | Your code only | Risk of conflicts |
| **Testing** | Your tools only | Full regression |
| **Annual Maintenance** | ~10 hours | ~40+ hours |

---

**Last Reviewed**: February 27, 2026  
**Next Review**: March 27, 2026
