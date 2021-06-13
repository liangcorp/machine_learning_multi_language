//! Simplified cost function with one feature
//!
//! This crate is a collection of functions to perform
//! calculation of J(theta)

/// # Cost function for a single feature (x\[1\])
///
/// - X and y are the training sets.
/// - theta is a chosen number.
///
/// ## Implement the following matlab formula:
///
/// ```
/// J = sum(((theta * X[i]) - y).^2 ./(2 * m), "all");
/// ```
///
pub fn cost_function(x: &Vec<f32>,
                        y: &Vec<f32>,
                        theta: &Vec<f32>) -> f32{
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

/// # Cost function for multiple features (x\[1\], x\[2\], ..., x\[n\]
///
/// - X and y are the training sets.
/// - theta is a vector that contains chosen numbers.
///
/// ## Implement the following matlab formula:
///
/// ```
/// J = sum(((theta[i] * X[j]) - y).^2 ./(2 * m), "all");
/// ```
///
#[allow(dead_code)]
pub fn cost_function_multiple<'a>(x: &Vec<f32>,
                                    y: &Vec<f32>,
                                    theta: &Vec<f32>)
                                    -> Box<Vec<f32>> {
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