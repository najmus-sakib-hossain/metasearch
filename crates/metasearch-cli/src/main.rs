//! Metasearch CLI — entry point for the application.

use std::sync::Arc;

use clap::{Parser, Subcommand};
use tracing_subscriber::EnvFilter;

use metasearch_core::config::Settings;
use metasearch_engine::{
    EngineRegistry, brave::Brave, duckduckgo::DuckDuckGo, google::Google, wikipedia::Wikipedia,
};
use metasearch_server::{app, cache::SearchCache, state::AppState};

#[derive(Parser)]
#[command(name = "metasearch")]
#[command(about = "🔍 A blazing-fast, privacy-respecting metasearch engine")]
#[command(version)]
struct Cli {
    #[command(subcommand)]
    command: Option<Commands>,

    /// Host to bind to
    #[arg(long, default_value = "0.0.0.0")]
    host: String,

    /// Port to listen on
    #[arg(short, long, default_value_t = 8888)]
    port: u16,
}

#[derive(Subcommand)]
enum Commands {
    /// Start the web server (default)
    Serve,
    /// List all registered engines
    Engines,
    /// Print the default configuration
    Config,
}

#[tokio::main]
async fn main() -> anyhow::Result<()> {
    // Initialize tracing
    tracing_subscriber::fmt()
        .with_env_filter(EnvFilter::from_default_env().add_directive("metasearch=info".parse()?))
        .init();

    let cli = Cli::parse();

    // Build settings
    let mut settings = Settings::default();
    settings.server.host = cli.host;
    settings.server.port = cli.port;

    // Build HTTP client shared across engines
    let http_client = reqwest::Client::builder()
        .user_agent("Metasearch/0.1 (https://github.com/najmus-sakib-hossain/metasearch)")
        .build()?;

    // Register engines
    let mut registry = EngineRegistry::new();
    registry.register(Arc::new(Google::new(http_client.clone())));
    registry.register(Arc::new(DuckDuckGo::new(http_client.clone())));
    registry.register(Arc::new(Brave::new(http_client.clone(), None)));
    registry.register(Arc::new(Wikipedia::new(http_client.clone())));

    let state = Arc::new(AppState {
        cache: SearchCache::new(settings.cache.max_entries, settings.cache.ttl_secs),
        engine_registry: Arc::new(registry),
        settings,
    });

    match cli.command.unwrap_or(Commands::Serve) {
        Commands::Serve => {
            app::run(state).await?;
        }
        Commands::Engines => {
            println!("Registered engines:");
            for name in state.engine_registry.engine_names() {
                println!("  - {}", name);
            }
        }
        Commands::Config => {
            println!("{}", serde_json::to_string_pretty(&state.settings)?);
        }
    }

    Ok(())
}
