use cfg_if::cfg_if;
use std::path::PathBuf;

use super::{ConfigError, APP_ID};

/// Determines and returns the data directory of the app based on the system
pub fn determine_app_data_dir() -> Result<PathBuf, ConfigError> {
    cfg_if! {
        // Web (WASM) - use IndexedDB / origin-private filesystem in JS; here return a logical path under "/" for callers
        if #[cfg(all(target_arch = "wasm32", target_family = "wasm"))] {
            // Return a stable logical path; actual storage must be handled via JS (IndexedDB / File System Access).
            // Provide something deterministic so code can key storage per app.
            Ok(PathBuf::from(format!("/app-data/{}", APP_ID)))
        }
        // Android - use Android context->getExternalFilesDir or getFilesDir; prefer external if available
        else if #[cfg(target_os = "android")] {
            use std::ffi::CStr;
            // We can't call Android APIs directly without JNI bindings. Use the ANDROID_DATA env fallback or common locations.
            // Try environment variables set by some bundles, then use conventional Android external storage path.
            if let Ok(p) = std::env::var("ANDROID_EXTERNAL_STORAGE") {
                let mut pb = PathBuf::from(p);
                pb.push("Android");
                pb.push("data");
                pb.push(APP_ID);
                return Ok(pb);
            }
            // Fallback to internal app files location under /data/data/<package>/files if APP_ID looks like package name
            if APP_ID.contains('.') {
                return Ok(PathBuf::from(format!("/data/data/{}/files", APP_ID)));
            }
            // Last resort: use /sdcard/Android/data/<APP_ID>
            Ok(PathBuf::from(format!("/sdcard/Android/data/{}", APP_ID)))
        }
        // iOS - standard sandboxed Library/Application Support/<AppId> or Documents
        else if #[cfg(target_os = "ios")] {
            // On iOS you normally query NSSearchPathForDirectoriesInDomains via Objective-C.
            // Use standard sandbox paths relative to home as a portable fallback.
            if let Some(home) = std::env::var_os("HOME") {
                let mut pb = PathBuf::from(home);
                pb.push("Library");
                pb.push("Application Support");
                pb.push(APP_ID);
                return Ok(pb);
            }
        }
        // Desktop (Windows, macOS, Linux, *BSD)
        else {
            // Use dirs-next to get appropriate user data dir and append APP_ID.
            if let Some(base) = dirs_next::data_dir() {
                let mut pb = base;
                // On Windows: data_dir() -> %APPDATA% (Roaming) typically; prefer Local for per-machine caches in other contexts.
                pb.push(APP_ID);
                return Ok(pb);
            }
            // As a fallback, use current executable directory
            if let Ok(exec) = std::env::current_exe() {
                if let Some(dir) = exec.parent() {
                    let mut pb = dir.to_path_buf();
                    pb.push(APP_ID);
                    return Ok(pb);
                }
            }
        }
    }

    Err(ConfigError::AppDataPathNotDeterminable)
}

/// Get and create the app data directory.
///
/// # Returns
/// Returns the path of the apps data dir.
///
/// # Errors
/// Returns an error when the directory could not be created or the data path could not be
/// determined.
pub fn app_data_dir() -> Result<PathBuf, ConfigError> {
    let path = determine_app_data_dir()?;
    if !path.exists() {
        std::fs::create_dir(&path).map_err(|_| ConfigError::AppDataPathNotCreatable)?;
    }
    Ok(path)
}
