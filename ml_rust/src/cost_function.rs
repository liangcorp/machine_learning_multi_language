pub fn cost_function(X: &[f32], y: &[f32], theta: &f32, m: &i32]) {
    /*
        Creating the algorithm for the cost function.
        X and y are the training sets.
        theta is a chosen number.
        m is the number of training sets.
        Calculate the cost (J) using the following formula

        J (theta[0], theta[1]) = 1/2m (sum( (h(x[i]) - y[i]) ^ 2) )

     */

    i: i32 = 0;
    J_theta: f32 = 0.0;     /* The cost */

    for (i = 0; i < m; i++)
    {
        J_theta += ((theta * X[i]) - y[i]) * ((theta * X[i]) - y[i]) / (2 * m);
    }

    J_theta
}

pub fn cost_function_multiple(X: &[f32], y: &[f32], theta: &[f32], m: &i32]) {
    /*
        Creating the algorithm for the cost function.
        X and y are the training sets.
        theta is a chosen number.
        m is the number of training sets.
        Calculate the cost (J) using the following formula from matlab

        J = sum(((theta' * X')' - y).^2 ./(2 * m), "all");

     */
}