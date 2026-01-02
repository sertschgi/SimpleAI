pub mod paths;

pub const APP_ID: &'static str = "simpleai";

use std::path::PathBuf;
use thiserror::Error;
#[derive(Debug, Error)]
pub enum StorageError {
    #[error("The storage dir could not be determined.")]
    DirNotDeterminable,
    #[error("The storage dir could not be created. {0}.")]
    DirNotCreatable(PathBuf),
    #[error("The storage file could not be created: {0}.")]
    FileNotCreatable(PathBuf),
    #[error("Found invalid syntax in storage file: {0}")]
    InvalidFileSyntax(String),
}

#[derive(Debug, Error)]
pub enum CacheError {
    #[error("Cache Error: {0}")]
    StorageError(#[from] StorageError),
}

pub struct Cache<T> {
    pat: T,
}

use serde::{de::DeserializeOwned, Serialize};
use std::{fs::File, io::Read};
impl<T> Cache<T>
where
    T: Serialize + DeserializeOwned,
{
    pub fn file() -> Result<File, CacheError> {
        let path = paths::cache_dir()?.join(format!("{}.toml", uuid::Uuid::new_v4()));
        Ok(File::create(path).map_err(|_| StorageError::FileNotCreatable(path))?)
    }

    pub fn get() -> Result<T, CacheError> {
        let mut content = String::new();
        Self::file()?.read_to_string(&mut content);
        Ok(toml::from_str(&content).map_err(|_| StorageError::InvalidFileSyntax(content))?)
    }
}
