/**
 * @file gradient_descent.h
 * @author Chen Liang
 * @brief Implementation of gradient descent in C
 * @version 0.1
 * @date 2021-05-05
 *
 * @copyright Copyright (c) 2021
 *
 */
#include <stdio.h>
#include <stdlib.h>

double* gradient_desent(double *x,
			double *y,
			double *theta,
			float alpha,
			int m);