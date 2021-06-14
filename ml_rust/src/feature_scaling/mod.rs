pub fn mean_normal_single(v: &Vec<f32>) -> (Box<Vec<f32>>, f32, f32) {
    let mut max: f32;
    let mut min: f32;
    let mut result: Vec<f32> = Vec::new();
    let mut sum: f32 = 0.0;

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

    for i in 0..v.len() {
        println!("{}", result[i]);
    }

    (Box::new(result), mean, std_dev)
}

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

    for i in 0..row {
        for j in 0..col {
            print!("{}, ", result[i][j]);
        }
        println!("");
    }

    (Box::new(result), Box::new(mean), Box::new(std_dev))
}
