use std::error;

use crate::Todo;

pub struct TodoManager {
    pub name: String,
    pub todos: Vec<Todo>,
}

impl TodoManager {
    pub fn new(name: impl Into<String>) -> TodoManager {
        TodoManager {
            name: name.into(),
            todos: vec![],
        }
    }

    pub fn load_to_vec(json_str: impl Into<String>) -> Result<Vec<Todo>, Box<dyn error::Error>> {
        let json_str: String = json_str.into();

        return Ok(serde_json::from_str(&json_str)?);
    }

    pub fn dump_to_json(todos: Vec<Todo>) -> Result<String, Box<dyn error::Error>> {
        return Ok(serde_json::to_string(&todos)?);
    }
}
