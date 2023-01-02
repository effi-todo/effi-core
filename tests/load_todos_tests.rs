use effi::TodoManager;

#[test]
fn empty_file_test() {
    let json_str: &str = "[]";

    let mut todo_manager = TodoManager::new("TodoManager");
    todo_manager.todos = TodoManager::load_to_vec(json_str).unwrap();

    assert_eq!(todo_manager.todos.len(), 0);
}
