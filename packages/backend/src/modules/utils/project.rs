use chrono::{DateTime, Utc};
use serde::{Deserialize, Serialize};

// -------------------- PROJECT -------------------- //
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct Project {
    pub name: String,
    pub date: DateTime<Utc>,
    pub desc: String,
    pub author: String,
    pub node: String,
}

impl Project {
    pub fn from_values(name: String, desc: String, author: String, node: Option<String>) -> Self {
        Self {
            name: name.clone(),
            desc,
            author,
            node: node.unwrap_or(name),
            date: Utc::now(),
        }
    }
}
