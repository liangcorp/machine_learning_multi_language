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

#include "cost_function.h"

int main(int argc, char *argv[])
{
    FILE *fp = NULL;

    char str[200];

    double *X = NULL;
    double *y = NULL;

    int m = 0;
    int i = 0;

    /* opening file for reading */
    fp = fopen(argv[1], "r");

    if (fp == NULL)
    {
        perror("Error opening file");
        return -1;
    }

    while(!feof(fp))
    {
        fgets(str, 200, fp);
        printf("%s", str);
        m++;
    }

    rewind(fp);

    X = calloc(m, sizeof(double));
    y = calloc(m, sizeof(double));

    #ifdef DEBUG
        printf("Allocated memory for token\n");
    #endif

    while(!feof(fp))
    {
        fscanf(fp, "%lf,%lf", &X[i], &y[i]);
        i++;
    }

    #ifdef DEBUG
        for (i = 0; i < m; i++)
        {
            printf("\t%lf\t |\t%lf\n", X[i], y[i]);
        }
    #endif

    fclose(fp);
    fp = NULL;

    #ifdef DEBUG
        printf("Closed file\n");
    #endif

    double *theta = calloc(2, sizeof(double));

    printf("\nThe cost is %lf\n", cost_function(X, y, theta, m));

    theta[0] = -1.0;
    theta[1] = 2.0;

    printf("\nThe cost is %lf\n", cost_function(X, y, theta, m));
    /*
    printf("\n\n The J theta of theta: 1, 1.1, 1.2, 1.3, 1.4 are:\n");

    double thetas[] = {1, 1.1, 1.2, 1.3, 1.4};

    double *J = cost_function_multiple(X, y, thetas, m,
                            (int)sizeof(thetas) / sizeof(thetas[0]));

    #ifdef DEBUG
        printf("Number of thetas: %d\n",
                        (int)sizeof(thetas) / sizeof(thetas[0]));
    #endif

    for (i = 0; i < 5; i++)
    {
        printf("%lf\n", J[i]);
    }
    */

    free(X);
    free(y);
    // free(J);
    #ifdef DEBUG
        printf("Freed all memory\n");
    #endif

    return 0;
}