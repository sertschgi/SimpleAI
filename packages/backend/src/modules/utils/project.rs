use anyhow::Error;
use chrono::{DateTime, Utc};
use serde::{Deserialize, Serialize};
use uuid::Uuid;

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct Project {
    pub id: Uuid,
    pub values: ProjectValues,
}

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct ProjectValues {
    pub name: String,
    pub date: DateTime<Utc>,
    pub desc: String,
    pub author: String,
    pub node: String,
}

impl Project {
    pub fn from_values(name: String, desc: String, author: String, node: Option<String>) -> Self {
        ProjectValues {
            name: name.clone(),
            desc,
            author,
            node: node.unwrap_or(name),
            date: Utc::now(),
        }
        .into()
    }
    pub fn try_from_id(id: Uuid) -> Result<Self, Error> {
        todo!() // TODO: query the project using the id
    }
    pub fn delete(self) -> Self {
        todo!() // TODO: find the path and delete it
    }
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

impl From<ProjectValues> for Project {
    fn from(values: ProjectValues) -> Self {
        Self {
            id: values.id(),
            values,
        }
    }
}
