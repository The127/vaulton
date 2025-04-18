pub mod config;
pub mod utils;
pub mod server;
pub mod oidc;

// Re-export main types for easier access
pub use config::{Config, ConfigSource};


#[cfg(test)]
mod test {
    #[test]
    fn foo(){
        assert!(true)
    }
}