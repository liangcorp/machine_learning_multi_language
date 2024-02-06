//! # Implementation of feature normalisation
///
/// Use mean normalization on 1D array.
/// This is used on Y that is a 1D array.
///
#[allow(dead_code)]
pub fn mean_normal_single(v: &Vec<f64>) -> (Box<Vec<f64>>, f64, f64) {
    let mut max: f64;
    let mut min: f64;
    let mut result: Vec<f64> = Vec::new();
    let mut sum: f64 = 0.0;

    /* Set max and min for feature */
    max = v[0];
    min = v[0];

    /* Find max and min for feature */
    for i in v.iter() {
        if max < *i {
            max = *i;
        } else if min > *i {
            min = *i;
        } else {
            // do nothing
        }
        sum += *i;
    }

    let mean = sum / v.len() as f64;

    sum = 0.0;
    for i in v.iter() {
        sum += (*i - mean) * (*i - mean);
    }

    let std_dev = (sum / v.len() as f64).sqrt();

    for i in v.iter() {
        result.push((*i - mean) / std_dev);
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
type DoubleVecF32 = Vec<Vec<f64>>;

pub fn mean_normal_multiple(v: &[Vec<f64>]) -> (Box<DoubleVecF32>, Box<Vec<f64>>, Box<Vec<f64>>) {
    let mut max: Vec<f64> = Vec::new();
    let mut min: Vec<f64> = Vec::new();
    let mut mean: Vec<f64> = Vec::new();
    let mut std_dev: Vec<f64> = Vec::new();

    let mut result: DoubleVecF32 = v.to_vec();

    let row = v.len();
    let col = v[0].len();

    let mut sum: f64;

    for i in v[0].iter() {
        max.push(*i);
        min.push(*i);
    }

    mean.push(1.0);

    for j in 1..col {
        sum = 0.0;
        for i in v.iter().enumerate().take(row) {
            if max[j] < v[i.0][j] {
                max[j] = v[i.0][j];
            } else if min[j] > v[i.0][j] {
                min[j] = v[i.0][j];
            } else {
                // Do nothing
            }
            sum += v[i.0][j];
        }
        mean.push(sum);
    }

    for j in mean.iter_mut().take(col).skip(1) {
        *j /= row as f64;
    }

    std_dev.push(1.0);

    for j in 1..col {
        sum = 0.0;
        for i in v.iter().take(row) {
            sum += (i[j] - mean[j]) * (i[j] - mean[j]);
        }

        std_dev.push((sum / v.len() as f64).sqrt());
    }

    for j in 1..col {
        for i in 0..row {
            result[i][j] = (v[i][j] - mean[j]) / std_dev[j];
        }
    }

    (Box::new(result), Box::new(mean), Box::new(std_dev))
}
