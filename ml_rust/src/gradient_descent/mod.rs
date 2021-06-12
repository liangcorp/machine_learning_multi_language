//! Implementation of Gradient Descent Algorithms
//!
//! This crate is a collection of functions to perform
//! calculation of gradient descent

/// # Gradient descent for a single theta vector
///
///
/// ## Calculate the cost (J) using the following matlab formula:
///
/// ```
/// theta(indx,:) = theta(indx,:) -
///                 alpha * ((((temp' * X') - y') * X(:,indx))/m);
/// ```
///
/// The function returns a 32bit float
///
use super::cost_functions;

pub fn gradient_descent_single( x: &Vec<f32>,
                                y: &Vec<f32>,
                                alpha: f32,
                                theta: &mut Vec<f32>)
                                -> Box<Vec<f32>> {
    let m = y.len();

    let mut sum: f32;
    let mut j_theta: f32;
    let mut tmp_j_theta: f32;
    let mut tmp_theta = theta.clone();

    j_theta = cost_functions::cost_function(&x, &x, &theta);

    loop {
        tmp_theta[0] = theta[0];
        tmp_theta[1] = theta[1];

        sum = 0.0;

        for i in 0..m {
            sum += (tmp_theta[1] * x[i]) - y[i];
        }

        theta[0] = theta[0] - alpha * sum / m as f32;

        sum = 0.0;

        for i in 0..m {
            sum += ((tmp_theta[1] * x[i]) - y[i]) * x[i];
        }

        theta[1] = theta[1] - alpha * sum / m as f32;

        tmp_j_theta = cost_functions::cost_function(&x, &y, theta);

        if j_theta > tmp_j_theta {
            j_theta = tmp_j_theta;
        } else {
            break;  // Break loop if found minium J theta
        }
    }

    Box::new(theta.to_vec())
}