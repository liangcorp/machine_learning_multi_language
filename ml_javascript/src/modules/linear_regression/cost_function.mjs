/**
 * Name: cost_function.mjs
 * Author: Chen Liang
 * Description: My implementation of Machine Learning formulas
 * 				in JavaScript. This is an exercise, so you may
 * 				want to use a professional library for production.
 * Date: 2021-06-15
 */

export const get_cost = function(X, y, theta) {

    let i, j, num_train, sum;

    let num_feat = theta.length;
    let h_x = [];

    let j_theta = 0.0;

    if (X.length === y.length) {
        num_train = X.length;
    }
    else {
        return -1;
    }

    for (let x of X) {
        if (x.length !== theta.length) {
            return -2;
        }
    }

    for (i = 0; i < num_train; i++) {
        sum = 0.0;
        for (j = 0; j < num_feat; j++) {
            sum += X[i][j] * theta[j];
        }
        h_x.push(sum);
    }

    for (j = 0; j < num_train; j++) {
        j_theta += (h_x[j] - y[j]) ** 2 / (2 * num_train);
    }

    return j_theta;
}
