//! Cost function with multiple features
//!
//! This crate is a collection of functions to perform
//! calculation of J(theta)

use std::io;
use std::io::{Error, ErrorKind};

/// # Cost function for multiple features (x\[1\], x\[2\], ..., x\[n\]
///
/// - X and y are the training sets.
/// - X is a 2D Vector represent multiple training sets
/// - theta is a vector that contains chosen numbers.
///
/// ## Implement the following matlab formula:
///
///
/// J = sum(((theta[i] * X[j]) - y).^2 ./(2 * m), "all");
///

pub fn get_cost(x: &[Vec<f64>], y: &[f64], theta: &[f64]) -> Result<f64, io::Error> {
    let num_feat = theta.len();
    let mut h_x: Vec<f64> = Vec::new();

    let mut j_theta: f64 = 0.0; /* The cost */
    let mut sum: f64;

    let num_train = if x.len() == y.len() {
        y.len()
    } else {
        return Err(Error::new(ErrorKind::Other, "Mis-matching training sets"));
    };

    for i in x.iter().enumerate().take(num_train) {
        if i.1.len() != num_feat {
            panic!(
                "Missing matching number of elements in theta and X[{}]",
                i.0
            );
        }
    }

    for i in x.iter().enumerate().take(num_train) {
        sum = 0.0;
        for j in i.1.iter().enumerate().take(num_feat) {
            sum += *j.1 * theta[j.0];
        }
        h_x.push(sum);
    }

    // for i in 0..num_train {
    //     sum = 0.0;
    //     for j in 0..num_feat {
    //         sum += x[i][j] * theta[j];
    //     }
    //     h_x.push(sum);
    // }

    for j in 0..num_train {
        j_theta += (h_x[j] - y[j]) * (h_x[j] - y[j]) / (2 * num_train) as f64;
    }

    Ok(j_theta)
}
