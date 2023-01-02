use effi_core::{generate_id, id_to_string};

#[test]
fn generate_id_test() {
    for _ in 0..100 {
        let id = generate_id();
        println!("{}", id_to_string(&id));
    }
}
