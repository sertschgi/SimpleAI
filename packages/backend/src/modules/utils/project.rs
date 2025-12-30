use std::{
    fs,
    path::{Path, PathBuf},
};
use uuid::Uuid;

pub const PROJECT_FILE_NAME: &'static str = "project.sai.json";

use crate::modules::config::ConfigError;
use thiserror::Error;
#[derive(Debug, Error)]
pub enum ProjectError {
    #[error("Could not find a project with the id {0}.")]
    NoProjectWithId(Uuid),
    #[error("Could not get a valid config while trying to get a project: {0}.")]
    InvalidConfig(#[from] ConfigError),
    #[error("Failed to open project file: {0}.")]
    FailedToOpenProjectFile(PathBuf),
    #[error("Encountered invalid Syntax when deserializing the project file: {0}.")]
    InvalidSyntax(String),
    #[error("No project was found that would have satisfied the query.")]
    ProjectNotFound,
    #[error("Failed to read project directory: {0}.")]
    FailedToReadProjectDir(PathBuf),
    #[error("Failed to traverse project dir: {0}.")]
    FailedToTraverseProjectDir(PathBuf),
    #[error("Failed to read project file: {0}.")]
    FailedToReadProjectFile(PathBuf),
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
    type Error = ProjectError;
    fn try_from(value: String) -> Result<Self, Self::Error> {
        Ok(serde_json::from_str(&value).map_err(|e| ProjectError::InvalidSyntax(e.to_string()))?)
    }
}

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct Project {
    pub id: Uuid,
    pub values: ProjectValues,
}

use crate::modules::utils::query_filter::ProjectQueryFilter;
impl Project {
    pub fn get_all() -> Result<Vec<Self>, ProjectError> {
        use crate::modules::config::Config;
        use std::{
            fs::{File, ReadDir},
            io::Read,
        };

        let dir_paths = Config::get()?.project_dirs;

        let mut projects = Vec::<Self>::new();
        let projects_ref = &mut projects;

        let ignore_err = move |e: ProjectError| {
            println!("Warning: {e}, ignoring, skipping project entry.");
        };

        let mut deser_content = move |content: String| match ProjectValues::try_from(content) {
            Ok(values) => projects_ref.push(values.into()),
            Err(e) => ignore_err(e),
        };

        let mut read_file = move |file: &mut File, file_path: PathBuf| {
            let mut content = String::new();
            match file
                .read_to_string(&mut content)
                .map_err(|_| ProjectError::FailedToReadProjectFile(file_path))
            {
                Ok(_) => deser_content(content),
                Err(e) => ignore_err(e),
            };
        };

        let mut open_file = move |file_path: PathBuf| match File::open(&file_path)
            .map_err(|_| ProjectError::FailedToOpenProjectFile(file_path.clone()))
        {
            Ok(mut file) => read_file(&mut file, file_path),
            Err(e) => ignore_err(e),
        };

        let mut check_filename = move |file_path: PathBuf| {
            if let Some(name) = file_path.file_name() {
                if name == PROJECT_FILE_NAME {
                    open_file(file_path);
                }
            }
        };

        let mut traverse_files = move |read_dir: ReadDir, dir_path: PathBuf| {
            for entry in read_dir {
                match entry.map_err(|_| ProjectError::FailedToTraverseProjectDir(dir_path.clone()))
                {
                    Ok(e) => check_filename(e.path()),
                    Err(e) => ignore_err(e),
                }
            }
        };

        for dir_path in dir_paths {
            match dir_path
                .read_dir()
                .map_err(|_| ProjectError::FailedToReadProjectDir(dir_path.clone()))
            {
                Ok(read_dir) => traverse_files(read_dir, dir_path),
                Err(e) => ignore_err(e),
            }
        }

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
    pub fn delete(
        Project {
            values: ProjectValues { name, author, .. },
            ..
        }: Self,
    ) -> Self {
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
