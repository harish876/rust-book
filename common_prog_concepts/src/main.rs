use std::io;

fn celsius_to_farenheit(degree_in_celsius: f64) -> f64 {
    9.0 / 5.0 * degree_in_celsius + 32.0
}
fn run_celsius_to_farenheit() {
    println!("Enter a Degree in celsius: ");

    let mut celsius_input = String::new();

    io::stdin()
        .read_line(&mut celsius_input)
        .expect("Error reading input");

    let celsius_input: f64 = celsius_input.trim().parse().expect("Enter a Valid Number");

    println!(
        "The Equivalent Farenheit Temparature for {celsius_input} C is {}",
        celsius_to_farenheit(celsius_input)
    );
}

fn generate_fibonacci(number: i32) -> i32 {
    if number == 0 || number == 1 {
        return number;
    }
    return generate_fibonacci(number - 1) + generate_fibonacci(number - 2);
}
fn run_fibonacci() {
    println!("Enter a Number : ");

    let mut f_input = String::new();

    io::stdin()
        .read_line(&mut f_input)
        .expect("Error reading input");

    let f_input = f_input.trim().parse().expect("Enter a Valid Number");

    println!(
        "The Fibonacci number for {f_input} is {}",
        generate_fibonacci(f_input)
    );
}
fn main() {
    run_fibonacci();
    run_celsius_to_farenheit();
}
