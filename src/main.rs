use vaulton::config::Config;
use vaulton::config::yaml_config_source::YamlConfigSource;
use vaulton::config::env_config_source::EnvConfigSource;
use vaulton::utils::fs::LocalFileSystem;
use vaulton::utils::env::SystemEnv;
use vaulton::config::builder::ConfigBuilder;

use tokio;
use clap::Parser;

/// Vaulton Server
#[derive(Parser, Debug)]
#[command(author, version, about, long_about = None)]
struct Args {
    /// Path to the config file
    #[arg(short, long, default_value = "config.yaml")]
    config: String,
}

#[tokio::main]
async fn main() {
    let args = Args::parse();
    
    let config = ConfigBuilder::default()
        .with_yaml_file(args.config)
        .build()
        .unwrap_or_else(|e| {
            eprintln!("Failed to load configuration: {}", e);
            std::process::exit(1);
        });
    
    println!("Loaded config: {:?}", config);

    let app = vaulton::server::create_server().await;
    let listener = tokio::net::TcpListener::bind("127.0.0.1:3000").await.unwrap();
    println!("Server running on http://127.0.0.1:3000");
    
    axum::serve(listener, app).await.unwrap();
}