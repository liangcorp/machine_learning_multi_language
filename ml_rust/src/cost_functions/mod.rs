//! Simplified cost function with one feature
//!
//! This crate is a collection of functions to perform
//! calculation of J(theta)
/// # Cost function for multiple features (x\[1\], x\[2\], ..., x\[n\]
///
/// - X and y are the training sets.
/// - X is a 2D Vector represent multiple training sets
/// - theta is a vector that contains chosen numbers.
///
/// ## Implement the following matlab formula:
///
/// ```
/// J = sum(((theta[i] * X[j]) - y).^2 ./(2 * m), "all");
/// ```
///
pub fn get_cost(x: &Vec<Vec<f32>>, y: &Vec<f32>,
                        theta: &Vec<f32>) -> f32 {
    let m;
    let n = theta.len();
    let mut h_x: Vec<f32> = Vec::new();

    let mut j_theta: f32 = 0.0;     /* The cost */
    let mut sum: f32;

    if x.len() == y.len() {
        m = y.len();
    } else {
        panic!("Miss matching number of elements in training sets");
    }

    for i in 0..m {
        if x[i].len() != theta.len() {
            panic!("Missing matching number of elements
                    in theta and X[{}]", i);
        }
    }

    for i in 0..m {
        sum = 0.0;
        for j in 0..n {
            sum += x[i][j] * theta[j];
        }
        h_x.push(sum);
    }

    for j in 0..m {
        j_theta += (h_x[j] - y[j]) * (h_x[j] - y[j]) / (2 * m) as f32;
    }

    j_theta
}