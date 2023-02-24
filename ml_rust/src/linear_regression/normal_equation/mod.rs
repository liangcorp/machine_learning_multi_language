//! # Implementation of normal equation
use std::io;
use std::io::{Error, ErrorKind};

///
/// # Calculate the Determinant (der) of a matrix
///
/// Calculate the Determinant (der) of larger scale matrix.
/// Tested for 2x2, 3x3 and 4x4 matrixes.
/// Implementation is not optimized for super large matrixes.
///
pub fn get_determinant(matrix: &[Vec<f64>]) -> Result<f64, io::Error> {
    let mut z: i32;
    let mut determinant: f64 = 0.0;
    let mut multiply: f64;
    let num_feat = matrix.len();

    if matrix.len() != matrix[0].len() {
        return Err(Error::new(ErrorKind::Other, "Matrix not symmetrical"));
    }

    if matrix.len() == 2 {
        // 2x2 matrixes
        /*
            Calculate the Determinant (der) of 2D matrix
            M = [[A, B],
                 [C, D]]
            Determinant = A * D - B * C
        */

        determinant = matrix[0][0] * matrix[1][1] - matrix[0][1] * matrix[1][0];
    } else if matrix.len() == 3 {
        // 3x3 matrixes
        /*
            Calculate the Determinant (der) of larger scale matrix
            (shortcut method)
            M = [[A, B, C]
                [D, E, F]
                [G, H, I]]

            der = A * E * I + B * F * G + C * D * H
                    - C * E * G - A * F * H - B * D * I;
        */

        for i in 0..num_feat {
            z = i as i32; // starting column (increase each loop)
            multiply = 1.0;
            for j in matrix.iter().take(num_feat) {
                if z >= num_feat as i32 {
                    z = 0; // Reset to column 0
                }

                multiply *= j[z as usize];
                z += 1; // Move to next column
            }
            // A * E * I + B * F * G + C * D * H
            determinant += multiply;
        }

        for i in 0..num_feat {
            z = (num_feat - 1 - i) as i32;
            multiply = 1.0;
            // starting column (decrease each loop)
            for j in matrix.iter().take(num_feat) {
                if z < 0 {
                    z = (num_feat - 1) as i32; // Reset to column 0
                }

                multiply *= j[z as usize];
                z -= 1; // Move to last column
            }
            // C * E * G - A * F * H - B * D * I
            determinant -= multiply;
        }
    } else {
        // 4x4 and above matrix
        /*
            Use self calling function to further reduce the
            length of matrix.

            E.g. Divide 4x4 into 4 3x3 matrix using cofactor of
            first element of each row.
            Calculate the determinant of 3x3.
            Return the 4 results upstream for calculation of
            determinant of 4x4.

            A = [[0.0, 0.1, 0.2, 0.3],
                 [1.0, 1.1, 1.2, 1.3],
                 [2.0, 2.1, 2.2, 2.3],
                 [3.0, 3.1, 3.2, 3.3]]

            Calculate the determinant of following 3x3 matrixes (D[3])

            Cof_A[0][0] = [[1.1, 1.2, 1.3],
                             [2.1, 2.2, 2.3],
                             [3.1, 3.2, 3.3]]

            Cof_A[1][0] = [[0.1, 0.2, 0.3],
                           [2.1, 2.2, 2.3],
                           [3.1, 3.2, 3.3]]

            Cof_A[2][0] = [[0.1, 0.2, 0.3],
                           [1.1, 1.2, 1.3],
                           [3.1, 3.2, 3.3]]

            Cof_A[3][0] = [[0.1, 0.2, 0.3],
                           [1.1, 1.2, 1.3],
                           [2.1, 2.2, 2.3]]

            Formula for calculating determinant of 4x4 matrix:
            |A| = Cof_A[0][0] * D[0] - Cof_A[1][0] * D[1]
                + Cof_A[2][0] * D[2] - Cof_A[3][0] * D[3]
        */
        let mut cofactor_row: Vec<f64> = Vec::new();
        let mut cofactor: Vec<Vec<f64>> = Vec::new();
        let mut first_col_elemnt: Vec<f64> = Vec::new();
        let mut deter_list: Vec<f64> = Vec::new();

        let j = 0;
        for i in 0..num_feat {
            // Save first element of each row
            first_col_elemnt.push(matrix[i][j]);

            /*
                Create cofactor matrix for first element
                of each row.
            */
            for m in matrix.iter().enumerate().take(num_feat) {
                for n in 0..num_feat {
                    if m.0 != i && n != j {
                        cofactor_row.push(m.1[n]);
                    }
                }
                if !cofactor_row.is_empty() {
                    cofactor.push(cofactor_row.clone());
                    cofactor_row.clear();
                }
            }

            // List of determinants for cofactor matrixes
            let der = match get_determinant(&cofactor) {
                Ok(determinant) => determinant,
                Err(e) => return Err(e),
            };

            deter_list.push(der);
            cofactor.clear();
        }

        /*
            Performing the formula for calculating determinant
            of large scale matrix.

            Sample formula for 4x4 matrix:
            |A| = Cof_A[0][0] * D[0] - Cof_A[1][0] * D[1]
                + Cof_A[2][0] * D[2] - Cof_A[3][0] * D[3]
        */
        for i in 0..first_col_elemnt.len() {
            if i % 2 == 0 {
                determinant += first_col_elemnt[i] * deter_list[i];
            } else {
                determinant -= first_col_elemnt[i] * deter_list[i];
            }
        }
    }

    Ok(determinant)
}

///# Calculate inverted matrix from provided matrix
/// Currently using adjugate matrix
/*
   Formula:
   A^-1 = 1/|A| * (A~)

   Sample 3x3 matrix:
       A = [[3.0, 0.0, 2.0],
           [2.0, 0.0, -2.0],
           [0.0, 1.0, 1.0]]

   Determinant = 10

   Matrix of Minors = [[0*1 - (-2)*1,	2*1 - (-2)*0,	2*1 - 0*0],
                       [0*1 - 2*1,	3*1 - 2*0,	3*1 - 0*0],
                       [0*(-2) - 2*0,	3*(-2) - 2*2,	3*0 - 0*2]]

                   = [[2, 2, 2],
                   [-2, 3, 3],
                   [0, -10, 0]]

   Matrix of Cofactors = [[2, -2, 2],
                           [2, 3, -3],
                           [0, 10, 0]]

   Transposed Matrix = [[2, 2, 0],
                       [-2, 3, 10]
                       [2, -3, 0]]

       Inverted Matrix = 1/determinant * Transposed Matrix

   A^-1 = [[2, 2, 0],
           [-2, 3, 10]
           [2, -3, 0]] / 10

       = [[0.2, 0.2, 0],
           [-0.2, 0.3, 1],
           [0.2, -0.3, 0]]
*/

pub fn get_invert(matrix: &[Vec<f64>]) -> Result<Box<Vec<Vec<f64>>>, io::Error> {
    let mut mtrx_result: Vec<Vec<f64>>;
    let determinant: f64 = match get_determinant(matrix) {
        Ok(determinant) => determinant,
        Err(e) => return Err(e),
    };

    if matrix.len() == 2 {
        mtrx_result = matrix.to_vec();

        mtrx_result[0][0] = matrix[1][1] / determinant;
        mtrx_result[1][1] = matrix[0][0] / determinant;
        mtrx_result[0][1] = -matrix[0][1] / determinant;
        mtrx_result[1][0] = -matrix[1][0] / determinant;
    } else {
        let mut mtrx_der: Vec<Vec<f64>> = Vec::new();
        let mut mtrx_der_row: Vec<f64> = Vec::new();

        let mut mtrx_minors: Vec<Vec<f64>> = Vec::new();
        let mut mtrx_minors_row: Vec<f64> = Vec::new();

        let mut mtrx_trans: Vec<Vec<f64>>;

        let row = matrix.len();
        let col = matrix[0].len();

        if row != col {
            return Err(Error::new(ErrorKind::Other, "Matrix not symmetrical"));
        }

        // Calculate matrix of minors
        for i in 0..row {
            for j in 0..col {
                for m in matrix.iter().enumerate().take(row) {
                    for n in 0..col {
                        if m.0 != i && n != j {
                            mtrx_der_row.push(m.1[n]);
                        }
                    }
                    if !mtrx_der_row.is_empty() {
                        mtrx_der.push(mtrx_der_row.clone());
                    }
                    mtrx_der_row.clear();
                }
                let tmp_der: f64 = match get_determinant(&mtrx_der) {
                    Ok(determinant) => determinant,
                    Err(e) => return Err(e),
                };

                mtrx_minors_row.push(tmp_der);
                mtrx_der.clear();
            }
            mtrx_minors.push(mtrx_minors_row.clone());
            mtrx_minors_row.clear();
        }
        // println!("Matrix minor: {:?}", mtrx_minors);

        // Matrix of Co-factors
        for i in mtrx_minors.iter_mut().enumerate().take(row) {
            for j in 0..col {
                if (i.0 % 2 == 0 && j % 2 != 0) || (i.0 % 2 != 0 && j % 2 == 0) {
                    i.1[j] *= -1.0;
                }
            }
        }
        // println!("Cofactors of matrix minor: {:?}", mtrx_minors);

        // Transpose matrix
        mtrx_trans = mtrx_minors.clone();

        for i in mtrx_minors.iter().enumerate().take(row) {
            for j in mtrx_trans.iter_mut().enumerate().take(col) {
                j.1[i.0] = i.1[j.0];
            }
        }
        // println!("Matrix transposed: {:?}", mtrx_trans);

        mtrx_result = mtrx_trans.clone();

        for i in 0..row {
            for j in 0..col {
                mtrx_result[i][j] = mtrx_trans[i][j] / determinant;
            }
        }
        // println!("Inverted matrix: {:?}", mtrx_result);
    }

    Ok(Box::new(mtrx_result))
}

///
/// Use normal equation to calculate theta
///
/// theta = (X.trans * X)^-1 * X.trans * y
///
/// - No need to choose alpha
/// - Don't need to iterate
/// - Slow if number of features is very large (10,000+)
///
pub fn get_theta(x: &[Vec<f64>], y: &[f64]) -> Result<Box<Vec<f64>>, io::Error> {
    let mut theta: Vec<f64> = Vec::new();
    let mut mltply_rslt: Vec<Vec<f64>> = Vec::new();
    let mut mltply_rslt_row: Vec<f64> = Vec::new();

    let mut sum: f64;

    let num_train = if x.len() == y.len() {
        x.len()
    } else {
        return Err(Error::new(ErrorKind::Other, "Matrix not symmetrical"));
    };

    let num_feat = x[0].len();

    /*
        X = [[A, B],
             [C, D]];

        X.transposed = [[A ,C],
                        [B, D]];

        X * X.transposed = [[AA + BB, AC + BD],
                            [CA + DB, CC + DD]]
    */
    for i in 0..num_feat {
        // loop for columns
        mltply_rslt_row.clear();
        for j in 0..num_feat {
            // loop for the rows of X
            sum = 0.0;
            for z in x.iter().take(num_train) {
                // loop for the rows
                sum += z[i] * z[j];
            }
            mltply_rslt_row.push(sum); // results for each row
        }
        mltply_rslt.push(mltply_rslt_row.clone());
    }

    /*
        Calculate inverted X * X.transposed
    */
    let invrt_mtrx = match get_invert(&mltply_rslt) {
        Ok(invrt_mtrx) => invrt_mtrx,
        Err(e) => return Err(e),
    };

    /*
        Calculate y * X.transposed
    */
    let mut y_x_trans: Vec<f64> = Vec::new();

    for j in 0..num_feat {
        sum = 0.0;
        for i in 0..num_train {
            sum += x[i][j] * y[i];
        }
        y_x_trans.push(sum);
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
        for j in y_x_trans.iter().enumerate().take(num_feat) {
            sum += invrt_mtrx[i][j.0] * j.1;
        }
        theta.push(sum);
    }

    Ok(Box::new(theta))
}
