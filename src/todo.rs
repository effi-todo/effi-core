use serde::{Deserialize, Serialize};

use crate::{generate_id, Id, TodoStatus};

#[derive(Debug, Serialize, Deserialize, Clone)]
#[serde(rename_all = "camelCase")]
pub struct Todo {
    id: Id,
    parent: Option<Id>,
    children: Vec<Id>,
    title: String,
    desc: String,
    status: TodoStatus,
    tags: Vec<String>,
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
            children: vec![],
            title: title.into(),
            desc: desc.into(),
            status: TodoStatus::Todo,
            tags: tags_string,
        }
    }

    pub fn get_id(&self) -> Id {
        return self.id;
    }

    pub fn get_title(&self) -> &String {
        return &self.title;
    }

    pub fn set_title(&mut self, title: impl Into<String>) {
        self.title = title.into();
    }

    pub fn get_desc(&self) -> &String {
        return &self.desc;
    }

    pub fn set_desc(&mut self, desc: impl Into<String>) {
        self.desc = desc.into();
    }

    pub fn get_tags(&self) -> &Vec<String> {
        return &self.tags;
    }

    pub fn add_tag(&mut self, tag: impl Into<String>) {
        self.tags.push(tag.into());
    }

    pub fn remove_tag(&mut self, tag: impl Into<String>) {
        let tag = tag.into();
        self.tags.retain(|t| *t != tag);
    }

    pub fn get_status(&self) -> &TodoStatus {
        return &self.status;
    }

    pub fn set_status(&mut self, status: TodoStatus) {
        self.status = status;
    }

    pub fn get_parent(&self) -> Option<Id> {
        return self.parent;
    }

    pub fn set_parent(&mut self, parent: Option<Id>) {
        self.parent = parent;
    }

    pub fn get_children(&self) -> &Vec<Id> {
        return &self.children;
    }

    pub fn add_child(&mut self, child: Id) {
        self.children.push(child);
    }

    pub fn remove_child(&mut self, child: Id) {
        self.children.retain(|t| *t != child);
    }
}
