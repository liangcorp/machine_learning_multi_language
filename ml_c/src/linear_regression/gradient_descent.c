/**
 * @file gradient_descent.c
 * @author Chen Liang
 * @brief Implementation of gradient descent in C
 * @version 0.1
 * @date 2021-06-14
 *
 * @copyright Copyright (c) 2021
 *
 */
#include "../machine_learning.h"

double* gradient_desent(double **X, double *y, double *theta,
			                    float alpha, int num_train,
                                int num_feat, int num_iters)
{
    #ifdef TIMER
        clock_t cpu_start = clock();    /* Initial processor time */
    #endif
    int i, j;

    double sum = 0.0L;
    double *tmp_theta = calloc(num_feat, sizeof(double));
    double *h_x = calloc(num_train, sizeof(double));
    double *final_theta = calloc(num_feat, sizeof(double));

    for (j = 0; j < num_feat; j++)
    {
        tmp_theta[j] = theta[j];
    }

    while (num_iters > 0)
    {
        memset(h_x, 0.0L, num_train * sizeof(double));

        for (j = 0; j < num_feat; j++)
        {
            tmp_theta[j] = theta[j];
        }

        for (i = 0; i < num_train; i++)
        {
            for (j = 0; j < num_feat; j++)
            {
                h_x[i] += tmp_theta[j] * X[i][j];
            }
        }

        for (j = 0; j < num_feat; j++)
        {
            sum = 0.0L;

            for (i = 0; i < num_train; i++)
            {
                sum += (h_x[i] - y[i]) * X[i][j];
            }

            theta[j] = tmp_theta[j] - (alpha * sum / (double) num_train);
        }

        num_iters--;
    }

    for (i = 0; i < num_feat; i++)
    {
        final_theta[i] = theta[i];
    }

    free(h_x);
    free(tmp_theta);

    #ifdef TIMER

        clock_t cpu_end = clock();          /* Final cpu time */

        printf("Gradient descent completed in %lf seconds\n",
                    ((double)(cpu_end - cpu_start)) / CLOCKS_PER_SEC);
    #endif
    return final_theta;
}
