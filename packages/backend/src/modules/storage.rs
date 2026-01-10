pub mod paths;

pub const APP_ID: &'static str = "simpleai";

use std::{io::Write, path::PathBuf};
use thiserror::Error;
#[derive(Debug, Error, Clone, PartialEq)]
pub enum StorageError {
    #[error("The storage dir could not be determined")]
    DirNotDeterminable,
    #[error("The storage dir could not be created. '{0}', {1}")]
    DirNotCreatable(PathBuf, String),
    #[error("The storage file could not be created: '{0}', {1}")]
    FileNotCreatable(PathBuf, String),
    #[error("The storage file could not be opened: '{0}', {1}")]
    FileNotOpenable(PathBuf, String),
    #[error("The storage file could not be read: '{0}', {1}")]
    FileNotReadable(PathBuf, String),
    #[error("The storage file could not be written: '{0}', {1}")]
    FileNotWriteable(PathBuf, String),
    #[error("The storage file could not be deleted: '{0}', {1}")]
    FileNotDeletable(PathBuf, String),
    #[error("The storage file is empty and could not be serialized: '{0}'")]
    EmptyFile(PathBuf),
    #[error("Found invalid syntax in storage file '{0}': {1}, contents: >>{2}<<")]
    InvalidFileSyntax(PathBuf, String, String),
}

use serde::{de::DeserializeOwned, Serialize};
use std::{fs::File, io::Read};
use uuid::Uuid;
pub trait Storeable<ST>
where
    ST: Serialize + DeserializeOwned,
{
    type Error;
    fn storage_content(&self) -> Result<ST, Self::Error>;
    fn storage_save(&mut self, value: ST) -> Result<(), Self::Error>;
    fn storage_delete(&self) -> Result<(), Self::Error>;
}

impl<ST> Storeable<ST> for PathBuf
where
    ST: Serialize + DeserializeOwned,
{
    type Error = StorageError;

    fn storage_content(&self) -> Result<ST, Self::Error> {
        let mut file = File::options()
            .write(true)
            .create(true)
            .read(true)
            .open(&self)
            .map_err(|e| StorageError::FileNotOpenable(self.clone(), e.to_string()))?;
        let mut content = String::new();
        file.read_to_string(&mut content)
            .map_err(|e| StorageError::FileNotReadable(self.clone(), e.to_string()))?;
        if content.is_empty() {
            return Err(StorageError::EmptyFile(self.clone()));
        }
        Ok(serde_json::from_str(&content)
            .map_err(|e| StorageError::InvalidFileSyntax(self.clone(), e.to_string(), content))?)
    }

    fn storage_save(&mut self, value: ST) -> Result<(), Self::Error> {
        let mut file = File::options()
            .write(true)
            .create(true)
            .truncate(true)
            .open(&self)
            .map_err(|e| StorageError::FileNotOpenable(self.clone(), e.to_string()))?;
        file.write_all(serde_json::to_string_pretty(&value).unwrap().as_bytes())
            .map_err(|e| StorageError::FileNotWriteable(self.clone(), e.to_string()))?;
        Ok(())
    }

    fn storage_delete(&self) -> Result<(), Self::Error> {
        Ok(std::fs::remove_file(self)
            .map_err(|e| StorageError::FileNotDeletable(self.clone(), e.to_string()))?)
    }
}

use std::marker::PhantomData;
pub struct Storage<ST> {
    path: PathBuf,
    _marker: PhantomData<ST>,
}

impl<ST> Storage<ST> {
    pub fn new(path: PathBuf) -> Self {
        Self {
            path,
            _marker: PhantomData,
        }
    }
}

impl<ST> From<PathBuf> for Storage<ST> {
    fn from(path: PathBuf) -> Self {
        Self::new(path)
    }
}

impl<ST> Storeable<ST> for Storage<ST>
where
    ST: Serialize + DeserializeOwned,
{
    type Error = StorageError;
    fn storage_content(&self) -> Result<ST, Self::Error> {
        self.path.storage_content()
    }

    fn storage_save(&mut self, value: ST) -> Result<(), Self::Error> {
        self.path.storage_save(value)
    }

    fn storage_delete(&self) -> Result<(), Self::Error> {
        Ok(<PathBuf as Storeable<ST>>::storage_delete(&self.path)?)
    }
}

#[derive(Debug, Error, Clone, PartialEq)]
pub enum CacheError {
    #[error("Error while getting cache dir: {0}")]
    StorageError(#[from] StorageError),
}

pub struct Cache<ST> {
    path: PathBuf,
    _marker: PhantomData<ST>,
}

impl<ST> Cache<ST> {
    pub fn new() -> Result<Self, CacheError> {
        Ok(Self {
            path: paths::cache_dir()?.join(format!(
                "{APP_ID}-{}.json",
                Uuid::new_v3(&Uuid::NAMESPACE_OID, std::any::type_name::<ST>().as_bytes())
            )),
            _marker: PhantomData,
        })
    }
}

impl<ST> Storeable<ST> for Cache<ST>
where
    ST: Serialize + DeserializeOwned + Default,
{
    type Error = CacheError;
    fn storage_content(&self) -> Result<ST, Self::Error> {
        let content = self.path.storage_content();
        match content {
            Ok(c) => Ok(c),
            Err(e) => match e {
                StorageError::EmptyFile(..) => Ok(ST::default()),
                _ => Err(e.into()),
            },
        }
    }

    fn storage_save(&mut self, value: ST) -> Result<(), Self::Error> {
        Ok(self.path.storage_save(value)?)
    }

    fn storage_delete(&self) -> Result<(), Self::Error> {
        Ok(<PathBuf as Storeable<ST>>::storage_delete(&self.path)?)
    }
}
