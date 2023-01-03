use std::{collections::HashMap, error};

use serde::{Deserialize, Serialize};

use crate::{id_to_string, EffiCoreError, Id, Todo};

#[derive(Serialize, Deserialize)]
pub struct TodoManager {
    /// The name of the tood list
    pub name: String,
    /// The todos that this Manager manages
    pub todos: HashMap<Id, Todo>,
}

impl TodoManager {
    /// Create a new todo manager with the given name
    pub fn new(name: impl Into<String>) -> TodoManager {
        TodoManager {
            name: name.into(),
            todos: HashMap::new(),
        }
    }

    pub fn add(&mut self, todo: Todo) -> Result<(), Box<dyn error::Error>> {
        if self.todos.contains_key(&todo.get_id()) {
            return Err(Box::new(EffiCoreError::IdAlreadyTaken(id_to_string(
                &todo.get_id(),
            ))));
        };

        self.todos.insert(todo.get_id(), todo);

        Ok(())
    }

    pub fn remove(&mut self, query: Option<&str>, tags: Vec<&str>, ids: Vec<Id>) {
        let ids_to_remove = self.filter(query, tags, ids);

        for id in &ids_to_remove {
            self.todos.remove(id);
        }
    }

    pub fn filter(&self, query: Option<&str>, tags: Vec<&str>, ids: Vec<Id>) -> Vec<Id> {
        let mut filtered_todos: Vec<Id> = ids;

        'todo_loop: for (id, todo) in &self.todos {
            for tag in &tags {
                if todo.get_tags().contains(&tag.to_string()) {
                    filtered_todos.push(*id);
                    continue 'todo_loop;
                }
            }

            match query {
                Some(ref query) => {
                    if todo.get_title().contains(query) || todo.get_desc().contains(query) {
                        filtered_todos.push(*id);
                        continue;
                    }
                }
                None => continue,
            }
        }

        return filtered_todos;
    }

    pub fn load_to_manager(
        json_str: impl Into<String>,
    ) -> Result<TodoManager, Box<dyn error::Error>> {
        let json_str: String = json_str.into();

        return Ok(serde_json::from_str(&json_str)?);
    }

    pub fn dump_to_json(todos: HashMap<Id, Todo>) -> Result<String, Box<dyn error::Error>> {
        return Ok(serde_json::to_string(&todos)?);
    }
}
