/**
 * Name: gradient_descent.mjs
 * Author: Chen Liang
 * Description: My implementation of Machine Learning formulas
 * 				in JavaScript. This is an exercise, so you may
 * 				want to use a professional library for production.
 * Date: 2021-06-15
 */

export const get_theta = function(X, y, alpha,
    theta, num_iters) {

    let sum, tmp_theta;

    let num_train = y.length;
    let num_feat = theta.length;
    let h_x = [];

    while (num_iters > 0) {
        h_x = [];

        tmp_theta = theta;

        for (i = 0; i < num_train; i++) {
            sum = 0.0;

            for (j = 0; j < num_feat; j++) {
                sum += tmp_theta[j] * X[i][j];
            }

            h_x.push(sum);
        }

        for (j = 0; j < num_feat; j++) {
            sum = 0.0;

            for (i = 0; i < num_train; i++) {
                sum += (h_x[i] - y[i]) * X[i][j];
            }

            theta[j] = tmp_theta[j] - (alpha * sum / num_train);
        }

        num_iters--;
    }

    return theta;
}
