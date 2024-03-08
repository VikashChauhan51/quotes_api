//! Server configuration
//!
use config::{Config, ConfigError, Environment, File};
use serde::Deserialize;

#[derive(Debug, Deserialize, Clone)]
/// The `Log` is use to set the logger configuration.
pub struct Log {
    /// Logging level.
    pub level: String,
}

#[derive(Debug, Deserialize, Clone)]

///  The `Server` is use to set the server configuration.
pub struct Server {
    /// Server port.
    pub port: u16,
    /// Server address
    pub url: String,
}

#[derive(Clone, Debug, Deserialize)]
/// The `ENV` is use to set the env types.
pub enum ENV {
    Development,
    Testing,
    Production,
}

impl From<&str> for ENV {
    fn from(env: &str) -> Self {
        match env {
            "Testing" => ENV::Testing,
            "Production" => ENV::Production,
            _ => ENV::Development,
        }
    }
}

#[derive(Debug, Deserialize, Clone)]
/// The `Settings` is use to set the server configuration.
pub struct Settings {
    pub server: Server,
    pub log: Log,
    pub env: ENV,
}

impl Settings {
    pub fn new() -> Result<Self, ConfigError> {
        let env = std::env::var("RUN_ENV").unwrap_or_else(|_| "Development".into());
        let mut s = Config::new();
        s.set("env", env.clone())?;

        s.merge(File::with_name("./config.toml"))?;
        s.merge(File::with_name(&format!("{}{}", "", env)))?;
        s.merge(Environment::with_prefix("quote").separator("__"))?;
        s.try_into()
    }
}
