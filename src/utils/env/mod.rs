pub mod test_env;

use std::env;
use std::error::Error;
use std::fmt;

/// Error type for environment variable operations
#[derive(Debug)]
pub enum EnvError {
    /// Error when setting an environment variable
    SetError(String),
    /// Error when getting an environment variable
    GetError(String),
}

impl fmt::Display for EnvError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            EnvError::SetError(msg) => write!(f, "Failed to set environment variable: {}", msg),
            EnvError::GetError(msg) => write!(f, "Failed to get environment variable: {}", msg),
        }
    }
}

impl Error for EnvError {}

/// Trait defining operations for environment variables
pub trait Env: Send + Sync {
    /// Set an environment variable
    fn set_var(&mut self, key: &str, value: &str) -> Result<(), EnvError>;

    /// Get an environment variable
    fn get_var(&mut self, key: &str) -> Result<String, EnvError>;

    /// Remove an environment variable
    fn remove_var(&mut self, key: &str) -> Result<(), EnvError>;
}

/// Implementation of Env that uses the system environment
#[derive(Debug, Default)]
pub struct SystemEnv;

impl Env for SystemEnv {
    fn set_var(&mut self, key: &str, value: &str) -> Result<(), EnvError> {
        env::set_var(key, value);
        Ok(())
    }

    fn get_var(&mut self, key: &str) -> Result<String, EnvError> {
        env::var(key).map_err(|e| EnvError::GetError(e.to_string()))
    }

    fn remove_var(&mut self, key: &str) -> Result<(), EnvError> {
        env::remove_var(key);
        Ok(())
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_set_and_get_var() {
        let mut env = SystemEnv::default();
        env.set_var("TEST_KEY", "test_value").unwrap();
        assert_eq!(env.get_var("TEST_KEY").unwrap(), "test_value");
    }

    #[test]
    fn test_remove_var() {
        let mut env = SystemEnv::default();
        env.set_var("TEST_KEY_REMOVE", "test_value").unwrap();
        env.remove_var("TEST_KEY_REMOVE").unwrap();
        assert!(env.get_var("TEST_KEY_REMOVE").is_err());
    }

    #[test]
    fn test_get_nonexistent_var() {
        let mut env = SystemEnv::default();
        assert!(env.get_var("NONEXISTENT_KEY").is_err());
    }
}