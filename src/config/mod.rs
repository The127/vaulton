mod yaml_config_source;

use serde::Deserialize;
use std::error::Error;

/// Main configuration structure for the Vaulton server
#[derive(Debug, Deserialize)]
pub struct Config {
    /// Server-specific configuration settings
    #[serde(default)]
    pub server: ServerConfig,
}

impl Default for Config {
    fn default() -> Self {
        Self {
            server: ServerConfig::default(),
        }
    }
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

impl Default for ServerConfig {
    fn default() -> Self {
        Self {
            bind_addr: "127.0.0.1".to_string(),
            port: 3000,
        }
    }
}

/// Trait for loading static configuration from different sources
pub trait ConfigSource {
    /// Apply configuration from this source to the given config
    fn apply(&self, config: &mut Config) -> Result<(), Box<dyn Error>>;
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_server_config_default() {
        let config = ServerConfig::default();
        assert_eq!(config.bind_addr, "127.0.0.1");
        assert_eq!(config.port, 3000);
    }

    #[test]
    fn test_config_default() {
        let config = Config::default();
        assert_eq!(config.server.bind_addr, "127.0.0.1");
        assert_eq!(config.server.port, 3000);
    }
}