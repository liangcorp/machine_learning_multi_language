/**
 * @file feature_scale.h
 * @author Chen Liang
 * @brief Implementation of gradient descent in C
 * @version 0.1
 * @date 2021-06-14
 *
 * @copyright Copyright (c) 2021
 *
 */

#include "machine_learning.h"

normal_multi_t* mean_normal_multiple(double **v,
                                    int num_train,
                                    int num_feat)
{

}

normal_single_t* mean_normal_single(double *v, int num_train)
{
    int i, j;

    double max, min, mean, std_dev;

    double sum = 0.0L;
    normal_single_t *result = NULL;
    double *result_v = NULL;

    max = v[0];
    min = v[1];

    for (i = 0; i < num_train; i++)
    {
        if (max < v[i])
            max = v[i];
        else if (min > v[i])
            min = v[i];

        sum += v[i];
    }

    mean = sum / num_train;

    sum = 0.0L;

    for (i = 0; i < num_train; i++)
    {
        sum += (v[i] - mean) * (v[i] - mean);
    }

    std_dev = sqrt(sum / num_train);

    result_v = calloc(num_train, sizeof(double));

    for (i = 0; i < num_train; i++)
    {
        result_v[i] = (v[i] - mean) / std_dev;
    }

    result = calloc(1, sizeof(normal_single_t));

    result->v = result_v;
    result->mean = mean;
    result->std_dev = std_dev;

    return result;
}