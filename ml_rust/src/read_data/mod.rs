//! # Read data from file and store the value into vectors
//!
use std::fs::File;
use std::io::{self, BufRead, Error, ErrorKind};
use std::path::Path;

type DoubleVecF64 = Vec<Vec<f64>>;

pub fn get_data(path: &Path) -> Result<(Box<DoubleVecF64>, Box<Vec<f64>>), io::Error> {
    let lines = match File::open(path) {
        Ok(file) => io::BufReader::new(file).lines(),
        Err(ref error) if error.kind() == ErrorKind::NotFound => {
            return Err(Error::new(ErrorKind::NotFound, "File not found"));
        }
        Err(error) if error.kind() == ErrorKind::PermissionDenied => {
            return Err(Error::new(ErrorKind::PermissionDenied, "Permission denied"));
        }
        Err(_) => {
            return Err(Error::new(ErrorKind::Other, "Can not open file."));
        }
    };

    let mut y: Vec<f64> = Vec::new();
    let mut v: Vec<String> = Vec::new();

    // Read the file line by line
    // split each line by the last ',' into two vectors of v and y
    for line in lines {
        if let Some(data_tuple) = line.unwrap().rsplit_once(',') {
            v.push(data_tuple.0.to_string());
            y.push(data_tuple.1.parse::<f64>().expect("Failed"));
        }
    }

    let mut tmp: Vec<Vec<&str>> = Vec::new();

    for i in v.iter() {
        tmp.push(i.split(',').collect::<Vec<&str>>());
    }

    let mut x: Vec<Vec<f64>> = Vec::new();

    for i in tmp.iter() {
        let mut tmp_f64: Vec<f64> = vec![1.0];

        for j in i.iter().map(|e| e.to_string().parse::<f64>()) {
            tmp_f64.push(j.unwrap());

            // match j {
            //     Ok(f) => tmp_f64.push(f),
            //     Err(_) => (),
            // }
        }
        x.push(tmp_f64);
    }

    Ok((Box::new(x), Box::new(y)))
}
