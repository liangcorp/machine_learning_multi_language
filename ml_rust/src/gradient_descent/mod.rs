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
                                mut theta: f32) -> f32 {
    let m = y.len();

    let mut sum: f32;
    let mut j_theta: f32;
    let mut tmp_j_theta: f32;

    j_theta = cost_functions::cost_function(&x, &y, theta);

    loop {
        sum = 0.0;

        for j in 0..m {
            sum += ((theta * x[j]) - y[j]) * x[j]
        }

        theta = theta - alpha * sum / m as f32;

        tmp_j_theta = cost_functions::cost_function(&x, &y, theta);

        if j_theta > tmp_j_theta {
            j_theta = tmp_j_theta;
        } else {
            break;
        }

        println!("{}", theta);
    }

    theta
}