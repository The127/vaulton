// tests/config_sources.rs

use std::path::PathBuf;
use vaulton::config::{Config, ConfigSource};
use vaulton::config::yaml_config_source::YamlConfigSource;
use vaulton::config::env_config_source::EnvConfigSource;
use vaulton::utils::env::test_env::TestEnv;
use vaulton::utils::fs::mock_file_system::MockFileSystem;

#[test]
fn test_multiple_config_sources() {
    // Set up mock filesystem with a YAML config
    let fs = MockFileSystem::new()
        .with_file(
            "config.yaml",
            r#"
server:
  bind_addr: "127.0.0.1"
  port: 3000
"#,
        );

    // Set up test environment variables
    let env = TestEnv::with_vars([
        ("VAULTON__SERVER__PORT".to_string(), "8080".to_string()),
    ]);

    // Create config sources
    let yaml_source = YamlConfigSource::new(
        "config.yaml",
        fs,
    );
    let env_source = EnvConfigSource::new(env);

    // Create and apply configuration
    let mut config = Config::default();
    yaml_source.apply(&mut config).expect("Failed to apply YAML config");
    env_source.apply(&mut config).expect("Failed to apply ENV config");

    // Verify the configuration
    assert_eq!(config.server.bind_addr, Some("127.0.0.1".to_string()));
    assert_eq!(config.server.port, Some(8080));
}

#[test]
fn test_config_precedence() {
    // Set up mock filesystem
    let fs = MockFileSystem::new()
        .with_file(
            "config.yaml",
            r#"
server:
  bind_addr: "0.0.0.0"
  port: 5000
"#,
        );

    // Set up test environment
    let env = TestEnv::with_vars([
        ("VAULTON__SERVER__BIND_ADDR".to_string(), "192.168.1.1".to_string()),
        ("VAULTON__SERVER__PORT".to_string(), "8080".to_string()),
    ]);

    let yaml_source = YamlConfigSource::new(
        "config.yaml",
        fs,
    );
    let env_source = EnvConfigSource::new(env);

    // Test default configuration
    let default_config = Config::default();
    assert_eq!(default_config.server.bind_addr, Some("127.0.0.1".to_string()));
    assert_eq!(default_config.server.port, Some(3000));

    // Test YAML overrides
    let mut yaml_config = Config::default();
    yaml_source.apply(&mut yaml_config).expect("Failed to apply YAML config");
    assert_eq!(yaml_config.server.bind_addr, Some("0.0.0.0".to_string()));
    assert_eq!(yaml_config.server.port, Some(5000));

    // Test complete override stack
    let mut full_config = Config::default();
    yaml_source.apply(&mut full_config).expect("Failed to apply YAML config");
    env_source.apply(&mut full_config).expect("Failed to apply ENV config");
    assert_eq!(full_config.server.bind_addr, Some("192.168.1.1".to_string()));
    assert_eq!(full_config.server.port, Some(8080));
}

#[test]
fn test_partial_overrides() {
    // Set up mock filesystem with partial config
    let fs = MockFileSystem::new()
        .with_file(
            "config.yaml",
            r#"
server:
  bind_addr: "0.0.0.0"
"#,
        );

    // Set up environment with different partial config
    let env = TestEnv::with_vars([
        ("VAULTON__SERVER__PORT".to_string(), "9000".to_string()),
    ]);

    let yaml_source = YamlConfigSource::new(
        "config.yaml",
        fs,
    );
    let env_source = EnvConfigSource::new(env);

    let mut config = Config::default();
    yaml_source.apply(&mut config).expect("Failed to apply YAML config");
    env_source.apply(&mut config).expect("Failed to apply ENV config");

    // Verify each source's contribution
    assert_eq!(config.server.bind_addr, Some("0.0.0.0".to_string())); // from YAML
    assert_eq!(config.server.port, Some(9000)); // from ENV
}