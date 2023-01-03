use effi_core::{Todo, TodoManager};

#[test]
fn filter_tags_test() {
    let mut todo_manager = TodoManager::new("Todo Manager");

    let todo_one = Todo::new("Todo One", "Todo One Desc", vec!["one", "todo"], None);
    let todo_two = Todo::new("Todo Two", "Todo Two Desc", vec!["two", "todo"], None);

    let todo_one_id = todo_one.get_id();
    let todo_two_id = todo_two.get_id();

    todo_manager.add(todo_one).unwrap();
    todo_manager.add(todo_two).unwrap();

    assert_eq!(
        todo_manager.filter(None, vec!["one"], vec![])[0],
        todo_one_id
    );

    assert_eq!(
        todo_manager.filter(None, vec!["two"], vec![])[0],
        todo_two_id,
    );

    assert_eq!(todo_manager.filter(None, vec!["todo"], vec![]).len(), 2);

    assert_eq!(todo_manager.filter(None, vec![], vec![]).len(), 0);
}

#[test]
fn filter_query_test() {
    let mut todo_manager = TodoManager::new("Todo Manager");

    let todo_one = Todo::new("Todo One", "Todo One Desc", vec!["one", "todo"], None);
    let todo_two = Todo::new("Todo Two", "Todo Two Desc", vec!["two", "todo"], None);

    let todo_one_id = todo_one.get_id();
    let todo_two_id = todo_two.get_id();

    todo_manager.add(todo_one).unwrap();
    todo_manager.add(todo_two).unwrap();

    assert_eq!(todo_manager.filter(Some("Todo"), vec![], vec![]).len(), 2);

    assert_eq!(
        todo_manager.filter(Some("One"), vec![], vec![])[0],
        todo_one_id,
    );

    assert_eq!(
        todo_manager.filter(Some("Two"), vec![], vec![])[0],
        todo_two_id,
    );

    assert_eq!(todo_manager.filter(Some("Three"), vec![], vec![]).len(), 0);
}
