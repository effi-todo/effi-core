use serde::{Deserialize, Serialize};

use crate::{generate_id, Id, TodoStatus};

#[derive(Debug, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct Todo {
    pub id: Id,
    pub parent: Option<Id>,
    pub title: String,
    pub desc: String,
    pub status: TodoStatus,
    pub tags: Vec<String>,
}

impl Todo {
    /// Create a new todo using the given title, description and parent
    pub fn new(
        title: impl Into<String>,
        desc: impl Into<String>,
        tags: Vec<&str>,
        parent: Option<Id>,
    ) -> Todo {
        let mut tags_string: Vec<String> = vec![];
        for tag in tags {
            tags_string.push(tag.to_string());
        }

        Todo {
            id: generate_id(),
            parent: match parent {
                Some(parent) => Some(parent.into()),
                None => None,
            },
            title: title.into(),
            desc: desc.into(),
            status: TodoStatus::Todo,
            tags: tags_string,
        }
    }
}
