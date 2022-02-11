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
	printf("Allocated memory for token\n");
#endif

	X = calloc(m, sizeof(double));

	for (i = 0; i < m; i++) {
		X[i] = calloc(n, sizeof(double));
	}

	for (i = 0; i < m; i++) {
		X[i][0] = 1.0L; // Initialized the first features into 1
	}

	y = calloc(m, sizeof(double));

	i = 0;
	while (!feof(fp)) {
		fgets(str, 200, fp); // Read line

		for (j = 1; j < n; j++) {
			// Read all but the last column into X
			// Convert the string to double
			X[i][j] = strtod(strtok(str, ","), NULL);
		}

		// Read the last column into y
		// Convert the string to double
		y[i] = strtod(strtok(NULL, ","), NULL);

		i++; // Move to the next line
	}

#ifdef DEBUG
	for (i = 0; i < m; i++) {
		printf("\t%lf\t |\t%lf\n", X[i], y[i]);
	}
#endif

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
