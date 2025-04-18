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
    
    let bind_addr = config.server.bind_addr.as_deref().unwrap();
    let port = config.server.port.unwrap();
    let addr = format!("{}:{}", bind_addr, port);
    
    let app = vaulton::server::create_server().await;
    let listener = tokio::net::TcpListener::bind(&addr).await.unwrap_or_else(|e| {
        eprintln!("Failed to bind to {}: {}", addr, e);
        std::process::exit(1);
    });
    
    println!("Server running on http://{}", addr);
    
    axum::serve(listener, app).await.unwrap();
}