//! ZeroClaw Hackerbot - Main Entry Point
//!
//! This binary wraps ZeroClaw with Hackerbot-specific tools and configuration.

use anyhow::Result;
use clap::Parser;
use std::path::PathBuf;
use tracing_subscriber::{fmt, EnvFilter};

use zeroclaw_hackerbot::{init_tools, VERSION};

/// ZeroClaw Hackerbot - Cybersecurity Training Bot
#[derive(Parser, Debug)]
#[command(name = "zeroclaw-hackerbot")]
#[command(author = "Hackerbot Team")]
#[command(version = VERSION)]
#[command(about = "Cybersecurity training bot for ZeroClaw", long_about = None)]
struct Args {
    /// Configuration file path
    #[arg(short, long, default_value = "~/.zeroclaw/hackerbot.toml")]
    config: PathBuf,

    /// IRC server address
    #[arg(long, env = "ZEROCLOW_HACKERBOT_IRC_SERVER")]
    irc_server: Option<String>,

    /// IRC server port
    #[arg(long, env = "ZEROCLOW_HACKERBOT_IRC_PORT", default_value = "6697")]
    irc_port: u16,

    /// Enable verbose logging
    #[arg(short, long)]
    verbose: bool,
}

#[tokio::main]
async fn main() -> Result<()> {
    // Initialize logging
    let args = Args::parse();
    
    let log_level = if args.verbose { "debug" } else { "info" };
    tracing_subscriber::fmt()
        .with_env_filter(EnvFilter::from_default_env().add_directive(log_level.parse()?))
        .init();

    tracing::info!("ZeroClaw Hackerbot v{}", VERSION);
    tracing::info!("Starting cybersecurity training bot...");

    // Load configuration
    let config = load_config(&args)?;
    tracing::info!("Configuration loaded from {:?}", args.config);

    // Initialize Hackerbot tools
    let tools = init_tools(config.secgen_datastore_path.as_deref());
    tracing::info!("Loaded {} Hackerbot tools", tools.len());

    // Log tool names
    for tool in &tools {
        tracing::info!("  - Tool: {}", tool.name());
    }

    tracing::info!("");
    tracing::info!("Hackerbot tools initialized successfully!");
    tracing::info!("");
    tracing::info!("NOTE: This overlay provides tools for ZeroClaw.");
    tracing::info!("To use these tools, start ZeroClaw with the --config option");
    tracing::info!("pointing to your hackerbot.toml configuration file.");
    tracing::info!("");
    tracing::info!("Example:");
    tracing::info!("  zeroclaw channel start --config ~/.zeroclaw/hackerbot.toml");
    tracing::info!("");

    Ok(())
}

/// Load configuration from file and environment
fn load_config(args: &Args) -> Result<Config> {
    // Try to load from file
    let config_path = shellexpand::tilde(&args.config.to_string_lossy()).to_string();
    
    let config = if std::path::Path::new(&config_path).exists() {
        // Load from TOML file
        let content = std::fs::read_to_string(&config_path)?;
        toml::from_str(&content)?
    } else {
        // Use defaults
        tracing::warn!("Config file not found at {:?}, using defaults", config_path);
        Config::default()
    };

    // Override with CLI args
    Ok(Config {
        irc_server: args.irc_server.clone().unwrap_or(config.irc_server),
        irc_port: args.irc_port,
        ..config
    })
}

/// Configuration structure
#[derive(Debug, Clone, serde::Deserialize)]
struct Config {
    #[serde(default = "default_irc_server")]
    irc_server: String,
    
    #[serde(default = "default_irc_port")]
    irc_port: u16,
    
    #[serde(default = "default_irc_nickname")]
    irc_nickname: String,
    
    #[serde(default = "default_irc_channel")]
    irc_channel: String,
    
    #[serde(default)]
    allowed_users: Vec<String>,
    
    #[serde(default)]
    secgen_datastore_path: Option<String>,
    
    #[serde(default)]
    ollama_host: String,
    
    #[serde(default)]
    ollama_port: u16,
    
    #[serde(default = "default_model")]
    model: String,
}

impl Default for Config {
    fn default() -> Self {
        Self {
            irc_server: default_irc_server(),
            irc_port: default_irc_port(),
            irc_nickname: default_irc_nickname(),
            irc_channel: default_irc_channel(),
            allowed_users: vec!["*".to_string()],
            secgen_datastore_path: None,
            ollama_host: "localhost".to_string(),
            ollama_port: 11434,
            model: default_model(),
        }
    }
}

fn default_irc_server() -> String { "localhost".to_string() }
fn default_irc_port() -> u16 { 6697 }
fn default_irc_nickname() -> String { "Hackerbot".to_string() }
fn default_irc_channel() -> String { "#hackerbot".to_string() }
fn default_model() -> String { "qwen3-vl:8b".to_string() }
