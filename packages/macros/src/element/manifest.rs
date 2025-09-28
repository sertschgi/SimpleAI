// %%% manifest.rs %%% //
// %% includes %%
use serde::{Deserialize, Serialize};
use std::{env, fs, path::PathBuf};

// %% style api %%
#[derive(Serialize, Deserialize)]
pub struct ElementConfig {
    #[serde(default = "default_asset_dir")]
    pub asset_dir: PathBuf,
    #[serde(default = "default_bundle")]
    pub bundle: bool,
    #[serde(default = "default_include_in_src")]
    pub include_in_src: bool,
    #[serde(default = "default_page_as_entry")]
    pub page_as_entry: bool,
    #[serde(default = "default_component_as_entry")]
    pub component_as_entry: bool,
}

impl ElementConfig {
    pub fn new() -> Self {
        let manifest_dir = env::var("CARGO_MANIFEST_DIR").unwrap();
        let config_path = PathBuf::from(manifest_dir).join("Elements.toml");
        let config_content = fs::read_to_string(config_path).unwrap_or_default();
        toml::from_str(&config_content)
            .expect("Couldn't parse the toml from the elements.toml file")
    }
}

// % defaults %
fn default_bundle() -> bool {
    true
}

fn default_include_in_src() -> bool {
    true
}

fn default_asset_dir() -> PathBuf {
    PathBuf::from("assets/style")
}

fn default_component_as_entry() -> bool {
    false
}

fn default_page_as_entry() -> bool {
    false
}
