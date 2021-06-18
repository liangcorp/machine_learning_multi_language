pub mod feature_scaling;
pub mod linear_regression;
pub mod read_data;

#[cfg(test)]

mod tests {
	#[test]
	fn four_by_four(){
		let matrix = vec![
			vec![1.0, 1.0, 1.0, -1.0],
			vec![1.0, 1.0, -1.0, 1.0],
			vec![1.0, -1.0, 1.0, 1.0],
			vec![-1.0, 1.0, 1.0, 1.0]];

		assert_eq!(super::linear_regression::normal_equation::get_determinant(&matrix), -16 as f64);
	}
}
