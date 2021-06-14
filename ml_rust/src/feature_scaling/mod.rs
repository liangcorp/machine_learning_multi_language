#[allow(dead_code)]
pub fn zero_mean(v: &Vec<f32>) -> Box<Vec<f32>> {
    let mut max: f32;
    let mut min: f32;
    let mut result: Vec<f32> = Vec::new();

    max = v[0];
    min = v[0];

    for i in v.iter() {
        if max < *i {
            max = *i;
        } else if min > *i {
            min = *i;
        }
        else {
            // do nothing
        }
    }

    for i in v.iter() {
        result.push(
            *i / (max - min)
        );
    }

    Box::new(result)
}

pub fn mean_normalization(v: &Vec<f32>) -> Box<Vec<f32>> {
    let mut max: f32;
    let mut result: Vec<f32> = Vec::new();

    max = v[0];

    for i in v.iter() {
        if max < *i {
            max = *i;
        }
    }

    for i in v.iter() {
        result.push(
            (*i - max / 2.0) / max
        )
    }

    Box::new(result)
}