use crate::modules::utils::{check_name, prelude::*};
use anyhow::Result;
use std::fs::{remove_file, File};
use std::io::Write;
use std::path::{Path, PathBuf};
use uuid::Uuid;

// #[cfg(feature = "desktop")]
pub fn delete_project(
    Project {
        values: ProjectValues { name, author, .. },
        ..
    }: Project,
) -> Result<(), String> {
    let project_path = Path::new("projects/")
        .join(author)
        .join(format!("{name}.json"));
    remove_file(&project_path).map_err(|e| e.to_string())?;
    Ok(())
}
