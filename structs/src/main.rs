#[derive(Debug)]
struct Rectangle {
    width: i32,
    length: i32
}
impl Rectangle {
    fn new(length: i32,width:i32) -> Self {
        Self {
            length,
            width
        }
    }

    fn area(&self) -> i32 {
        self.length * self.width
    }
}
fn main() {
    let rect = Rectangle {
        width:10,
        length:20
    };

    let rect1 = Rectangle::new(25,50);
    println!("The Rectangle is {:#?}",rect);
    println!("The Area of Rectangle is {} square units",rect.area());

    println!("The Rectangle is {:#?}",rect1);
    println!("The Area of Rectangle is {} square units",rect1.area());
}