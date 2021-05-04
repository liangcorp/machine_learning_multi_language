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
#include "cost_function.h"

int cost_function(double* X, double* y, float theta, int m)
{
    /*
        Creating the algrithem for the cost function.
        X and y are the training sets.
        theta is a chosen number.
        m is the number of training sets.
        Calculate the cost (J) using the following formula

        J (theta[0], theta[1]) = 1/2m (sum( (h(x[i]) - y[i]) ^ 2) )

     */

    double J_theta = 0.0;     /* The cost */
    int i = 0;

    for (i = 0; i < m; i++)
    {
        J_theta += pow((theta * X[i]) - y[i], 2) / (2 * m);
    }

    return J_theta;
}
