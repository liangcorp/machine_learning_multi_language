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

#include <stdio.h>
#include <math.h>
#include <time.h>
#include <ctype.h>
#include <malloc.h>

#include "cost_function.h"

double cost_function(double* X, double* y, double theta, int m)
{
    /*
        Creating the algorithm for the cost function.
        X and y are the training sets.
        theta is a chosen number.
        m is the number of training sets.
        Calculate the cost (J) using the following formula

        J (theta[0], theta[1]) = 1/2m (sum( (h(x[i]) - y[i]) ^ 2) )

     */
    int i = 0;

    double J_theta = 0.0;   /* The cost */

    #ifdef DEBUG
        clock_t cpu_start = clock();    /* Initial processor time */
    #endif

    for (i = 0; i < m; i++)
    {
        /*
            It turned out by using a cleaner code formate, the program
            executes longer. So copying variable is expensive.

            n = theta * X[i]) - y[i];
            J_theta += (n * n) / (2 * m);

        */
        J_theta += ((theta * X[i]) - y[i]) * ((theta * X[i]) - y[i])
                                                            / (2 * m);
    }

    #ifdef DEBUG
        printf("The final cost is %lf\n", J_theta);


        clock_t cpu_end = clock();          /* Final cpu time */

        printf("CPU time is %lf seconds\n",
                    ((double)(cpu_end - cpu_start)) / CLOCKS_PER_SEC);
    #endif

    return J_theta;
}

double* cost_function_multiple(double* X, double* y, double* theta,
                                                int m, int th_count)
{
    /*
        Creating the algorithm for the cost function.
        X and y are the training sets.
        theta is a chosen number.
        m is the number of training sets.
        Calculate the cost (J) using the following formula from matlab

        J = sum(((theta' * X')' - y).^2 ./(2 * m), "all");

     */
    int i = 0;
    int j = 0;

    double *J_theta = NULL;   /* The cost */

    #ifdef DEBUG
        clock_t cpu_start = clock();    /* Initial processor time */
    #endif

    J_theta = calloc(th_count, sizeof(double));

    for (i = 0; i < th_count; i++)
    {
        for (j = 0; j < m; j++)
        {
             J_theta[i] += ((theta[i] * X[j]) - y[j])
                            * ((theta[i] * X[j]) - y[j]) / (2 * m);
        }
    }

    #ifdef DEBUG
        printf("The final cost is %lf\n", J_theta);


        clock_t cpu_end = clock();          /* Final cpu time */

        printf("CPU time is %lf seconds\n",
                    ((double)(cpu_end - cpu_start)) / CLOCKS_PER_SEC);
    #endif

    return J_theta;
}