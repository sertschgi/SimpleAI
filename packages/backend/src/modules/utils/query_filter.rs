use crate::modules::utils::prelude::*;
use fuzzy_matcher::{skim::SkimMatcherV2, FuzzyMatcher};
use uuid::Uuid;

#[derive(Clone)]
pub enum NodeQueryFilter {
    Id(Uuid),
    Name(String),
    Older(Date),
    Newer(Date),
    Author(String),
    Environment(Environment),
}

#[derive(Clone)]
pub enum ProjectQueryFilter {
    Id(Uuid),
    Name(String),
    Older(Date),
    Newer(Date),
    Author(String),
    Node(String),
    Desc(String),
}

impl NodeQueryFilter {
    pub fn is_ok(self, node: Node) -> bool {
        match self {
            NodeQueryFilter::Name(name) => {
                let matcher = SkimMatcherV2::default();
                matcher.fuzzy_match(&node.name, &name).is_some()
            }
            NodeQueryFilter::Older(date) => date > node.date,
            NodeQueryFilter::Newer(date) => date < node.date,
            NodeQueryFilter::Author(author) => {
                let matcher = SkimMatcherV2::default();
                matcher.fuzzy_match(&node.author, &author).is_some()
            }
            NodeQueryFilter::Environment(env) => env.merge(&node.version.env).is_ok(),
            NodeQueryFilter::Id(id) => node.id() == id,
        }
    }
}

impl ProjectQueryFilter {
    pub fn is_ok(
        self,
        Project {
            id,
            values:
                ProjectValues {
                    name,
                    node,
                    date,
                    author,
                    desc,
                },
        }: &Project,
    ) -> bool {
        match self {
            ProjectQueryFilter::Id(qid) => qid == *id,
            ProjectQueryFilter::Name(qname) => {
                let matcher = SkimMatcherV2::default();
                matcher.fuzzy_match(name, &qname).is_some()
            }
            ProjectQueryFilter::Node(qnode) => {
                let matcher = SkimMatcherV2::default();
                matcher.fuzzy_match(node, &qnode).is_some()
            }
            ProjectQueryFilter::Older(qdate) => qdate > *date,
            ProjectQueryFilter::Newer(qdate) => qdate < *date,
            ProjectQueryFilter::Author(qauthor) => {
                let matcher = SkimMatcherV2::default();
                matcher.fuzzy_match(author, &qauthor).is_some()
            }
            ProjectQueryFilter::Desc(qdesc) => {
                let matcher = SkimMatcherV2::default();
                matcher.fuzzy_match(desc, &qdesc).is_some()
            }
        }
    }
}
