use std::path::PathBuf;
use thiserror::Error;
use uuid::Uuid;

pub const PROJECT_FILE_NAME: &'static str = "project.sai.json";

pub type ProjectQueryResult = Result<Project, ProjectQueryError>;

use std::io;
#[derive(Debug, Error)]
pub enum ProjectQueryError {
    #[error("Encountered invalid syntax when deserializing the project file: {0}.")]
    InvalidSyntax(String),
    #[error("No project was found that would have satisfied the query.")]
    ProjectNotFound,
    #[error("No project found in the given project dir: {0}.")]
    NoProjectFoundInProjectDir(PathBuf),
    #[error("Failed to open project file: {0}.")]
    FailedToOpenProjectFile(io::Error),
    #[error("Failed to read project directory: {0}.")]
    FailedToReadProjectDir(io::Error),
    #[error("Failed to read project file: {0}.")]
    FailedToReadProjectFile(io::Error),
}

use crate::modules::config::ConfigError;
#[derive(Debug, Error)]
pub enum ProjectError {
    #[error("Could not find a project with the id {0}.")]
    NoProjectWithId(Uuid),
    #[error("Could not get a valid config while trying to get a project: {0}.")]
    InvalidConfig(#[from] ConfigError),
    #[error("Could not query project: {0}.")]
    QueryError(#[from] ProjectQueryError),
}

use chrono::{DateTime, Utc};
pub type ProjectDate = DateTime<Utc>;

use serde::{Deserialize, Serialize};
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

impl TryFrom<String> for ProjectValues {
    type Error = ProjectQueryError;
    fn try_from(value: String) -> Result<Self, Self::Error> {
        Ok(serde_json::from_str(&value).map_err(|e| Self::Error::InvalidSyntax(e.to_string()))?)
    }
}

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct Project {
    pub id: Uuid,
    pub values: ProjectValues,
}

use crate::modules::utils::query_filter::ProjectQueryFilter;
impl Project {
    pub fn get_all() -> Result<Vec<ProjectQueryResult>, ProjectError> {
        use crate::modules::config::Config;
        use std::{
            fs::{DirEntry, File, ReadDir},
            io::Read,
        };

        let dir_paths = Config::get()?.project_dirs;

        let projects = dir_paths
            .iter()
            .map(|dir_path: &PathBuf| -> ProjectQueryResult {
                let file_path = dir_path
                    .read_dir()
                    .map_err(|e| ProjectQueryError::FailedToReadProjectDir(e))?
                    .find(|&entry_r| {
                        if let Ok(entry) = entry_r {
                            entry.file_name() == PROJECT_FILE_NAME;
                        }
                        false
                    })
                    .ok_or(ProjectQueryError::NoProjectFoundInProjectDir(*dir_path))?
                    .unwrap()
                    .path();

                let mut file = File::open(&file_path)
                    .map_err(|e| ProjectQueryError::FailedToOpenProjectFile(e))?;

                let mut content = String::new();
                file.read_to_string(&mut content)
                    .map_err(|e| ProjectQueryError::FailedToReadProjectFile(e))?;

                ProjectValues::try_from(content)
            })
            .collect();

        Ok(projects)
    }
    pub fn query_all(query_filters: Vec<ProjectQueryFilter>) -> Result<Vec<Self>, ProjectError> {
        let all_projects = Self::get_all()?;

        Ok(all_projects
            .iter()
            .filter(|project| {
                query_filters
                    .iter()
                    .all(|filter| filter.clone().is_ok(project))
            })
            .cloned()
            .collect())
    }
    pub fn query(query_filters: Vec<ProjectQueryFilter>) -> Result<Self, ProjectError> {
        Ok(Self::query_all(query_filters)?
            .first()
            .ok_or(ProjectError::ProjectNotFound)?
            .clone())
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
        todo!()
    }
    pub fn edit(self) -> Result<(), ProjectError> {
        todo!()
    }
    pub fn delete(self) -> Result<(), ProjectError> {
        todo!()
    }
}

impl TryFrom<Uuid> for Project {
    type Error = ProjectError;
    fn try_from(id: Uuid) -> Result<Self, Self::Error> {
        Project::query(vec![ProjectQueryFilter::Id(id)])
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
