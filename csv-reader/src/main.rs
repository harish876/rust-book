use csv::{Reader, Writer};
use std::error::Error;
use std::{env, process};
use serde::{Deserialize, Serialize};

#[derive(Debug, Serialize, Deserialize)]
#[serde(rename_all = "PascalCase")]
struct Record {
    city: String,
    state: String,
    population: Option<u64>,
    latitude: f64,
    longitude: f64,
}

struct Args {
    filename: String,
    query: String
}
impl Args {
    fn new(args: Vec<String>) -> Result<Self,Box<dyn Error>> {
        if args.len() !=3 {
            return Err("Enter 2 args, the filename followed by the query".into());
        }
        Ok(Self {
            filename: String::from(&args[1]),
            query: String::from(&args[2])
        })
    }
}

fn run() -> Result<(), Box<dyn Error>> {
    let args = get_args()?;
    let mut rdr = Reader::from_path(args.filename)?;
    let mut wtr = Writer::from_path("output.csv")?;

    wtr.write_record(rdr.headers()?)?;

    for result in rdr.records() {
        let record = result?;
        if record.iter().any(|field| field == &args.query) {
            wtr.write_record(&record)?;
        }
    }
    wtr.flush()?;
    Ok(())
}
fn main() -> Result<(), Box<dyn Error>> {
    if let Err(e) = run() {
        println!("{:#?}", e);
        process::exit(1);
    }
    Ok(())
}

fn get_args() -> Result<Args, Box<dyn Error>> {
    let args: Vec<String> = env::args().collect();
    Ok(Args::new(args)?)
}
