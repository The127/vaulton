use std::collections::HashMap;
use super::{Env, EnvError};

/// Test implementation of environment variables interface
#[derive(Debug, Default)]
pub struct TestEnv {
    vars: HashMap<String, String>,
}

impl TestEnv {
    /// Creates a new empty test environment
    pub fn new() -> Self {
        Self {
            vars: HashMap::new()
        }
    }

    /// Creates a new test environment with initial variables
    pub fn with_vars<I>(vars: I) -> Self
    where
        I: IntoIterator<Item = (String, String)>,
    {
        Self {
            vars: HashMap::from_iter(vars)
        }
    }
}

impl Env for TestEnv {
    fn set_var(&mut self, key: &str, value: &str) -> Result<(), EnvError> {
        self.vars.insert(key.to_string(), value.to_string());
        Ok(())
    }

    fn get_var(&self, key: &str) -> Result<String, EnvError> {
        self.vars
            .get(key)
            .cloned()
            .ok_or_else(|| EnvError::GetError(format!("Environment variable not found: {}", key)))
    }

    fn remove_var(&mut self, key: &str) -> Result<(), EnvError> {
        self.vars.remove(key);
        Ok(())
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_new_empty() {
        let mut env = TestEnv::new();
        assert!(env.get_var("ANY_KEY").is_err());
    }

    #[test]
    fn test_with_vars() {
        let mut env = TestEnv::with_vars([
            ("KEY1".to_string(), "value1".to_string()),
            ("KEY2".to_string(), "value2".to_string()),
        ]);
        
        assert_eq!(env.get_var("KEY1").unwrap(), "value1");
        assert_eq!(env.get_var("KEY2").unwrap(), "value2");
    }

    #[test]
    fn test_set_and_get() {
        let mut env = TestEnv::new();
        env.set_var("TEST_KEY", "test_value").unwrap();
        assert_eq!(env.get_var("TEST_KEY").unwrap(), "test_value");
    }

    #[test]
    fn test_remove() {
        let mut env = TestEnv::new();
        env.set_var("TEST_KEY", "test_value").unwrap();
        env.remove_var("TEST_KEY").unwrap();
        assert!(env.get_var("TEST_KEY").is_err());
    }
}