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

    // Initialize ZeroClaw with Hackerbot tools
    let mut additional_tools = init_tools();
    tracing::info!("Loaded {} Hackerbot tools", additional_tools.len());

    // Start ZeroClaw channel server with our tools
    // This delegates to ZeroClaw's channel start logic
    start_channel_server(config, &mut additional_tools).await?;

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
        Config::default()
    };

    // Override with CLI args
    Ok(Config {
        irc_server: args.irc_server.clone().unwrap_or(config.irc_server),
        irc_port: args.irc_port,
        ..config
    })
}

/// Start the IRC channel server
async fn start_channel_server(config: Config, tools: &mut Vec<Box<dyn zeroclaw::tools::Tool>>) -> Result<()> {
    use zeroclaw::channels::irc::{IrcChannel, IrcChannelConfig};
    use zeroclaw::channels::Channel;
    use tokio::sync::mpsc;

    tracing::info!("Connecting to IRC server {}:{}", config.irc_server, config.irc_port);

    // Create IRC channel
    let irc_config = IrcChannelConfig {
        server: config.irc_server.clone(),
        port: config.irc_port,
        nickname: config.irc_nickname,
        username: Some(config.irc_nickname.clone()),
        channels: vec![config.irc_channel.clone()],
        allowed_users: config.allowed_users.clone(),
        server_password: None,
        nickserv_password: None,
        sasl_password: None,
        verify_tls: false, // Accept self-signed certs for local testing
    };

    let irc_channel = IrcChannel::new(irc_config);

    // Create message channel
    let (tx, mut rx) = mpsc::channel(100);

    // Start listening for messages
    let listen_handle = tokio::spawn(async move {
        tracing::info!("IRC channel listening for messages...");
        
        // This would normally integrate with ZeroClaw's agent loop
        // For now, we'll just log received messages
        while let Some(msg) = rx.recv().await {
            tracing::info!("Received message from {}: {}", msg.sender, msg.content);
            
            // Process message with LLM + tools
            // This is where ZeroClaw's agent loop would be invoked
            process_message(msg, tools).await?;
        }

        Ok::<(), anyhow::Error>(())
    });

    // Start IRC connection
    irc_channel.listen(tx).await?;

    // Wait for listener
    listen_handle.await??;

    Ok(())
}

/// Process a message with the LLM and tools
async fn process_message(
    msg: zeroclaw::channels::ChannelMessage,
    tools: &[Box<dyn zeroclaw::tools::Tool>],
) -> Result<()> {
    // This is where ZeroClaw's agent loop would be invoked
    // For the overlay, we delegate to ZeroClaw's agent processing
    
    tracing::debug!("Processing message: {:?}", msg);
    
    // Tool selection and execution would happen here via ZeroClaw's agent
    // The tools are registered and available for the LLM to call
    
    Ok(())
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
