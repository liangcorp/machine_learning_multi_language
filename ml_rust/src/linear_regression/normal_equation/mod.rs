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

pub fn get_theta(x: &Vec<Vec<f32>>, y: &Vec<f32>) -> Box<Vec<f32>> {

	let mut result: Vec<f32> = Vec::new();
	let mut mltply_rslt: Vec<Vec<f64>> = Vec::new();
	let mut mltply_rslt_row: Vec<f64> = Vec::new();

	let num_train;
	let num_feat;

	let mut sum: f32;

	if x.len() == y.len() {
		num_train = x.len();
	} else {
		panic!("Miss matching training set numbers");
	}

	/*
		Uncomment the following to test (X * X.transposed):
		let x: Vec<Vec<f32>> = vec![vec![0.0, 1.0],
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

	let mut z: i32;
	let mut determinant: f64 = 0.0;
	let mut tmp_multiply: f64;

	if mltply_rslt.len() == 2 {
		/*
			Calculate the Determinant (der) of 2D matrix
			M = [[A, B],
				 [C, D]]
			Determinant = A * D - B * C
		*/

		determinant = mltply_rslt[0][0] * mltply_rslt[1][1]
					- mltply_rslt[0][1] * mltply_rslt[1][0];
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
			let x: Vec<Vec<f32>> = vec![vec![0.0, 0.1, 0.2],
										vec![1.0, 1.1, 1.2],
										vec![2.0, 2.1, 2.2]];
			let num_train = x.len();
		*/
		for i in 0..num_feat {
			z = i as i32;	// starting column (increase each loop)
			tmp_multiply = 1.0;
			for j in 0..num_feat {

				if z >= num_feat as i32 {
					z = 0;	// Reset to column 0
				}
				// A * E * I + B * F * G + C * D * H
				tmp_multiply *= mltply_rslt[j][z as usize];

				z += 1;		// Move to next column
			}
			// first part of determinant
			determinant += tmp_multiply;
		}

			for i in 0..num_feat {
				z = (num_feat - 1 - i) as i32;
				tmp_multiply = 1.0;
				// starting column (decrease each loop)
				for j in 0..num_feat {

					if z < 0 {
						z = (num_feat - 1) as i32; // Reset to column 0
					}

					// C * E * G - A * F * H - B * D * I
					tmp_multiply *=
						mltply_rslt[num_feat - 1 - j][z as usize];

					z -= 1;		// Move to last column
				}
				// last part of determinant
				determinant -= tmp_multiply;
			}
		}

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
			Support inverting large scale matrix
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
			sum += (invrt_mtrx[i][j] * y_x_trans[j]) as f32;
		}
		result.push(sum);
	}

	Box::new(result)
}