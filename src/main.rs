use vaulton::config::{Config, ConfigSource};
use vaulton::config::yaml_config_source::YamlConfigSource;
use vaulton::config::env_config_source::EnvConfigSource;
use vaulton::utils::fs::LocalFileSystem;
use vaulton::utils::env::SystemEnv;

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
    
    let mut config = Config::default();
    
    // Set up YAML config source with the local filesystem
    let yaml_source = YamlConfigSource::new(
        args.config,
        LocalFileSystem::default()
    );
    
    // Apply the YAML configuration first
    if let Err(e) = yaml_source.apply(&mut config) {
        eprintln!("Failed to load YAML config: {}", e);
        std::process::exit(1);
    }
    
    // Set up and apply environment variables configuration
    let env_source = EnvConfigSource::new(SystemEnv::default());
    if let Err(e) = env_source.apply(&mut config) {
        eprintln!("Failed to load environment variables: {}", e);
        std::process::exit(1);
    }
    
    println!("Loaded config: {:?}", config);

    let app = vaulton::server::create_server().await;
    let listener = tokio::net::TcpListener::bind("127.0.0.1:3000").await.unwrap();
    println!("Server running on http://127.0.0.1:3000");
    
    axum::serve(listener, app).await.unwrap();
}