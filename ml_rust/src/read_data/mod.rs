//! # Read data from file and store the value into vectors
//!
use std::fs::File;
use std::path::Path;
use std::io::{self, ErrorKind, BufRead};

pub fn get_data(path: &Path) -> (Box<Vec<Vec<f32>>>, Box<Vec<f32>>) {
    let lines = match File::open(path) {
        Ok(file) => io::BufReader::new(file).lines(),
        Err(ref error) if error.kind() == ErrorKind::NotFound => {
            panic!("File not found");
        },
        Err(error) => {
            panic!("Unable to open file {:?}", error);
        }
    };


    let mut y: Vec<f32> = Vec::new();
    let mut v: Vec<String> = Vec::new();

    // Read the file line by line
    // split each line by the last ',' into two vectors of v and y
    for line in lines {
        match line {
            Ok(line) => {
                match line.rsplit_once(',') {
                    Some(data_tuple) => {
                        v.push(data_tuple.0.to_string());
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

    let mut tmp: Vec<Vec<&str>> = Vec::new();

    for i in v.iter() {
        tmp.push(i.split(',').collect::<Vec<&str>>());
    }

    let mut x: Vec<Vec<f32>> = Vec::new();

    for i in tmp.iter() {
        let mut tmp_f32: Vec<f32> = Vec::new();
        tmp_f32.push(1.0);
        for j in i.into_iter().map(|e| {
                e.to_string().parse::<f32>()
            }) {
            match j {
                Ok(f) => tmp_f32.push(f),
                Err(_) => (),
            }
        }
        x.push(tmp_f32);
    }

    (Box::new(x), Box::new(y))
}