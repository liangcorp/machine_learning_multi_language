pub mod feature_scaling;
pub mod linear_regression;
pub mod logistic_regression;
pub mod read_data;

#[cfg(test)]
mod tests {
    use super::linear_regression::normal_equation;

    #[test]
    #[should_panic]
    fn three_by_four() {
        let matrix = vec![
            vec![1.0, 1.0, 1.0, -1.0],
            vec![1.0, 1.0, -1.0, 1.0],
            vec![1.0, -1.0, 1.0, 1.0],
        ];

        match normal_equation::get_determinant(&matrix) {
            Ok(der) => assert_eq!(der, -16_f64),
            Err(e) => panic!("{}", e.get_ref().unwrap()),
        };
    }

    #[test]
    fn four_by_four() {
        let matrix = vec![
            vec![1.0, 1.0, 1.0, -1.0],
            vec![1.0, 1.0, -1.0, 1.0],
            vec![1.0, -1.0, 1.0, 1.0],
            vec![-1.0, 1.0, 1.0, 1.0],
        ];

        match normal_equation::get_determinant(&matrix) {
            Ok(der) => assert_eq!(der, -16_f64),
            Err(e) => panic!("{}", e.get_ref().unwrap()),
        };
    }
}
