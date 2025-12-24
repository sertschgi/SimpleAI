use anyhow::Error;
use chrono::{DateTime, Utc};
use serde::{Deserialize, Serialize};
use simple_ai_macros::Formifiable;
use uuid::Uuid;

use crate::modules::{projects::query::query_projects, utils::query_filter::*};

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct Project {
    pub id: Uuid,
    pub values: ProjectValues,
}

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize, Formifiable)]
pub struct ProjectValues {
    pub name: String,
    #[ffignore(all)]
    pub date: DateTime<Utc>,
    pub desc: String,
    pub author: String,
    #[ffignore(all)]
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
        query_projects(vec![ProjectQueryFilter::Id(id)])
            .first()
            .ok_or(Error::msg(format!("No project with id: {id} found")))
            .cloned()
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
