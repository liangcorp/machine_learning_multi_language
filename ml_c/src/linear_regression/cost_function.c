/**
 * @file cost_function.c
 * @author Chen Liang
 * @brief Implementing the cost function and returning the cost.
 * @version 0.1
 * @date 2021-05-04
 *
 * @copyright Copyright (c) 2021
 *
 */

#include "../machine_learning.h"

double cost_function(double** X, double* y, double* theta,
                                int num_train, int num_feat)
{
    #ifdef TIMER
        clock_t cpu_start = clock();    /* Initial processor time */
    #endif
    /*
        Creating the algorithm for the cost function.
        X and y are the training sets.
        theta is a chosen number.
        m is the number of training sets.
        Calculate the cost (J) using the following formula from matlab

        J = sum(((theta' * X')' - y).^2 ./(2 * m), "all");

        m is the length of training set.
        n is the number of features.

     */

    int i, j;

    double J_theta = 0.0L;   /* The cost */
    double sum = 0.0L;
    double *h_x = NULL;



    h_x = calloc(num_train, sizeof(double));

    for (i = 0; i < num_train; i++)
    {
        sum = 0.0L;
        for (j = 0; j < num_feat; j++)
        {
            sum += X[i][j] * theta[j];
        }
        h_x[i] = sum;
    }

    for (i = 0; i < num_train; i++)
    {
        J_theta += (h_x[i] - y[i]) * (h_x[i] - y[i])
                    / (2 * (double)num_train);
    }

    #ifdef TIMER

        clock_t cpu_end = clock();          /* Final cpu time */

        printf("Cost function completed in %lf seconds\n",
                    ((double)(cpu_end - cpu_start)) / CLOCKS_PER_SEC);
    #endif

    free(h_x);

    return J_theta;
}
