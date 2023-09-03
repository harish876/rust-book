fn main() {
    let mut number_list = vec![1,2,3];
    let iter_1 = number_list.iter_mut();
    for i in iter_1 {
        *i = *i+1;
        println!("{}",i);
    }
}
