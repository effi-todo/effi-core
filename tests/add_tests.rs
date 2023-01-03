use effi_core::{Todo, TodoManager};

#[test]
fn add_test() {
    let mut todo_manager: TodoManager = TodoManager::new("TodoManager");
    assert_eq!(todo_manager.todos.len(), 0);
    todo_manager
        .add(Todo::new("Todo One", "Todo One Desc", vec![], None))
        .unwrap();
    assert_eq!(todo_manager.todos.len(), 1);
}

#[test]
fn already_present_test() {
    let mut todo_manager: TodoManager = TodoManager::new("TodoManager");

    let todo_one = Todo::new("Todo One", "Todo One Desc", vec![], None);
    todo_manager.add(todo_one.clone()).unwrap();

    match todo_manager.add(todo_one) {
        Ok(_) => panic!("Should have errored; same id"),
        Err(_) => {}
    }
}
