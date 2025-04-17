use serde::Deserialize;

/// Main configuration structure for the Vaulton server
#[derive(Debug, Deserialize)]
pub struct Config {
    /// Server-specific configuration settings
    pub server: ServerConfig,
}

/// Configuration for the server's network settings
#[derive(Debug, Deserialize)]
pub struct ServerConfig {
    /// The IP address the server will bind to
    /// 
    /// Examples:
    /// - "127.0.0.1" for localhost only
    /// - "0.0.0.0" for all interfaces
    pub bind_addr: String,

    /// The port number the server will listen on
    /// 
    /// Valid values are from 1 to 65535, though ports below 1024
    /// typically require root/administrator privileges
    pub port: u16,
}

