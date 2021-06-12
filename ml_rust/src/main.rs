use std::fs::File;
use std::path::Path;
use std::io::{self, ErrorKind, BufRead};
use std::env;
use std::string::String;

mod cost_functions;

fn main() {
    let args: Vec<String> = env::args().collect();

    let path = Path::new(&args[1]);

    let lines = match File::open(path) {
        Ok(file) => io::BufReader::new(file).lines(),
        Err(ref error) if error.kind() == ErrorKind::NotFound => {
            panic!("File not found");
        },
        Err(error) => {
            panic!("Unable to open file {:?}", error);
        }
    };

    let mut x: Vec<f32> = Vec::new();
    let mut y: Vec<f32> = Vec::new();

    for line in lines {
        match line {
            Ok(line) => {
                // println!("{:?}", line.split_once(','));
                match line.split_once(',') {
                    Some(data_tuple) => {
                        x.push(data_tuple.0.parse::<f32>()
                                            .expect("Failed"));
                        y.push(data_tuple.1.parse::<f32>()
                                            .expect("Failed"));
                    },
                    None => (),
                }
            }
            Err(error) => {
                println!("ERROR: {}", error);
            }
        }
    }

    println!("J theta is {}",
            cost_functions::cost_function(&x, &y, 1.0));
}
