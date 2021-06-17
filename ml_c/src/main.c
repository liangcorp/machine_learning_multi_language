/**
 * @file main.c
 * @author Chen Liang
 * @brief main.c used to test the machine learning library
 * @version 0.1
 * @date 2021-05-04
 *
 * @copyright Copyright (c) 2021
 *
 */

#include <stdio.h>
#include <stdlib.h>
#include <string.h>
#include <malloc.h>

#include "machine_learning.h"

int main(int argc, char *argv[])
{

    data_t *data_set = NULL;

    int i, j;

    unsigned int num_train = 0;  // number of training set
    unsigned int num_feat = 0;   // number of features

    double **X = NULL;  // features
    double *y = NULL;   // results

    data_set = get_data(argv[1]);   // Get data set from data file

    X = data_set->X;
    y = data_set->y;
    num_train = data_set->num_train;
    num_feat = data_set->num_feat;

    double *theta = calloc(num_feat, sizeof(double));

    printf("Thetas are [0.0, 0.0]. The cost is %lf\n",
                    cost_function(X, y, theta, num_train, num_feat));

    theta[0] = -1.0;
    theta[1] = 2.0;

    printf("Thetas are [-1.0, 2.0]. The cost is %lf\n",
                    cost_function(X, y, theta, num_train, num_feat));

    float alpha = 0.01;
    int num_iters = 1500;

    theta[0] = 0.0;
    theta[1] = 0.0;

    double *final_theta = gradient_desent(X, y, theta, alpha,
                                            num_train, num_feat,
                                            num_iters);

    printf("Found thetas using Gradient Descent: [");

    for (i = 0; i < num_feat; i++)
    {
        printf("%lf ", final_theta[i]);
    }
    printf("]\n");


    double *final_theta_ne = normal_equation(X, y, num_train, num_feat);
    printf("Found thetas using Normal Equation: [");

    for (i = 0; i < num_feat; i++)
    {
        printf("%lf ", final_theta_ne[i]);
    }
    printf("]\n");

    /*
    normal_single_t* result_y = mean_normal_single(y, num_train);
    free(result_y->v);
    free(result_y);
    */

    for (i = 0; i < num_train; i++)
    {
        free(X[i]); // Free the inner pointers before outer pointers
    }

    free(X);
    free(y);
    free(theta);
    free(final_theta);
    free(final_theta_ne);
    free(data_set);

    #ifdef DEBUG
        printf("Freed all memory\n");
    #endif


    /*
    	A = [[4, -1, 1],
			[4, 5, 3],
			[-2, 0, 0]]

     */

    double **matrix = NULL;

    matrix = calloc(4, sizeof(double));
    for (i = 0; i < 4; i++)
    {
        matrix[i] = calloc(4, sizeof(double));
    }

    matrix[0][0] = 1L;
    matrix[0][1] = 1L;
    matrix[0][2] = 1L;
    matrix[0][3] = -1L;
    matrix[1][0] = 1L;
    matrix[1][1] = 1L;
    matrix[1][2] = -1L;
    matrix[1][3] = 1L;
    matrix[2][0] = 1L;
    matrix[2][1] = -1L;
    matrix[2][2] = 1L;
    matrix[2][3] = 1L;
    matrix[3][0] = -1L;
    matrix[3][1] = 1L;
    matrix[3][2] = 1L;
    matrix[3][3] = 1L;


    printf("Determinant: %lf\n", get_determinant(matrix, 4));

    double **m_inverted = get_invert(matrix, 4);

    for (i = 0; i < 4; i++)
    {
        for (j = 0; j < 4; j++)
        {
            printf("%lf ", m_inverted[i][j]);
        }
        printf("\n");
    }

    for (i = 0; i < 4; i++)
        free(m_inverted[i]);
    free(m_inverted);

    for (i = 0; i < 4; i++)
        free(matrix[i]);
    free(matrix);

    return 0;
}