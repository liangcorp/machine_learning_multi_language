/**
 * @file machine_learning.h
 * @author Chen Liang
 * @brief Implementation of gradient descent in C
 * @version 0.1
 * @date 2021-06-14
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

typedef struct
{
    double **X;
    double *y;
    int num_train;
    int num_feat;
} data_t;

typedef struct
{
    double **V;
    double *mean;
    double *std_dev;
} normal_multi_t;

typedef struct
{
    double * v;
    double mean;
    double std_dev;
} normal_single_t;

data_t* get_data(char *file_name);

double cost_function(double** X, double* y, double* theta,
                                int no_train, int no_feat);

double* gradient_desent(double **X, double *y, double *theta,
	float alpha, int num_train, int num_feat, int num_iters);

normal_multi_t* mean_normal_multiple(double **v,
                                    int num_train,
                                    int num_feat);

normal_single_t* mean_normal_single(double *v, int num_train);