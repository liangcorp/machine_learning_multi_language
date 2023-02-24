pub use std::path::Path;

use ml_rust::{linear_regression, read_data};
// mod feature_scaling;

fn main() {
    let path = Path::new("../data_files/ex1data1.txt");

    let (x_ptr, y_ptr) = match read_data::get_data(path) {
        Ok((x_ptr, y_ptr)) => (x_ptr, y_ptr),
        Err(e) => panic!("{}", e.get_ref().unwrap()),
    };

    let x = *x_ptr;
    let y = *y_ptr;

    // let alpha = 0.01; // the learning speed

    let theta = vec![0.0, 0.0]; // set theta 0 and theta 1 to 0.0

    match linear_regression::cost_functions::get_cost(&x, &y, &theta) {
        Ok(theta) => {
            println!("Thetas are [0.0, 0.0], J(theta) is {:?}", theta);
        }
        Err(e) => panic!("{}", e.get_ref().unwrap()),
    }

    match linear_regression::cost_functions::get_cost(&x, &y, &[-1.0, 2.0]) {
        Ok(theta) => {
            println!("Thetas are [-1.0, 2.0], J(theta) is {:?}", theta);
        }
        Err(e) => panic!("{}", e.get_ref().unwrap()),
    }

    println!("Found thetas using Gradient Descent: {:?}", theta);

    match linear_regression::normal_equation::get_theta(&x, &y) {
        Ok(theta) => {
            println!("Found thetas using Normal Equation: {:?}", theta)
        }
        Err(e) => panic!("{}", e.get_ref().unwrap()),
    }

    /*
       let path = Path::new("../data_files/ex1data2.txt");
       let (x_ptr, y_ptr) = read_data::get_data(&path);

       let x = *x_ptr;
       let y = *y_ptr;

       let alpha = 0.01;   // the learning speed
       let num_iters = 400;    // Number of gradient descent iterations
       let mut theta: Vec<f64> = vec![0.0; x[0].len()];

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
