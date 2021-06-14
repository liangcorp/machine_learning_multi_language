/**
 * @file cost_function.h
 * @author Chen Liang
 * @brief Header file for the cost function
 * @version 0.1
 * @date 2021-05-04
 *
 * @copyright Copyright (c) 2021
 *
 */

#include <stdio.h>
#include <stdlib.h>
#include <math.h>
#include <time.h>
#include <ctype.h>
#include <malloc.h>

double cost_function(double** X, double* y, double* theta,
                                int no_train, int no_feat);