use std::path::Path;

mod cost_functions;
mod gradient_descent;
mod feature_scaling;
mod read_data;

fn main() {

    let path = Path::new("../data_files/ex1data1.txt");

    let (x_ptr, y_ptr) = read_data::get_data(&path);

    let x = *x_ptr;
    let y = *y_ptr;

    let alpha = 0.01;   // the learning speed
    let mut theta = vec![0.0, 0.0]; // set theta 0 and theta 1 to 0.0

    println!("Thetas are [0.0, 0.0], J(theta) is {}",
            cost_functions::get_cost(&x, &y, &theta));

    println!("Thetas are [-1.0, 2.0], J(theta) is {}",
             cost_functions::get_cost(&x, &y, &vec![-1.0, 2.0]));

/*     println!("Found thetas using Gradient Descent: {:?}",
            gradient_descent::get_thetas(&x, &y,
                                        alpha, &mut theta, 1500));
 */
    let path = Path::new("../data_files/ex1data2.txt");
    let (x_ptr, y_ptr) = read_data::get_data(&path);

    let x = *x_ptr;
    let y = *y_ptr;
    let mut ex2_theta: Vec<f32> = vec![0.0, 0.0, 0.0];

    let ex_2_nor_y = feature_scaling::mean_normalization(&y);

    println!("Cost (multi-featured): {}",
    cost_functions::get_cost(&x, &ex_2_nor_y, &ex2_theta));

    println!("Final theta for multi-featured X is {:?}",
        gradient_descent::get_thetas(&x,
            &ex_2_nor_y,
            alpha,
            &mut ex2_theta, 400));
}
