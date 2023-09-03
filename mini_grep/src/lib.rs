use std::io::Read;
use std::{env, error::Error, fs::File};

#[derive(Debug)]
pub struct UserInput {
    path: String,
    search_text: String,
}
impl UserInput {
    fn new(args: Vec<String>) -> Result<Self, Box<dyn Error>> {
        if args.len() != 3 {
            return Err("Number of Args wrong".into());
        }
        Ok(Self {
            path: String::from(&args[1]),
            search_text: String::from(&args[2]),
        })
    }
}
fn search_by_line(file_contents: &String, search_text: String) -> Vec<String> {
    let mut result: Vec<String> = Vec::new();
    for line in file_contents.split_terminator('\n') {
        match line.to_lowercase().find(&search_text.to_lowercase()) {
            Some(_) => result.push(String::from(line)),
            None => {}
        }
    }
    result
}

fn get_file(path: String) -> Result<File, Box<dyn Error>> {
    let f = File::open(path)?;
    Ok(f)
}

fn read_file(mut file: File) -> Result<String, Box<dyn Error>> {
    let mut read_buff = String::new();
    file.read_to_string(&mut read_buff)?;

    Ok(read_buff)
}
pub fn run() -> Result<(), Box<dyn Error>>{
    let args: Vec<String> = env::args().collect();

    let user_input = UserInput::new(args)?;

    //debug
    println!("{:#?}", user_input);

    let file = get_file(user_input.path)?;
    let file_contents = read_file(file)?;
    let lines_found = search_by_line(&file_contents, user_input.search_text);

    println!("The Line which match the entered search text:");
    for i in &lines_found {
        println!("{}", i)
    }
    Ok(())
}