use std::path::Path;

mod feature_scaling;
mod read_data;

use ml_rust::linear_regression;

fn main() {
    let path = Path::new("../data_files/ex1data1.txt");

    let (x_ptr, y_ptr) = read_data::get_data(&path);

    let x = *x_ptr;
    let y = *y_ptr;

    let alpha = 0.01;   // the learning speed

    let mut theta = vec![0.0, 0.0]; // set theta 0 and theta 1 to 0.0

    println!("Thetas are [0.0, 0.0], J(theta) is {}",
    linear_regression::cost_functions::get_cost(&x, &y, &theta));

    println!("Thetas are [-1.0, 2.0], J(theta) is {}",
    linear_regression::cost_functions::get_cost(&x, &y, &vec![-1.0, 2.0]));

    println!("Found thetas using Gradient Descent: {:?}",
    linear_regression::gradient_descent::get_thetas(&x, &y,
                                        alpha, &mut theta, 1500));

    println!("Found thetas using Normal Equation: {:?}",
                linear_regression::normal_equation::get_theta(&x, &y));
/*
    let path = Path::new("../data_files/ex1data2.txt");
    let (x_ptr, y_ptr) = read_data::get_data(&path);

    let x = *x_ptr;
    let y = *y_ptr;

    let alpha = 0.01;   // the learning speed
    let num_iters = 400;    // Number of gradient descent iterations
    let mut theta: Vec<f32> = vec![0.0; x[0].len()];

    let (ex_2_nor_y, y_mean, y_std_dev)
                    = feature_scaling::mean_normal_y(&y);

    let (ex_2_nor_x, x_mean, x_std_dev)
                    = feature_scaling::mean_normal_x(&x);

    println!("X Mean: {:?}", x_mean);
    println!("X Standard Deviation: {:?}", x_std_dev);

    println!("Y Mean: {}", y_mean);
    println!("Y Standard Deviation: {}", y_std_dev);

    println!("Thetas are {:?}, J (the cost) is {}", theta,
    cost_functions::get_cost(&ex_2_nor_x, &ex_2_nor_y, &theta));

    println!("Found thetas using Gradient Descent: {:?}",
        gradient_descent::get_thetas(&ex_2_nor_x,
            &ex_2_nor_y,
            alpha,
            &mut theta, num_iters));
 */

}