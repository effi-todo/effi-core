use effi_core::{Todo, TodoManager, TodoStatus};

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
    let todo_one_id = todo_one.id;

    let todo_two = Todo {
        id: todo_one_id,
        parent: None,
        title: "Todo Two".to_string(),
        desc: "Todo Two Desc".to_string(),
        status: TodoStatus::Todo,
        tags: vec![],
    };

    todo_manager.add(todo_one).unwrap();

    match todo_manager.add(todo_two) {
        Ok(_) => panic!("Should have errored; same id"),
        Err(_) => {}
    }
}
