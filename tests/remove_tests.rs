use effi_core::{Todo, TodoManager};

#[test]
fn remove_tags_test() {
    let mut todo_manager = TodoManager::new("Todo Manager");

    let todo_one = Todo::new("Todo One", "Todo One Desc", vec!["one", "todo"], None);
    let todo_two = Todo::new("Todo Two", "Todo Two Desc", vec!["two", "todo"], None);

    todo_manager.add(todo_one).unwrap();
    todo_manager.add(todo_two).unwrap();

    todo_manager.remove(None, vec!["one"], vec![]);

    assert_eq!(todo_manager.todos.len(), 1);
}

#[test]
fn filter_query_test() {
    let mut todo_manager = TodoManager::new("Todo Manager");

    let todo_one = Todo::new("Todo One", "Todo One Desc", vec!["one", "todo"], None);
    let todo_two = Todo::new("Todo Two", "Todo Two Desc", vec!["two", "todo"], None);

    todo_manager.add(todo_one).unwrap();
    todo_manager.add(todo_two).unwrap();

    todo_manager.remove(Some("Todo"), vec![], vec![]);

    assert_eq!(todo_manager.todos.len(), 0);
}
