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
#include <string.h>

double* gradient_desent_multi(double **X, double *y, double *theta,
	float alpha, int num_train, int num_feat, int num_iters);