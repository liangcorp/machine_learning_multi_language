//! Simplified gradient descent with one feature
//!
//! This crate is a collection of functions to perform
//! calculation of gradient descent

// use super::cost_functions;

/// # Gradient descent for a single feature (x\[1\])
///
/// - X and y are the training sets.
/// - alpha is the learning rate
/// - theta is a chosen number.
///
/// ## Implement the following matlab formula:
///
/// ```
/// theta(indx,:) = theta(indx,:) -
///                 alpha * ((((temp[] * X[]) - y[]) * X(:,indx))/m);
/// ```
///

pub fn get_thetas( x: &Vec<Vec<f32>>, y: &Vec<f32>,
                        alpha: f32, theta: &mut Vec<f32>,
                        num_iters: u32) -> Box<Vec<f32>> {

    let m = y.len();
    let n = theta.len();

    let mut sum: f32;
    let mut tmp_theta: Vec<f32> = theta.clone();
    let mut h_x: Vec<f32> = Vec::new();

    for _ in 0..num_iters {
        h_x.clear();

        for j in 0..n {
            tmp_theta[j] = theta[j];
        }

        for i in 0..m {
            sum = 0.0;

            for j in 0..n {
                sum += tmp_theta[j] * x[i][j];
            }

            h_x.push(sum);
        }

        for j in 0..n {
            sum = 0.0;

            for i in 0..m {
                sum += (h_x[i] - y[i]) * x[i][j];
            }

            theta[j] = tmp_theta[j] - (alpha * sum / (m as f32));
        }
    }

    Box::new(theta.to_vec())
}