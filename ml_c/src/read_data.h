#include <stdio.h>
#include <stdlib.h>
#include <string.h>
#include <malloc.h>

typedef struct data
{
    double **X;
    double *y;
    int num_train;
    int num_feat;
} data_t;

data_t* get_data(char *file_name);