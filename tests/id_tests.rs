use effi_core::generate_id;

#[test]
fn generate_id_test() {
    for _ in 0..100 {
        let id = generate_id();
        println!("{}{}{}{}", id[0], id[1], id[2], id[3]);
    }
}
