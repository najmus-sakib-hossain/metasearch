//! Metasearch CLI — entry point for the application.

use std::sync::Arc;

use clap::{Parser, Subcommand};
use tracing_subscriber::EnvFilter;

use metasearch_core::config::Settings;
use metasearch_engine::EngineRegistry;
use metasearch_server::{app, cache::SearchCache, state::AppState, templates::Templates};

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

    /// Path to templates directory
    #[arg(long, default_value = "templates")]
    templates: String,
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
        .timeout(std::time::Duration::from_secs(10))
        .connect_timeout(std::time::Duration::from_secs(5))
        .build()?;

    // Register ALL engines using with_defaults() — 200+ engines across all categories
    let registry = EngineRegistry::with_defaults(http_client);
    let engine_count = registry.count();

    // Load templates
    let templates = Arc::new(Templates::new(&cli.templates)?);

    let state = Arc::new(AppState {
        cache: SearchCache::new(settings.cache.max_entries, settings.cache.ttl_secs),
        engine_registry: Arc::new(registry),
        templates,
        settings,
    });

    match cli.command.unwrap_or(Commands::Serve) {
        Commands::Serve => {
            tracing::info!("Registered {} search engines", engine_count);
            app::run(state).await?;
        }
        Commands::Engines => {
            println!("Registered engines ({}):", engine_count);
            let mut names = state.engine_registry.engine_names();
            names.sort();
            for name in names {
                println!("  - {}", name);
            }
        }
        Commands::Config => {
            println!("{}", serde_json::to_string_pretty(&state.settings)?);
        }
    }

    Ok(())
}
