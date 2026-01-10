use serde::{Deserialize, Serialize};
use std::path::PathBuf;
use thiserror::Error;
use uuid::Uuid;

pub const PROJECT_FILE_NAME: &'static str = "project.sai.json";

pub type ProjectQueryResult = Result<Project, ProjectQueryError>;
pub type ProjectResult<T> = Result<T, ProjectError>;

#[derive(Debug, Error, Clone, PartialEq)]
pub enum ProjectQueryError {
    #[error("Encountered invalid syntax when deserializing the project file: {0}.")]
    InvalidSyntax(String),
    #[error("No project was found that would have satisfied the query.")]
    ProjectNotFound,
    #[error("No project found in the given project dir: {0}.")]
    NoProjectFoundInProjectDir(PathBuf),
    #[error("Failed to open project file: {0}.")]
    FailedToOpenProjectFile(String),
    #[error("Failed to read project directory: {0}.")]
    FailedToReadProjectDir(String),
    #[error("Failed to read project file: {0}.")]
    FailedToReadProjectFile(String),
    #[error("Encountered projects with same ids.")]
    MultipleSameIds(Vec<Project>),
    #[error("Storage Error while querying project: {0}")]
    StorageError(#[from] StorageError),
}

use crate::modules::storage::{CacheError, StorageError};
#[derive(Debug, Error, Clone, PartialEq)]
pub enum ProjectError {
    #[error("Could not find a project with the id {0}.")]
    NoProjectWithId(Uuid),
    #[error("Could not get a valid cache dir while trying to get a project: {0}.")]
    InvalidCache(#[from] CacheError),
    #[error("Could not query project: {0}.")]
    QueryError(#[from] ProjectQueryError),
    #[error("Storage Error: {0}.")]
    StorageError(#[from] StorageError),
}

use crate::modules::storage::Cache;
type ProjectCache = Cache<Vec<PathBuf>>;

use crate::modules::storage::Storage;
type ProjectStorage = Storage<Project>;

use chrono::{DateTime, Utc};
pub type ProjectDate = DateTime<Utc>;

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct ProjectValues {
    pub name: String,
    pub date: ProjectDate,
    pub desc: String,
    pub author: String,
    pub path: PathBuf,
}

impl ProjectValues {
    pub fn id(&self) -> Uuid {
        Self::id_from_json(&self.to_json())
    }
    pub fn id_from_json(json: &str) -> Uuid {
        Uuid::new_v3(&Uuid::NAMESPACE_OID, json.as_bytes())
    }
    pub fn to_json(&self) -> String {
        serde_json::to_string(self).unwrap()
    }
}

// impl TryFrom<String> for ProjectValues {
//     type Error = ProjectQueryError;
//     fn try_from(value: String) -> Result<Self, Self::Error> {
//         Ok(serde_json::from_str(&value).map_err(|e| Self::Error::InvalidSyntax(e.to_string()))?)
//     }
// }

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct Project {
    pub id: Uuid,
    pub values: ProjectValues,
}

use crate::modules::storage::Storeable;
use crate::modules::utils::query_filter::ProjectQueryFilter;
impl Project {
    pub fn get_all() -> ProjectResult<Vec<ProjectQueryResult>> {
        let dir_paths: Vec<PathBuf> = ProjectCache::new()?.storage_content()?;

        let projects = dir_paths
            .iter()
            .map(|dir_path: &PathBuf| -> ProjectQueryResult {
                let file_path = dir_path
                    .read_dir()
                    .map_err(|e| ProjectQueryError::FailedToReadProjectDir(e.to_string()))?
                    .find(|entry_r| {
                        if let Ok(entry) = entry_r {
                            return entry.file_name() == PROJECT_FILE_NAME;
                        }
                        false
                    })
                    .ok_or(ProjectQueryError::NoProjectFoundInProjectDir(
                        dir_path.to_owned().clone(),
                    ))?
                    .unwrap()
                    .path();

                Ok(ProjectStorage::new(file_path).storage_content()?)
            })
            .collect();

        Ok(projects)
    }
    pub fn query(query_filters: Vec<ProjectQueryFilter>) -> ProjectResult<Vec<ProjectQueryResult>> {
        Ok(Self::get_all()?
            .into_iter()
            .filter(|project| {
                query_filters.iter().all(|filter| match project {
                    Ok(p) => filter.is_ok(&p),
                    Err(_) => true,
                })
            })
            .collect())
    }
    pub fn query_save(query_filters: Vec<ProjectQueryFilter>) -> ProjectResult<Vec<Project>> {
        Ok(Self::get_all()?
            .into_iter()
            .filter_map(Result::ok)
            .filter(|project| query_filters.iter().all(|filter| filter.is_ok(&project)))
            .collect())
    }
    pub fn new(name: String, desc: String, author: String, path: PathBuf) -> Self {
        ProjectValues {
            name,
            date: Utc::now(),
            desc,
            author,
            path,
        }
        .into()
    }
    pub fn create(self) -> Result<(), ProjectError> {
        let path = self.values.path.clone();
        let mut storage = ProjectStorage::from(path.clone().join(PROJECT_FILE_NAME));
        storage.storage_save(self)?;
        let mut cache = ProjectCache::new()?;
        let mut cache_content = cache.storage_content()?;
        cache_content.push(path);
        cache.storage_save(cache_content)?;
        Ok(())
    }
    pub fn edit(self) -> Result<(), ProjectError> {
        todo!()
    }
    pub fn delete(self) -> Result<(), ProjectError> {
        todo!()
    }
}

// impl TryFrom<String> for Project {
//     type Error = ProjectQueryError;
//     fn try_from(value: String) -> Result<Self, Self::Error> {
//         Ok(ProjectValues::try_from(value)?.into())
//     }
// }
impl TryFrom<Uuid> for Project {
    type Error = ProjectError;
    fn try_from(id: Uuid) -> Result<Self, Self::Error> {
        let mut all = Self::query_save(vec![ProjectQueryFilter::Id(id)])?;
        if all.len() == 1 {
            Ok(all.swap_remove(0))
        } else if all.is_empty() {
            Err(ProjectQueryError::ProjectNotFound.into())
        } else {
            Err(ProjectQueryError::MultipleSameIds(all).into())
        }
    }
}
impl From<ProjectValues> for Project {
    fn from(values: ProjectValues) -> Self {
        Self {
            id: values.id(),
            values,
        }
    }
}
