//! Simplified Cost function with 1 theta or 1 theta vector
//!
//! This crate is a collection of functions to perform
//! calculation of J (theta 0 and theta 1).

/// # Cost function for a single theta variable
///
/// - X and y are the training sets.
/// - theta is a chosen number.
/// - m is the number of training sets.
///
/// ## Calculate the cost (J) using the following matlab formula:
///
/// ```
/// J = sum(((theta * X[i]) - y).^2 ./(2 * m), "all");
/// ```
///
/// The function returns a 32bit float
///
pub fn cost_function(x: &Vec<f32>, y: &Vec<f32>, theta: &Vec<f32>) -> f32{
    let mut j_theta: f32 = 0.0;     /* The cost */
    let m;

    if x.len() == y.len() {
        m = x.len();
    }
    else {
        panic!("ERROR: training sets have
                different number of elements");
    }

    for i in 0..m {
        j_theta += ((theta[0] + theta[1] * x[i]) - y[i])
                        * ((theta[0] + theta[1] * x[i]) - y[i])
                        / (2 * m) as f32;
    }

    j_theta
}

/// # Cost function for a single theta vector
///
/// - X and y are the training sets.
/// - theta is a vector that contains chosen numbers.
/// - m is the number of training sets.
///
/// ## Calculate the cost (J) using the following matlab formula:
///
/// ```
/// J = sum(((theta[i] * X[j]) - y).^2 ./(2 * m), "all");
/// ```
///
/// The function returns the pointer to a Vector.
/// The vector contains a list of J
///
#[allow(dead_code)]
pub fn cost_function_multiple<'a>(x: &Vec<f32>, y: &Vec<f32>,
                            theta: &Vec<f32>) -> Box<Vec<f32>> {
    let m;
    let n = theta.len();

    let mut j_theta: Vec<f32> = Vec::new();     /* The cost */
    let mut j_theta_single: f32 = 0.0;

    if x.len() == y.len() {
        m = x.len();
    }
    else {
        panic!("ERROR: training sets have
                different number of elements");
    }



    for i in 0..n {
        for j in 0..m {
            j_theta_single += ((theta[i] * x[j]) - y[j])
                                * ((theta[i] * x[j]) - y[j])
                                / (2 * m) as f32;
        }
        j_theta.push(j_theta_single);
    }

    Box::new(j_theta)
}