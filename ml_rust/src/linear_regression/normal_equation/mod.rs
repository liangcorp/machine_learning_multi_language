//! # Implementation of normal equation
//! Currently only support 2D matrix of X * X.transposed
///
/// Use normal equation to calculate theta
///```
/// theta = (X.trans * X)^-1 * X.trans * y
///```
/// - No need to choose alpha
/// - Don't need to iterate
/// - Slow if number of features is very large (10,000+)
///

fn get_determinant(matrix:&Vec<Vec<f64>>) -> f64 {
	let mut z: i32;
	let mut determinant: f64 = 0.0;
	let mut multiply: f64;
	let num_feat = matrix.len();

	if matrix.len() == 2 {
		/*
			Calculate the Determinant (der) of 2D matrix
			M = [[A, B],
				 [C, D]]
			Determinant = A * D - B * C
		*/

		determinant = matrix[0][0] * matrix[1][1]
					- matrix[0][1] * matrix[1][0];
	} else {
		/*
			Calculate the Determinant (der) of larger scale matrix
			(shortcut method)
			M = [[A, B, C], A, B
				[D, E, F], D, E
				[G, H, I]] G, H;

			der = A * E * I + B * F * G + C * D * H
					- C * E * G - A * F * H - B * D * I;
		*/

		/*
			let x: Vec<Vec<f64>> = vec![vec![0.0, 0.1, 0.2],
										vec![1.0, 1.1, 1.2],
										vec![2.0, 2.1, 2.2]];
			let num_train = x.len();
		*/
		for i in 0..num_feat {
			z = i as i32;	// starting column (increase each loop)
			multiply = 1.0;
			for j in 0..num_feat {

				if z >= num_feat as i32 {
					z = 0;	// Reset to column 0
				}
				// A * E * I + B * F * G + C * D * H
				multiply *= matrix[j][z as usize];

				z += 1;		// Move to next column
			}
			// first part of determinant
			determinant += multiply;
		}

			for i in 0..num_feat {
				z = (num_feat - 1 - i) as i32;
				multiply = 1.0;
				// starting column (decrease each loop)
				for j in 0..num_feat {

					if z < 0 {
						z = (num_feat - 1) as i32; // Reset to column 0
					}

					// C * E * G - A * F * H - B * D * I
					multiply *=
					matrix[num_feat - 1 - j][z as usize];

					z -= 1;		// Move to last column
				}
				// last part of determinant
				determinant -= multiply;
			}
		}

	determinant
}

fn get_invert(matrix: &Vec<Vec<f64>>) -> Box<Vec<Vec<f64>>> {
	let mut result: Vec<Vec<f64>> = Vec::new();

	Box::new(result)
}

pub fn get_theta(x: &Vec<Vec<f64>>, y: &Vec<f64>) -> Box<Vec<f64>> {

	let mut result: Vec<f64> = Vec::new();
	let mut mltply_rslt: Vec<Vec<f64>> = Vec::new();
	let mut mltply_rslt_row: Vec<f64> = Vec::new();

	let num_train;
	let num_feat;

	let mut sum: f64;

	if x.len() == y.len() {
		num_train = x.len();
	} else {
		panic!("Miss matching training set numbers");
	}

	/*
		Uncomment the following to test (X * X.transposed):
		let x: Vec<Vec<f64>> = vec![vec![0.0, 1.0],
									vec![2.0, 3.0]];
		num_train = x.len();

	*/
	num_feat = x[0].len();

	/*
		X = [[A, B],
			 [C, D]];

		X.transposed = [[A ,C],
						[B, D]];

		X * X.transposed = [[AA + BB, AC + BD],
							[CA + DB, CC + DD]]
	*/
	for i in 0..num_feat { // loop for columns
		mltply_rslt_row.clear();
		for j in 0..num_feat {	// loop for the rows of X
			sum = 0.0;
			for z in 0..num_train { // loop for the rows
				sum += x[z][i] * x[z][j];
			}
			mltply_rslt_row.push(sum as f64);	// results for each row
		}
		mltply_rslt.push(mltply_rslt_row.clone());
	}

	let determinant: f64 = get_determinant(&mltply_rslt);

	/*
		Calculate inverted X * X.transposed
	*/
	let mut invrt_mtrx = mltply_rslt.clone();

	if mltply_rslt.len() == 2 {
		/*
			Currently only support 2x2 matrix
		*/
		invrt_mtrx[0][0] = mltply_rslt[1][1] / determinant;
		invrt_mtrx[1][1] = mltply_rslt[0][0] / determinant;
		invrt_mtrx[0][1] = -mltply_rslt[0][1] / determinant;
		invrt_mtrx[1][0] = -mltply_rslt[1][0] / determinant;
	} else {
		/*
			@TODO Support inverting large scale matrix
		*/
	}

	/*
		Calculate y * X.transposed
	*/
	let mut y_x_trans: Vec<f64> = Vec::new();

	for j in 0..num_feat {
		sum = 0.0;
		for i in 0..num_train {
			sum += x[i][j] * y[i];
		}
		y_x_trans.push(sum as f64);
	}

	/*
		Formula for calculating theta
		theta = (X.trans * X)^-1 * X.trans * y

		Because:
			invrt_mtrx = (X.trans * X)^-1
			y_x_trans = X.trans * y

		Therefore:
			theta = invrt_mtrx * y_x_trans
	*/

	for i in 0..num_feat {
		sum = 0.0;
		for j in 0..num_feat {
			sum += (invrt_mtrx[i][j] * y_x_trans[j]);
		}
		result.push(sum);
	}

	Box::new(result)
}