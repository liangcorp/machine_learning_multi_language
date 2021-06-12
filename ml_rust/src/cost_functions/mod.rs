pub fn cost_function(x: &Vec<f32>, y: &Vec<f32>, theta: f32) -> f32{
    /*
        Creating the algorithm for the cost function.
        X and y are the training sets.
        theta is a chosen number.
        m is the number of training sets.
        Calculate the cost (J) using the following formula

        J (theta[0], theta[1]) = 1/2m (sum( (h(x[i]) - y[i]) ^ 2) )

    */

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
        j_theta += ((theta * x[i]) - y[i]) * ((theta * x[i]) - y[i])
                                                    / (2 * m) as f32;
    }

    j_theta
}

/* pub fn cost_function_multiple(X: &[f32], y: &[f32],
                            theta: &[f32], m: i32) -> Vec<[f32]> {
    /*
        Creating the algorithm for the cost function.
        X and y are the training sets.
        theta is a chosen number.
        m is the number of training sets.
        Calculate the cost (J) using the following formula from matlab

        J = sum(((theta' * X')' - y).^2 ./(2 * m), "all");

    */

    let mut i: i32 = 0;

    let mut J_theta: f32 = Vec::new();     /* The cost */

    // @TODO

    &J_theta
} */