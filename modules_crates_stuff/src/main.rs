pub mod garden;

fn main() {
    println!("Hello, world!");
    let vegetable = garden::Vegetable::new(String::from("Green"),String::from("Large"));
    println!("{:#?}",vegetable);
    garden::print_garden()
}
