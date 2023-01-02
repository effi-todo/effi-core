use effi_core::{Todo, TodoManager};

#[test]
fn filter_tags_test() {
    let mut todo_manager = TodoManager::new("Todo Manager");

    let mut todo_one = Todo::new(
        "Todo One",
        "Todo One Desc",
        vec!["one".to_string(), "todo".to_string()],
        None,
    );
    todo_one.id = ['a', 'a', 'a', 'a'];

    let mut todo_two = Todo::new(
        "Todo Two",
        "Todo Two Desc",
        vec!["two".to_string(), "todo".to_string()],
        None,
    );
    todo_two.id = ['a', 'a', 'a', 'b'];

    todo_manager.todos.insert(todo_one.id, todo_one);
    todo_manager.todos.insert(todo_two.id, todo_two);

    assert_eq!(
        todo_manager.filter(None, vec!["one".to_string()])[0],
        ['a', 'a', 'a', 'a']
    );

    assert_eq!(
        todo_manager.filter(None, vec!["two".to_string()])[0],
        ['a', 'a', 'a', 'b']
    );

    assert_eq!(todo_manager.filter(None, vec!["todo".to_string()]).len(), 2);

    assert_eq!(todo_manager.filter(None, vec![]).len(), 0);
}

#[test]
fn filter_query_test() {
    let mut todo_manager = TodoManager::new("Todo Manager");

    let mut todo_one = Todo::new(
        "Todo One",
        "Todo One Desc",
        vec!["one".to_string(), "todo".to_string()],
        None,
    );
    todo_one.id = ['a', 'a', 'a', 'a'];

    let mut todo_two = Todo::new(
        "Todo Two",
        "Todo Two Desc",
        vec!["two".to_string(), "todo".to_string()],
        None,
    );
    todo_two.id = ['a', 'a', 'a', 'b'];

    todo_manager.todos.insert(todo_one.id, todo_one);
    todo_manager.todos.insert(todo_two.id, todo_two);

    assert_eq!(
        todo_manager.filter(Some("Todo".to_string()), vec![]).len(),
        2
    );

    assert_eq!(
        todo_manager.filter(Some("One".to_string()), vec![])[0],
        ['a', 'a', 'a', 'a']
    );

    assert_eq!(
        todo_manager.filter(Some("Two".to_string()), vec![])[0],
        ['a', 'a', 'a', 'b']
    );

    assert_eq!(
        todo_manager.filter(Some("Three".to_string()), vec![]).len(),
        0
    );
}
