use effi_core::{string_to_id, Todo, TodoManager};

#[test]
fn filter_tags_test() {
    let mut todo_manager = TodoManager::new("Todo Manager");

    let mut todo_one = Todo::new("Todo One", "Todo One Desc", vec!["one", "todo"], None);
    todo_one.id = string_to_id("aaaa").unwrap();

    let mut todo_two = Todo::new("Todo Two", "Todo Two Desc", vec!["two", "todo"], None);
    todo_two.id = string_to_id("aaab").unwrap();

    todo_manager.add(todo_one).unwrap();
    todo_manager.add(todo_two).unwrap();

    assert_eq!(
        todo_manager.filter(None, vec!["one"], vec![])[0],
        string_to_id("aaaa").unwrap(),
    );

    assert_eq!(
        todo_manager.filter(None, vec!["two"], vec![])[0],
        string_to_id("aaab").unwrap(),
    );

    assert_eq!(todo_manager.filter(None, vec!["todo"], vec![]).len(), 2);

    assert_eq!(todo_manager.filter(None, vec![], vec![]).len(), 0);
}

#[test]
fn filter_query_test() {
    let mut todo_manager = TodoManager::new("Todo Manager");

    let mut todo_one = Todo::new("Todo One", "Todo One Desc", vec!["one", "todo"], None);
    todo_one.id = string_to_id("aaaa").unwrap();

    let mut todo_two = Todo::new("Todo Two", "Todo Two Desc", vec!["two", "todo"], None);
    todo_two.id = string_to_id("aaab").unwrap();

    todo_manager.add(todo_one).unwrap();
    todo_manager.add(todo_two).unwrap();

    assert_eq!(todo_manager.filter(Some("Todo"), vec![], vec![]).len(), 2);

    assert_eq!(
        todo_manager.filter(Some("One"), vec![], vec![])[0],
        string_to_id("aaaa").unwrap(),
    );

    assert_eq!(
        todo_manager.filter(Some("Two"), vec![], vec![])[0],
        string_to_id("aaab").unwrap(),
    );

    assert_eq!(todo_manager.filter(Some("Three"), vec![], vec![]).len(), 0);
}
