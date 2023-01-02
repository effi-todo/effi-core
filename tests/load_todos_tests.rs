use effi_core::TodoManager;

#[test]
fn empty_file_test() {
    let json_str: &str = "{\"name\": \"TodoManager\", \"todos\": {}}";

    let todo_manager = TodoManager::load_to_manager(json_str).unwrap();

    assert_eq!(todo_manager.todos.len(), 0);
    assert_eq!(todo_manager.name, "TodoManager".to_string());
}
