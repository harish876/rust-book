fn main() {
    match modify_number(None) {
        Some(number) => println!("{:?}",number),
        None => println!("Value is undefined")
    };
}

fn modify_number(number: Option<i32>) -> Option<i32> {
    return number.map(|x| x * 5);
}
/*
    use std::fs::File;
    use std::io::{Result, Read};

    let mut f = File::open("hello.txt")?;
    let mut data = String::new();
    f.read_to_string(&mut data)?;

    for (idx,line) in data.split_terminator('\n').enumerate() {
        match idx % 2 {
            0 => println!("{:?}",line),
            _ => () 
        }
    }

    Ok(())

*/

