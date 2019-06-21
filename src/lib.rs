/// A Robotics crate
///

pub mod matrix;

//-------------------------------------------------------------------------
//                        tests
//-------------------------------------------------------------------------
#[cfg(test)]
mod test_matrix {
    use crate::matrix::Matrix3x3;

    #[test]
    fn create_matrix() {
        let matrix = Matrix3x3::new([
                                    [0.0, 1.0, 2.0],
                                    [3.0, 4.0, 5.0],
                                    [6.0, 7.0, 8.0],
                                 ]);
        assert_eq!(matrix[0][2], 2.0);
    }
}
