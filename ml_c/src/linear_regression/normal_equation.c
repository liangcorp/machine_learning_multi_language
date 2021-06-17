/**
 * @file normal_equation.c
 * @author Chen Liang (chen.liang.mail@protonmail.com)
 * @brief Implementation of normal equation in C
 * @version 0.1
 * @date 2021-06-17
 *
 * @copyright Copyright (c) 2021
 *
 */
#include "../machine_learning.h"

/*
	Calculate the Determinant (der) of a matrix
	Calculate the Determinant (der) of larger scale matrix.
	Tested for 2x2, 3x3 and 4x4 matrixes.
	Implementation is not optimized for super large matrixes.
 */
double get_determinant(double **matrix, unsigned int size)
{
	unsigned int z, i, j, m, n;
	unsigned int size_minor = size - 1;

	double determinant = 0.0L;
	double multiply =0.0L;

	if (size == 2)	// 2x2 matrixes
	{
		/*
			Calculate the Determinant (der) of 2D matrix
			M = [[A, B],
				 [C, D]]
			Determinant = A * D - B * C
		*/
		determinant = matrix[0][0] * matrix[1][1]
						- matrix[0][1] * matrix[1][0];
	}
	else if (size == 3)	// 3x3 matrixes
	{
		/*
			Calculate the Determinant (der) of larger scale matrix
			(shortcut method)
			M = [[A, B, C]
				[D, E, F]
				[G, H, I]]

			der = A * E * I + B * F * G + C * D * H
					- C * E * G - A * F * H - B * D * I;
		*/
		for (i = 0; i < size; i++)
		{
			z = i;	// starting column (increase each loop)
			multiply = 1.0L;
			for (j = 0; j < size; j++)
			{
				if (z >= size)
					z = 0;	// Reset to column 0

				multiply *= matrix[j][z];
				z += 1;
			}
			// A * E * I + B * F * G + C * D * H
			determinant += multiply;
		}

		for (i = 0; i < size; i++)
		{
			z = size - 1 - i;
			multiply = 1.0L;

			// starting column (decrease each loop)
			for (j = 0; j < size; j++)
			{
				if (z == -1)
					z = size - 1;	// Reset to column 0

				multiply *= matrix[j][z];
				z -= 1;		// Move to last column
			}
			// C * E * G - A * F * H - B * D * I
			determinant -= multiply;
		}
	}
	else // 4x4 and above matrixes
	{
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

		double **m_cofactor = NULL;
		double *first_col = NULL;
		double *deter_list = NULL;

		m_cofactor = calloc((size_minor), sizeof(double));

		for (i = 0; i < size_minor; i++)
			m_cofactor[i] = calloc(size_minor, sizeof(double));

		first_col = calloc(size, sizeof(double));
		deter_list = calloc(size, sizeof(double));

		/*
			Create cofactor matrix for first element
			of each row.
		*/
		int new_i, new_j;
		j = 0;

		for (i = 0; i < size; i++)
		{
			first_col[i] = matrix[i][0];
			/*
				Create cofactor matrix for first element
				of each row.
			*/
			new_i = 0;
			for (m = 0; m < size; m++)
			{
				new_j = 0;
				for (n = 0; n < size; n++)
				{
					if (m != i && n != j)
					{
						m_cofactor[new_i][new_j] = matrix[m][n];
						new_j++;
					}
				}
				if (m != i && n != j)
					new_i++;
			}
			deter_list[i] = get_determinant(m_cofactor, size_minor);
		}	// end of i

		for (i = 0; i < size; i++)
			if (i % 2 == 0)
				determinant += first_col[i] * deter_list[i];
			else
				determinant -= first_col[i] * deter_list[i];

		for (i = 0; i < size_minor; i++)
			free(m_cofactor[i]);

		free(m_cofactor);
		free(first_col);
		free(deter_list);
	}

	return determinant;
}

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

double** get_invert(double **matrix, unsigned int size)
{
	#ifdef TIMER
        clock_t cpu_start = clock();    /* Initial processor time */
    #endif
	unsigned int i, j, m, n;
	unsigned int size_minor = size - 1;
	double **m_deter = NULL;
	double **m_minors = NULL;
	double **m_trans = NULL;
	double **m_invert = NULL;
	double determinant = get_determinant(matrix, size);

	m_invert = calloc(size, sizeof(double));
	for (i = 0; i < size; i++)
		m_invert[i] = calloc(size, sizeof(double));

	if (size == 2)
	{
		m_invert[0][0] = matrix[1][1] / determinant;
		m_invert[1][1] = matrix[0][0] / determinant;
		m_invert[0][1] = -(matrix[0][1] / determinant);
		m_invert[1][0] = -(matrix[1][0] / determinant);
	}
	else
	{
		// Calculate matrix of minors
		m_deter = calloc(size_minor, sizeof(double));
		for (i = 0; i < size_minor; i++)
			m_deter[i] = calloc(size_minor, sizeof(double));

		m_minors = calloc(size, sizeof(double));
		for (i = 0; i < size; i++)
			m_minors[i] = calloc(size, sizeof(double));

		int new_i = 0;
		int new_j = 0;

		for (i = 0; i < size; i++)
		{
			for (j = 0; j < size; j++)
			{
				new_i = 0;
				for (m = 0; m < size; m++)
				{
					new_j = 0;
					for (n = 0; n < size; n++)
					{
						if (m != i && n != j)
						{
							m_deter[new_i][new_j] = matrix[m][n];
							new_j++;
						}
					}

					if (m != i && n != j)
						new_i++;
				}	// end of m

				m_minors[i][j] = get_determinant(m_deter, size_minor);
			}	// end of j
		}	// end of i

		// Matrix of Cofactors
		for (i = 0; i < size; i++)
			for (j = 0; j < size; j++)
				if (i % 2 == 0 && j % 2 != 0)
					m_minors[i][j] *= -1;
				else if (i % 2 != 0 && j % 2 == 0)
					m_minors[i][j] *= -1;

		// Transpose matrix
		m_trans = calloc(size, sizeof(double));
		for (i = 0; i < size; i++)
			m_trans[i] = calloc(size, sizeof(double));

		for (i = 0; i < size; i++)
			for (j = 0; j < size; j++)
				m_trans[j][i] = m_minors[i][j];

		for (i = 0; i < size; i++)
			for (j = 0; j < size; j++)
				m_invert[i][j] = m_trans[i][j] / determinant;

		for (i = 0; i < size; i++)
			free(m_trans[i]);
		free(m_trans);

		for (i = 0; i < size ; i++)
			free(m_minors[i]);
		free(m_minors);

		for (i = 0; i < size_minor; i++)
			free(m_deter[i]);

		free(m_deter);
	}
	#ifdef TIMER

        clock_t cpu_end = clock();          /* Final cpu time */

        printf("Matrix inversion completed in %lf seconds\n",
                    ((double)(cpu_end - cpu_start)) / CLOCKS_PER_SEC);
    #endif
	return m_invert;
}


/*
	Use normal equation to calculate theta

	theta = (X.trans * X)^-1 * X.trans * y

	- No need to choose alpha
	- Don't need to iterate
	 - Slow if number of features is very large (10,000+)
 */

double* normal_equation(double **X, double *y,
				unsigned int num_train, unsigned int num_feat)
{
	#ifdef TIMER
        clock_t cpu_start = clock();    /* Initial processor time */
    #endif
	unsigned int i, j, z;
	double *theta = NULL;
	double **m_X_X_trans = NULL;
	double **m_invert = NULL;
	double *y_x_trans = NULL;

	double sum = 0.0L;

	/*
		X = [[A, B],
			 [C, D]];

		X.transposed = [[A ,C],
						[B, D]];

		X * X.transposed = [[AA + BB, AC + BD],
							[CA + DB, CC + DD]]
	*/
	m_X_X_trans = calloc(num_feat, sizeof(double));
	for (i = 0; i < num_feat; i++)
		m_X_X_trans[i] = calloc(num_feat, sizeof(double));

	for (i = 0; i < num_feat; i++)
	{
		for (j = 0; j < num_feat; j++)
		{
			sum = 0.0L;
			for (z = 0; z < num_train; z++)
				sum += X[z][i] * X[z][j];

			m_X_X_trans[i][j] = sum;
		}
	}

	/*
		Calculate inverted X * X.transposed
	*/
	m_invert = get_invert(m_X_X_trans, num_feat);

	/*
		Calculate y * X.transposed
	*/
	y_x_trans = calloc(num_feat, sizeof(double));

	for (j = 0; j < num_feat; j++)
	{
		sum = 0.0L;
		for (i = 0; i < num_train; i++)
			sum += X[i][j] * y[i];

		y_x_trans[j] = sum;
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
	theta = calloc(num_feat, sizeof(double));
	for (i = 0; i < num_feat; i++)
	{
		sum = 0.0L;
		for (j = 0; j < num_feat; j++)
			sum += m_invert[i][j] * y_x_trans[j];

		theta[i] = sum;
	}

	for (i = 0; i < num_feat; i++)
		free(m_invert[i]);

	for (i = 0; i < num_feat; i++)
		free(m_X_X_trans[i]);

	free(m_invert);
	free(m_X_X_trans);
	free(y_x_trans);

	#ifdef TIMER

        clock_t cpu_end = clock();          /* Final cpu time */

        printf("Normal equation completed in %lf seconds\n",
                    ((double)(cpu_end - cpu_start)) / CLOCKS_PER_SEC);
    #endif
	return theta;
}