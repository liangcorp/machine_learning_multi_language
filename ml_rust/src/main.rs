use std::fs::File;
use std::path::Path;
use std::io::{self, ErrorKind, BufRead};
use std::env;
use std::string::String;

mod cost_functions;
mod gradient_descent;

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

    // Read the file line by line
    // split each line by ',' into two vectors of f32
    for line in lines {
        match line {
            Ok(line) => {
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

    let alpha = 0.01;   // the learning speed
    let mut theta = vec![0.0, 0.0]; // set theta 0 and theta 1 to 0.0

    println!("Thetas are [0.0, 0.0], J(theta) is {}",
            cost_functions::cost_function(&x, &y, &theta));

    println!("Thetas are [-1.0, 2.0], J(theta) is {}",
            cost_functions::cost_function(&x, &y, &vec![-1.0, 2.0]));

    println!("Found thetas using Gradient Descent: {:?}",
            gradient_descent::gradient_descent(&x, &y,
                                        alpha, &mut theta));
}
