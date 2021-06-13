use std::fs::File;
use std::path::Path;
use std::io::{self, ErrorKind, BufRead};
use std::env;
use std::string::String;

mod cost_functions;
mod gradient_descent;
mod feature_scaling;

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
            cost_functions::cost_fn_single(&x, &y, &theta));

    println!("Thetas are [-1.0, 2.0], J(theta) is {}",
            cost_functions::cost_fn_single(&x, &y, &vec![-1.0, 2.0]));

    println!("Found thetas using Gradient Descent: {:?}",
            gradient_descent::gradient_descent(&x, &y,
                                        alpha, &mut theta, 1500));

    println!("Feature scaled X: {:?}",
                            feature_scaling::zero_mean(&x));
    println!("Feature scaled X: {:?}",
                            feature_scaling::mean_normalization(&x));


    // I just needed some samples. Don't judge me
    let ex2_sample_x: Vec<Vec<f32>> = vec![vec![1.0, 2104.0, 3.0],
                                        vec![1.0, 1600.0, 3.0],
                                        vec![1.0, 2400.0, 3.0],
                                        vec![1.0, 1416.0, 2.0],
                                        vec![1.0, 3000.0, 4.0],
                                        vec![1.0, 1985.0, 4.0],
                                        vec![1.0, 1534.0, 3.0],
                                        vec![1.0, 1427.0, 3.0],
                                        vec![1.0, 1380.0, 3.0],
                                        vec![1.0, 1494.0, 3.0]];
    let ex2_sample_y: Vec<f32> = vec![399900.0,
                                        329900.0,
                                        369000.0,
                                        232000.0,
                                        539900.0,
                                        299900.0,
                                        314900.0,
                                        198999.0,
                                        212000.0,
                                        242500.0];

    let ex2_theta: Vec<f32> = vec![0.0, 0.0, 0.0];


    println!("Multi-featured cost function: {:?}",
    cost_functions::cost_fn_multiple(&ex2_sample_x,
                                    &ex2_sample_y,
                                    &ex2_theta));
}
