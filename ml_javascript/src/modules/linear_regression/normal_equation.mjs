/**
 * Name: normal_equation.mjs
 * Author: Chen Liang
 * Description: My implementation of Machine Learning formulas
 * 				in JavaScript. This is an exercise, so you may
 * 				want to use a professional library for production.
 * Date: 2021-06-15
 */

/*
    Calculate the Determinant (der) of a matrix

    Calculate the Determinant (der) of larger scale matrix.
    Tested for 2x2, 3x3 and 4x4 matrixes.
    Implementation is not optimized for super large matrixes.
 */
const get_determinant = function(matrix) {
    let i, j, z, multiply;

    let determinant = 0.0;
    let size = matrix.length;
    let size_minor = size - 1;

    if (matrix.length !== matrix[0].length) {
        console.log(matrix.length + "x" + matrix[0].length);
        process.exit(1);
    }

    if (size === 2) {
        /*
                Calculate the Determinant (der) of 2D matrix
                M = [[A, B],
                     [C, D]]
                Determinant = A * D - B * C
            */
        determinant = matrix[0][0] * matrix[1][1] - matrix[0][1] * matrix[1][0];
    } else if (size === 3) {
        /*
                Calculate the Determinant (der) of larger scale matrix
                (shortcut method)
                M = [[A, B, C]
                     [D, E, F]
                     [G, H, I]]

                der = A * E * I + B * F * G + C * D * H
                        - C * E * G - A * F * H - B * D * I;
            */

        for (i in matrix) {
            z = i;
            multiply = 1.0;
            for (j in matrix) {
                if (z >= size) {
                    z = 0; // reset column to 0
                }

                multiply *= matrix[j][z];
                z++;
            }
            // A * E * I + B * F * G + C * D * H
            determinant += multiply;
        }

        for (i in matrix) {
            z = size - 1 - i;
            multiply = 1.0;

            for (j in matrix) {
                if (z < 0) {
                    z = size - 1;
                }

                multiply *= matrix[j][z];
                z--;
            }
            // C * E * G - A * F * H - B * D * I
            determinant -= multiply;
        }
    } else {
        // 4x4 and above matrixes
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
        let m_cofactor = [];
        let m_cofactor_row = [];
        let first_col_element = [];
        let deter_list = [];
        let m, n;

        j = 0;
        for (i in matrix) {
            // Save first element of each row
            first_col_element.push(matrix[i][j]);

            /*
                      Create cofactor matrix for first element
                      of each row.
                  */
            for (m in matrix) {
                for (n in matrix) {
                    if (m != i && n != j) {
                        m_cofactor_row.push(matrix[m][n]);
                    }
                }

                if (m_cofactor_row.length != 0) {
                    m_cofactor.push(m_cofactor_row);
                    m_cofactor_row = [];
                }
            }
            deter_list[i] = get_determinant(m_cofactor);
            m_cofactor = [];
        } // end of i

        for (i in matrix) {
            if (i % 2 == 0) {
                determinant += first_col_element[i] * deter_list[i];
            } else {
                determinant -= first_col_element[i] * deter_list[i];
            }
        }
    } // end of else

    return determinant;
};

/*
    Calculate inverted matrix from provided matrix
    Currently using adjugate matrix

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
const get_invert = function(matrix) {
    let i, j, m, n;
    let m_result = [];
    let determinant = get_determinant(matrix);
    let size = matrix.length;

    if (size == 2) {
        m_result = matrix;
        m_result[0][0] = matrix[1][0] / determinant;
        m_result[1][1] = matrix[0][0] / determinant;
        m_result[0][1] = -matrix[0][1] / determinant;
        m_result[1][0] = -matrix[1][0] / determinant;
    } else {
        let m_der = [];
        let m_der_row = [];
        let m_minors = [];
        let m_minors_row = [];

        let m_trans = [];
        let m_trans_row = [];

        // Calculate matrix of minors
        for (i in matrix) {
            for (j in matrix) {
                for (m in matrix) {
                    for (n in matrix) {
                        if (m != i && n != j) {
                            m_der_row.push(matrix[m][n]);
                        }
                    }
                    if (m_der_row.length != 0) {
                        m_der.push(m_der_row);
                        m_der_row = [];
                    }
                }

                m_minors_row.push(get_determinant(m_der));
                m_der = [];
            }
            m_minors.push(m_minors_row);
            m_minors_row = [];
        }

        // Matrix of Cofactors
        for (i in matrix) {
            for (j in matrix) {
                if (i % 2 == 0 && j % 2 != 0) m_minors[i][j] *= -1;
                else if (i % 2 != 0 && j % 2 == 0) m_minors[i][j] *= -1;
            }
        }

        // Transpose matrix
        for (i in m_minors) {
            for (j in m_minors) {
                m_trans_row.push(m_minors[j][i]);
            }
            m_trans.push(m_trans_row);
            m_trans_row = [];
        }

        // Calculate inverted matrix
        m_result = m_trans;
        for (i in m_trans) {
            for (j in m_trans) {
                m_result[i][j] = m_trans[i][j] / determinant;
            }
        }
    }

    return m_result;
};

/*
    Use normal equation to calculate theta

    theta = (X.trans * X)^-1 * X.trans * y

    - No need to choose alpha
    - Don't need to iterate
    - Slow if number of features is very large (10,000+)
 */
const normal_equation = function(X, y) {
    let i, j, z, sum;
    let theta = [];
    let m_result = [];
    let m_result_row = [];
    let m_inverted = [];
    let y_x_trans = [];

    let num_feat = X[0].length;
    let num_train = y.length;

    if (X.length != y.length) {
        console.log("Mis-matching training set");
        process.exit(-1);
    }

    /*
          X = [[A, B],
               [C, D]];

          X.transposed = [[A ,C],
                          [B, D]];

          X * X.transposed = [[AA + BB, AC + BD],
                              [CA + DB, CC + DD]]
      */
    for (i = 0; i < num_feat; i++) {
        for (j = 0; j < num_feat; j++) {
            sum = 0;
            for (z = 0; z < num_train; z++) {
                sum += X[z][i] * X[z][j];
            }
            m_result_row.push(sum);
        }
        m_result.push(m_result_row);
        m_result_row = [];
    }

    /*
          Calculate inverted X * X.transposed
      */
    m_inverted = get_invert(m_result);

    /*
          Calculate y * X.transposed
      */

    for (j = 0; j < num_feat; j++) {
        sum = 0;
        for (i = 0; i < num_train; i++) {
            sum += X[i][j] * y[i];
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
    for (i = 0; i < num_feat; i++) {
        sum = 0;
        for (j = 0; j < num_feat; j++) {
            sum += m_inverted[i][j] * y_x_trans[j];
        }
        theta.push(sum);
    }

    return theta;
};

export { get_invert, get_determinant, normal_equation };
