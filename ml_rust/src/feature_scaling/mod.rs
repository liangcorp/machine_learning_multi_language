//! # Implementation of feature normalization
///
/// Use mean normalization on 1D array.
/// This is used on Y that is a 1D array.
///
#[allow(dead_code)]
pub fn mean_normal_single(v: &Vec<f32>) -> (Box<Vec<f32>>, f32, f32) {
    let mut max: f32;
    let mut min: f32;
    let mut result: Vec<f32> = Vec::new();
    let mut sum: f32 = 0.0;

    /* Set max and min for feature */
    max = v[0];
    min = v[0];

    /* Find max and min for feature */
    for i in v.iter() {
        if max < *i {
            max = *i;
        } else if min > *i {
            min = *i;
        }
        else {
            // do nothing
        }
        sum += *i;
    }

    let mean = sum / v.len() as f32;

    sum = 0.0;
    for i in v.iter() {
        sum += (*i - mean) * (*i - mean);
    }

    let std_dev = (sum / v.len() as f32).sqrt();

    for i in v.iter() {
        result.push(
            (*i - mean)/ std_dev
        );
    }

    (Box::new(result), mean, std_dev)
}

/// # Use mean normalization on 2D array.
/// This is used on X that usually contains multiple features.
/// The function returns a pointer to a structure.
/// The structure contains pointers to the following:
///     - Pointer to the result 2D array
///     - Pointer to the list of mean
///     - Pointer to the list of standard deviation
///
/// NOTE: Run free on pointers of the structure
/// and its elements in main.
///
/// NOTE: The values of first feature (i.e. x\[0\] is 1.0)
///
#[allow(dead_code)]
pub fn mean_normal_multiple(v: &Vec<Vec<f32>>)
                                -> (Box<Vec<Vec<f32>>>, Box<Vec<f32>>, Box<Vec<f32>>) {
    let mut max: Vec<f32> = Vec::new();
    let mut min: Vec<f32> = Vec::new();
    let mut mean: Vec<f32> = Vec::new();
    let mut std_dev: Vec<f32> = Vec::new();

    let mut result: Vec<Vec<f32>> = v.clone();

    let row = v.len();
    let col = v[0].len();

    let mut sum: f32;


    for i in v[0].iter(){
        max.push(*i);
        min.push(*i);
    }

    mean.push(1.0);

    for j in 1..col {
        sum = 0.0;
        for i in 0..row {
            if max[j] < v[i][j] {
                max[j] = v[i][j];
            } else if min[j] > v[i][j] {
                min[j] = v[i][j];
            } else {
                // Do nothing
            }
            sum += v[i][j];
        }
        mean.push(sum);
    }

    for j in 1..col {
        mean[j] = mean[j] / row as f32;
    }

    std_dev.push(1.0);

    for j in 1..col {
        sum = 0.0;
        for i in 0..row {
            sum += (v[i][j] - mean[j]) * (v[i][j] - mean[j]);
        }

        std_dev.push((sum / v.len() as f32).sqrt());
    }


    for j in 1..col {
        for i in 0..row {
            result[i][j] = (v[i][j] - mean[j]) / std_dev[j];
        }
    }

    (Box::new(result), Box::new(mean), Box::new(std_dev))
}
