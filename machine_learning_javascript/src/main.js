import { get_determinant, get_invert, normal_equation } from "./modules/linear_regression/normal_equation.mjs";
import { get_cost } from "./modules/linear_regression/cost_function.mjs";
import { get_theta } from "./modules/linear_regression/gradient_descent.mjs";

/*
let matrix = [[3, 8],
            [4, 6]];

let matrix = [[3, 0, 2],
            [2, 0, -2],
            [0, 1, 1]];
*/

let matrix = [[1, 1, 1, -1],
[1, 1, -1, 1],
[1, -1, 1, 1],
[-1, 1, 1, 1]];

// let determinant = get_determinant(matrix);
console.log(get_invert(matrix));
// console.log(get_theta(matrix));
