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
#include "cost_function.h"

double* gradient_desent(double *x,
                        double *y,
                        double *theta,
                        float alpha,
                        int m)
{
    double j_theta = 0.0L;
    double tmp_j_theta = 0.0L;
    double sum = 0.0L;
    double *tmp_theta = calloc(2, sizeof(double));

    int i = 0;
    int j = 0;


    j_theta = cost_function(x, y, theta, m);

    while(1)
    {
        tmp_theta[0] = theta[0];
        tmp_theta[1] = theta[1];

        sum = 0.0L;

        for (i = 0; i < m; i++)
        {
            sum += (tmp_theta[0] * x[i]) - y[i];
        }
        theta[0] = theta[0] - alpha * sum / m;

        sum = 0.0L;

        for (i = 0; i < m; i++)
        {
            sum += ((tmp_theta[1] * x[i]) - y[i]) * x[i];
        }
        theta[1] = theta[1] - alpha * sum / m;

        tmp_j_theta = cost_function(x, y, theta, m);

        if (j_theta > tmp_j_theta)
        {
            j_theta = tmp_j_theta;
        }
        else
        {
            break;
        }
    }

    return theta;
}