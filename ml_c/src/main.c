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
#include "gradient_descent.h"

int main(int argc, char *argv[])
{
    FILE *fp = NULL;

    char str[200];

    double **X = NULL;  // features
    double *y = NULL;   // results

    int m = 0;  // number of training set
    int n = 0;  // number of features

    int i = 0;
    int j = 0;

    /* opening file for reading */
    fp = fopen(argv[1], "r");

    if (fp == NULL)
    {
        perror("Error opening file");
        return -1;
    }

    while(!feof(fp))
    {
        // Find number of training set
        fgets(str, 200, fp);
        m++;
    }

    char *token = strtok(str, ",");
    while (token != NULL)
    {
        // Find number of features
        token = strtok(NULL, ",");
        n++;
    }

    printf("Number of training sets: %d\n", m);
    printf("Number of features: %d\n", n);

    rewind(fp);

    #ifdef DEBUG
        printf("Allocated memory for token\n");
    #endif

    X = calloc(m, sizeof(double));

    for (i = 0; i < m; i++)
    {
        X[i] = calloc(n, sizeof(double));
    }

    for (i = 0; i < m; i++)
    {
        X[i][0] = 1.0L; // Initialised the first features into 1
    }

    y = calloc(m, sizeof(double));

    i = 0;
    while(!feof(fp))
    {
        fgets(str, 200, fp);    // Read line

        for (j = 1; j < n; j++)
        {
            // Read all but the last colum into X
            // Convert the string to double
            X[i][j] = strtod(strtok(str, ","), NULL);
        }

        // Read the last colum into y
        // Convert the string to double
        y[i] = strtod(strtok(NULL, ","), NULL);

        i++;    // Move to the next line
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

    double *theta = calloc(n, sizeof(double));

    printf("Thetas are [0.0, 0.0], J(theta) is %lf\n",
                            cost_function(X, y, theta, m, n));

    theta[0] = -1.0;
    theta[1] = 2.0;

    printf("Thetas are [-1.0, 2.0], J(theta) is %lf\n",
                            cost_function(X, y, theta, m, n));
/*
    theta[0] = 0.0;
    theta[1] = 0.0;
    float alpha = 0.01;

    double *final_theta = gradient_desent(X, y, theta, alpha, m, 1500);
    printf("Found thetas using Gradient Descent: [%lf, %lf]\n",
                                    final_theta[0], final_theta[1]);

*/

    for (i = 0; i < m; i++)
    {
        free(X[i]); // Free the inner pointers before outer pointers
    }
    free(X);

    free(y);
    free(theta);
    #ifdef DEBUG
        printf("Freed all memory\n");
    #endif

    return 0;
}