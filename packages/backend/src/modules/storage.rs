pub mod paths;

pub const APP_ID: &'static str = "simpleai";

use std::{io::Write, path::PathBuf};
use thiserror::Error;
#[derive(Debug, Error, Clone, PartialEq)]
pub enum StorageError {
    #[error("The storage dir could not be determined")]
    DirNotDeterminable,
    #[error("The storage dir could not be created. {0}")]
    DirNotCreatable(PathBuf),
    #[error("The storage file could not be created: {0}")]
    FileNotCreatable(PathBuf),
    #[error("The storage file could not be opened: {0}")]
    FileNotOpenable(PathBuf),
    #[error("The storage file could not be read: {0}")]
    FileNotReadable(PathBuf),
    #[error("The storage file could not be read: {0}")]
    FileNotWriteable(PathBuf),
    #[error("Found invalid syntax in storage file '{0}': {1}, contents: >>{2}<<")]
    InvalidFileSyntax(PathBuf, String, String),
}

#[derive(Debug, Error, Clone, PartialEq)]
pub enum CacheError {
    #[error("while getting cache dir: {0}")]
    StorageError(#[from] StorageError),
}

pub struct Cache<T> {
    pub value: T,
    pub path: PathBuf,
}

use serde::{de::DeserializeOwned, Serialize};
use std::{fs::File, io::Read};
use uuid::Uuid;
impl<T> Cache<T>
where
    T: Serialize + DeserializeOwned + Default,
{
    pub fn get() -> Result<Self, CacheError> {
        let path = paths::cache_dir()?.join(format!(
            "{APP_ID}-{}.json",
            Uuid::new_v3(&Uuid::NAMESPACE_OID, std::any::type_name::<T>().as_bytes())
        ));
        if !path.exists() {
            File::create(&path).map_err(|_| StorageError::FileNotCreatable(path.clone()))?;
            return Ok(Self {
                value: T::default(),
                path,
            });
        }
        let mut file =
            File::open(&path).map_err(|_| StorageError::FileNotOpenable(path.clone()))?;
        let mut content = String::new();
        file.read_to_string(&mut content)
            .map_err(|_| StorageError::FileNotReadable(path.clone()))?;
        if content.is_empty() {
            content = serde_json::to_string(&T::default()).unwrap()
        }
        Ok(Self {
            value: serde_json::from_str(&content).map_err(|e| {
                StorageError::InvalidFileSyntax(path.clone(), e.to_string(), content)
            })?,
            path,
        })
    }

    pub fn write(&mut self) -> Result<(), CacheError> {
        let mut file = File::open(self.path.clone())
            .map_err(|_| StorageError::FileNotOpenable(self.path.clone()))?;
        file.write(serde_json::to_string(&self.value).unwrap().as_bytes())
            .map_err(|_| StorageError::FileNotWriteable(self.path.clone()))?;
        Ok(())
    }
}
