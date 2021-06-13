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
pub fn gradient_descent( x: &Vec<f32>,
                         y: &Vec<f32>,
                         alpha: f32,
                         theta: &mut Vec<f32>,
                         num_iters: u32) -> Box<Vec<f32>> {
    let m = y.len();

    let mut sum: f32;
    let mut tmp_theta = theta.clone();

    for _ in 0..num_iters {
        tmp_theta[0] = theta[0];
        tmp_theta[1] = theta[1];

        sum = 0.0;

        for i in 0..m {
            sum += (tmp_theta[0] * x[i]) - y[i];
        }

        theta[0] = theta[0] - alpha * sum / m as f32;

        sum = 0.0;

        for i in 0..m {
            sum += ((tmp_theta[1] * x[i]) - y[i]) * x[i];
        }

        theta[1] = theta[1] - alpha * sum / m as f32;
    }

    Box::new(theta.to_vec())
}