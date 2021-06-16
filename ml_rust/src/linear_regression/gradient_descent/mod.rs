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

    let num_train = y.len();
    let num_feat = theta.len();

    let mut sum: f32;
    let mut tmp_theta: Vec<f32>;
    let mut h_x: Vec<f32> = Vec::new();

    for _ in 0..num_iters {
        h_x.clear();

        tmp_theta = theta.clone();

        for i in 0..num_train {
            sum = 0.0;

            for j in 0..num_feat {
                sum += tmp_theta[j] * x[i][j];
            }

            h_x.push(sum);
        }

        for j in 0..num_feat {
            sum = 0.0;

            for i in 0..num_train {
                sum += (h_x[i] - y[i]) * x[i][j];
            }

            theta[j] = tmp_theta[j]
                        - (alpha * sum / (num_train as f32));
        }
    }

    Box::new(theta.to_vec())
}