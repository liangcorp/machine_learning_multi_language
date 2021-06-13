/**
 * @file gradient_descent.c
 * @author Chen Liang
 * @brief Implementation of gradient descent in C
 * @version 0.1
 * @date 2021-05-05
 *
 * @copyright Copyright (c) 2021
 *
 */

#include "gradient_descent.h"

double* gradient_desent(double *x,
                        double *y,
                        double *theta,
                        float alpha,
                        int m,
                        int num_iters)
{
    double sum = 0.0L;
    double *tmp_theta = calloc(2, sizeof(double));

    int i = 0;
    int j = 0;

    for (i = 0; i < num_iters; i++)
    {
        tmp_theta[0] = theta[0];
        tmp_theta[1] = theta[1];

        sum = 0.0L;

        for (j = 0; j < m; j++)
        {
            sum += (tmp_theta[0] * x[j]) - y[j];
        }
        theta[0] = theta[0] - alpha * sum / m;

        sum = 0.0L;

        for (j = 0; j < m; j++)
        {
            sum += ((tmp_theta[1] * x[j]) - y[j]) * x[j];
        }
        theta[1] = theta[1] - alpha * sum / m;
    }

    return theta;
}