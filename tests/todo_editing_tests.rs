use effi_core::{string_to_id, Todo};

#[test]
fn edit_tags_test() {
    let mut todo = Todo::new("Todo", "Todo Desc", vec!["todo", "one"], None);

    todo.add_tag("added_tag");
    assert_eq!(todo.get_tags().len(), 3);

    todo.remove_tag("todo");
    assert_eq!(todo.get_tags().len(), 2);
}

#[test]
fn edit_children_test() {
    let mut todo = Todo::new("Todo", "Todo Desc", vec!["todo", "one"], None);

    todo.add_child(string_to_id("aaaa").unwrap());
    assert_eq!(todo.get_children().len(), 1);

    todo.remove_child(string_to_id("aaaa").unwrap());
    assert_eq!(todo.get_children().len(), 0);
}
