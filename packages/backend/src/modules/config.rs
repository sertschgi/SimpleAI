pub mod paths;

pub const APP_ID: &'static str = "simpleai";

use thiserror::Error;
#[derive(Debug, Error)]
pub enum ConfigError {
    #[error("The app data path could not be determined.")]
    AppDataPathNotDeterminable,
    #[error("The app data dir could not be created.")]
    AppDataPathNotCreatable,
    #[error("The app data dir could not be created.")]
    ConfigPathNotCreatable,
    #[error("Found invalid syntax in config file.")]
    InvalidConfigFileSyntax(String),
}

use serde::{Deserialize, Serialize};
use std::io::Read;
use std::path::PathBuf;
#[derive(Serialize, Deserialize)]
pub struct Config {
    pub project_dirs: Vec<PathBuf>,
}

use std::fs::File;
impl Config {
    /// Get and create the config file.
    ///
    /// # Returns
    /// Returns the config file as a `File`
    ///
    /// # Errors
    /// Returns a ConfigError when the app data dir cannot be obtained
    /// or the creation of the file fails
    pub fn file() -> Result<File, ConfigError> {
        Ok(File::create(paths::app_data_dir()?.join("Config.toml"))
            .map_err(|_| ConfigError::ConfigPathNotCreatable)?)
    }

    /// Get the `Config`
    ///
    /// # Returns
    /// Returns the `Config` type
    ///
    /// # Errors
    /// Returns a `ConfigError` when the config file cannot be obtained or the deserialisation failes
    pub fn get() -> Result<Self, ConfigError> {
        let mut content = String::new();
        Self::file()?.read_to_string(&mut content);
        Ok(toml::from_str(&content).map_err(|_| ConfigError::InvalidConfigFileSyntax(content))?)
    }
}
