/**
 * @file read_from_data_file.c
 * @author Chen Liang
 * @brief Read data from file and store it in array
 * @version 0.1
 * @date 2021-06-15
 *
 * @copyright Copyright (c) 2021
 *
 */

#include <stdio.h>
#include <stdlib.h>
#include <string.h>
#include <malloc.h>

typedef struct {
	double **X;
	double *y;
	int num_train;
	int num_feat;
} data_t;

// Function name is the same of the source code file name.
// This is for convenient purpose.
data_t *read_from_data_file(char *file_name)
{
	data_t *data_set = NULL;

	FILE *fp = NULL;

	char str[200];

	double **X = NULL; // features
	double *y = NULL; // results

	int m = 0; // number of training set
	int n = 0; // number of features

	int i = 0;
	int j = 0;

	/* opening file for reading */
	fp = fopen(file_name, "r");

	if (fp == NULL) {
		perror("Error opening file");
		exit(EXIT_FAILURE);
	}

	while (!feof(fp)) {
		// Find number of training set
		fgets(str, 200, fp);
		m++;
	}

	char *token = strtok(str, ",");
	while (token != NULL) {
		// Find number of features
		token = strtok(NULL, ",");
		n++;
	}

	printf("Number of training sets: %d\n", m);
	printf("Number of features: %d\n", n);

	rewind(fp);

#ifdef DEBUG
	printf("Allocate memory for training set\n");
#endif

	X = calloc(m, sizeof(double));

#ifdef DEBUG
	printf("Allocate memory for features\n");
#endif

	for (i = 0; i < m; i++) {
		X[i] = calloc(n, sizeof(double));
	}

#ifdef DEBUG
	printf("Initialize the first features into 1.0\n");
#endif

	for (i = 0; i < m; i++) {
		// Initialize the first features into 1.0
		X[i][0] = 1.0L;
	}

#ifdef DEBUG
	printf("Allocate memory for test set\n");
#endif
	y = calloc(m, sizeof(double));

	i = 0;
#ifdef DEBUG
	printf("Read all but the last column into X");
	printf("Read the last column into y\n");
#endif
	while (!feof(fp)) {
		fgets(str, 200, fp); // Read line

		X[i][1] = strtod(strtok(str, ","), NULL);

#ifdef DEBUG
		printf("\t%lf  |  %lf  |", X[i][0], X[i][1]);
#endif
		for (j = 2; j < n; j++) {
			// Read all but the last column into X
			// Convert the string to double
			X[i][j] = strtod(strtok(NULL, ","), NULL);
#ifdef DEBUG
			printf("  %lf  |", X[i][j]);
#endif
		}

		// Read the last column into y
		// Convert the string to double
		y[i] = strtod(strtok(NULL, ","), NULL);

#ifdef DEBUG
		printf("  %lf\n", y[i]);
#endif
		i++; // Move to the next line
	}

	fclose(fp);
	fp = NULL;

	data_set = calloc(1, sizeof(data_t));
	data_set->X = X;
	data_set->y = y;
	data_set->num_train = m;
	data_set->num_feat = n;

#ifdef DEBUG
	printf("Closed file\n");
#endif

	return data_set;
}
