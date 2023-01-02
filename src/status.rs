use std::fmt;

use serde::{Deserialize, Serialize};

#[derive(Debug, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub enum TodoStatus {
    Todo,
    InProgress,
    Done,
}

impl fmt::Display for TodoStatus {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match *self {
            TodoStatus::Todo => write!(f, "Todo"),
            TodoStatus::InProgress => write!(f, "In Progress"),
            TodoStatus::Done => write!(f, "Done"),
        }
    }
}
