//! Gradient descent with multiple features
//!
//! This crate is a collection of functions to perform
//! calculation of gradient descent

use std::io;
use std::io::{Error, ErrorKind};

/// # Gradient descent for a single feature (x\[1\])
///
/// - X and y are the training sets.
/// - alpha is the learning rate
/// - theta is a chosen number.
///
/// ## Implement the following matlab formula:
///
///
/// theta(indx,:) = theta(indx,:) -
///                 alpha * ((((temp[] * X[]) - y[]) * X(:,indx))/m);
///
///
pub fn get_thetas(
    x: &[Vec<f64>],
    y: &[f64],
    alpha: f64,
    theta: &mut Vec<f64>,
    num_iters: u32,
) -> Result<Box<Vec<f64>>, io::Error> {
    let num_train = y.len();
    let num_feat = theta.len();

    let mut sum: f64;
    let mut tmp_theta: Vec<f64>;
    let mut h_x: Vec<f64> = Vec::new();

    if x.len() != y.len() {
        return Err(Error::new(ErrorKind::Other, "Mis-matching training sets"));
    }

    for _ in 0..num_iters {
        h_x.clear();

        tmp_theta = theta.clone();

        for i in x.iter().take(num_train) {
            sum = 0.0;

            for j in 0..num_feat {
                sum += tmp_theta[j] * i[j];
            }

            h_x.push(sum);
        }

        for j in 0..num_feat {
            sum = 0.0;

            for i in 0..num_train {
                sum += (h_x[i] - y[i]) * x[i][j];
            }

            theta[j] = tmp_theta[j] - (alpha * sum / (num_train) as f64);
        }
    }

    Ok(Box::new(theta.to_vec()))
}
