/**
 * Name: ml_modules.js
 * Author: Chen Liang
 * Description: My implementation of Machine Learning formulas
 * 				in JavaScript. This is an exercise, so you may
 * 				want to use a professional library for production.
 * Date: 2021-06-15
 */

const get_cost = function(X, y, theta) {
	let i, j, m, n, sum;
	let h_x = [];

	let j_theta = 0.0;

	if (X.length === y.length) {
		m = X.length;
	}
	else {
		return -1;
	}

	for (let x of X) {
		if (x.length === theta.length) {
			n = theta.length;
		}
		else {
			return -2;
		}
	}

	for (i = 0; i < m; i++) {
		sum = 0.0;
		for (j = 0; j < n; j++) {
			sum += X[i][j] * theta[j];
		}
		h_x.push(sum);
	}

	for (j = 0; j < m; j++){
		j_theta += (h_x[j] - y[j]) ** 2 / (2 * m);
	}

	return j_theta;
}