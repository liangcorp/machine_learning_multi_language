/**
 * @file machine_learning.h
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
#include <math.h>
#include <time.h>
#include <ctype.h>
#include <malloc.h>

typedef struct data
{
    double **X;
    double *y;
    int num_train;
    int num_feat;
} data_t;

data_t* get_data(char *file_name);

double cost_function(double** X, double* y, double* theta,
                                int no_train, int no_feat);

double* gradient_desent(double **X, double *y, double *theta,
	float alpha, int num_train, int num_feat, int num_iters);