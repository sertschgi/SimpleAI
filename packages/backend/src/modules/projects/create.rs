use crate::modules::utils::{check_name, prelude::*};
use anyhow::Result;
use std::fs::{create_dir_all, read_to_string, File};
use std::io::Write;
use std::path::{Path, PathBuf};
use uuid::Uuid;

// #[cfg(feature = "desktop")]
pub fn create_project(project: Project, overwrite: bool) -> Result<Project, String> {
    let ProjectValues { name, author, .. } = &project.values;

    let name = name.clone();
    let author = author.clone();

    if !check_name(name.clone()) {
        return Err(format!(
            "Project name {} is not allowed! Please only use letters, dashes and underscores.",
            name
        ));
    }

    let project_path = Path::new("projects/").join(author);
    if !project_path.exists() {
        create_dir_all(&project_path).map_err(|e| e.to_string())?;
    }

    let meta_path = project_path.join(format!("{}.json", name));
    if meta_path.exists() && !overwrite {
        return Err(format!("Project name {} does already exist!", name));
    }

    let meta_json = serde_json::to_string(&project).unwrap();

    let mut meta_file = File::create(&meta_path).map_err(|e| e.to_string())?;
    meta_file
        .write_all(meta_json.as_bytes())
        .map_err(|e| e.to_string())?;

    Ok(project)
}
