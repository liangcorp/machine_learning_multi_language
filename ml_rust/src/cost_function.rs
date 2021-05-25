pub fn cost_function(X: &[f32], y: &[f32], theta: &f32, m: &i32]) {
    /*
        Creating the algorithm for the cost function.
        X and y are the training sets.
        theta is a chosen number.
        m is the number of training sets.
        Calculate the cost (J) using the following formula

        J (theta[0], theta[1]) = 1/2m (sum( (h(x[i]) - y[i]) ^ 2) )

    */

    let mut i: i32 = 0;
    let mut J_theta: f32 = 0.0;     /* The cost */

    for i in 0..m {
        J_theta += ((theta * X[i]) - y[i]) * ((theta * X[i]) - y[i])
                                                            / (2 * m);
    }

    J_theta
}

pub fn cost_function_multiple(X: &Vec<f32>, y: &[f32], theta: &Vec<f32>, m: &i32]) {
    /*
        Creating the algorithm for the cost function.
        X and y are the training sets.
        theta is a chosen number.
        m is the number of training sets.
        Calculate the cost (J) using the following formula from matlab

        J = sum(((theta' * X')' - y).^2 ./(2 * m), "all");

    */

    let mut i: i32 = 0;

    let mut J_theta: f32 = 0.0;     /* The cost */

    // @TODO

    J_theta
}