use data_structures_rust::binary_search;
fn main() {
    let list = vec![10, 9, 8, 7, 6, 5];
    if let Some(idx) = binary_search(0, list.len() - 1, 10, list) {
        println!("The target is found at position {}",idx);
    }
}
