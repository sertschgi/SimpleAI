use cfg_if::cfg_if;
use std::{env, path::PathBuf};

use super::{StorageError, APP_ID};

/// Determines and returns the data directory of the app based on the system
pub fn determine_app_data_dir() -> Result<PathBuf, StorageError> {
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

    Err(StorageError::DirNotDeterminable)
}

/// Returns a platform-appropriate cache directory PathBuf.
/// On success returns the directory path (does not create it).
pub fn determine_cache_dir() -> Result<PathBuf, StorageError> {
    // WASM (browser) — no real filesystem path
    #[cfg(target_arch = "wasm32")]
    {
        // For browser targets, return a virtual indicator. Use browser storage APIs instead.
        return Ok(PathBuf::from("browser-cache:"));
    }

    // Android: prefer ANDROID_CACHE if provided (set by some build systems), else use HOME/Android/data/<package>/cache if package provided via env var
    #[cfg(target_os = "android")]
    {
        if let Ok(dir) = env::var("ANDROID_CACHE") {
            return Ok(PathBuf::from(dir));
        }

        // Some apps set ANDROID_APP_PACKAGE at build time; try to use it if available.
        if let Ok(pkg) = env::var("ANDROID_APP_PACKAGE") {
            if let Ok(home) = env::var("HOME") {
                // Typical external cache: /data/data/<pkg>/cache or $HOME/Android/data/<pkg>/cache
                let candidate1 = PathBuf::from(format!("/data/data/{}/cache", pkg));
                if candidate1.exists() {
                    return Ok(candidate1);
                }
                let candidate2 = PathBuf::from(home)
                    .join("Android")
                    .join("data")
                    .join(pkg)
                    .join("cache");
                return Ok(candidate2);
            }
        }

        // Fallback to $HOME/.cache
        if let Ok(home) = env::var("HOME") {
            return Ok(PathBuf::from(home).join(".cache"));
        }

        return Err(CacheDirError::HomeDirUnavailable);
    }

    // iOS: prefer HOME/Library/Caches
    #[cfg(target_os = "ios")]
    {
        if let Ok(home) = env::var("HOME") {
            return Ok(PathBuf::from(home).join("Library").join("Caches"));
        }
        return Err(CacheDirError::HomeDirUnavailable);
    }

    // Windows
    #[cfg(target_os = "windows")]
    {
        // Prefer LOCALAPPDATA
        if let Ok(local) = env::var("LOCALAPPDATA") {
            return Ok(PathBuf::from(local).join("Cache"));
        }
        // Fallback to APPDATA
        if let Ok(appdata) = env::var("APPDATA") {
            return Ok(PathBuf::from(appdata).join("Cache"));
        }
        // Fallback to USERPROFILE
        if let Ok(user) = env::var("USERPROFILE") {
            return Ok(PathBuf::from(user)
                .join("AppData")
                .join("Local")
                .join("Cache"));
        }
        return Err(CacheDirError::HomeDirUnavailable);
    }

    // Unix-like (macOS, Linux, others)
    #[cfg(unix)]
    {
        // macOS
        #[cfg(target_os = "macos")]
        {
            if let Ok(home) = env::var("HOME") {
                return Ok(PathBuf::from(home).join("Library").join("Caches"));
            }
            return Err(CacheDirError::HomeDirUnavailable);
        }

        // Linux and other Unix
        #[cfg(all(unix, not(target_os = "macos")))]
        {
            // XDG_CACHE_HOME preferred
            if let Ok(xdg) = env::var("XDG_CACHE_HOME") {
                return Ok(PathBuf::from(xdg));
            }
            // HOME/.cache
            if let Ok(home) = env::var("HOME") {
                return Ok(PathBuf::from(home).join(".cache"));
            }
            return Err(StorageError::DirNotDeterminable);
        }
    }
}

fn create_dir(path: PathBuf) -> Result<PathBuf, StorageError> {
    if !path.exists() {
        std::fs::create_dir(&path).map_err(|_| StorageError::DirNotCreatable(path.clone()))?;
    }
    Ok(path)
}

/// Get and create the app data directory.
///
/// # Returns
/// Returns the path of the apps data dir.
///
/// # Errors
/// Returns an error when the directory could not be created or the data path could not be
/// determined.
pub fn cache_dir() -> Result<PathBuf, StorageError> {
    create_dir(determine_cache_dir()?)
}
/// Get and create the app data directory.
///
/// # Returns
/// Returns the path of the apps data dir.
///
/// # Errors
/// Returns an error when the directory could not be created or the data path could not be
/// determined.
pub fn app_data_dir() -> Result<PathBuf, StorageError> {
    create_dir(determine_app_data_dir()?)
}
