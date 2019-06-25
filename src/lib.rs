/// A Robotics crate
///

pub mod matrix;

//-------------------------------------------------------------------------
//                        auxiliar functions
//-------------------------------------------------------------------------

//-------------------------------------------------------------------------
//                        tests
//-------------------------------------------------------------------------
#[cfg(test)]
mod test_matrix {
    use approx::{AbsDiffEq, RelativeEq, UlpsEq, assert_ulps_eq};
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

    #[test]
    fn add_matrix() {
        let m1 = Matrix3x3::new([
                                    [0.0, 1.0, 2.0],
                                    [3.0, 4.0, 5.0],
                                    [6.0, 7.0, 8.0],
                                 ]);

        let m2 = Matrix3x3::new([
                                    [0.0, 1.0, 2.0],
                                    [3.0, 4.0, 5.0],
                                    [6.0, 7.0, 8.0],
                                 ]);

        let expected = Matrix3x3::new([
                                    [0.0, 2.0, 4.0],
                                    [6.0, 8.0, 10.0],
                                    [12.0, 14.0, 16.0],
                                 ]);
        let m3 = m1 + m2;

        // NOTE(elsuizo:2019-06-24): comparo usando slides
        assert_eq!(&m3[..], &expected[..], "\nExpected\n{:?}\nfound\n{:?}", &m3[..], &expected[..]);
    }

    #[test]
    fn test_identity_creation() {

        let I = Matrix3x3::identity();

        let expected = Matrix3x3::new([
                                    [1.0, 0.0, 0.0],
                                    [0.0, 1.0, 0.0],
                                    [0.0, 0.0, 1.0],
                                 ]);
        assert_eq!(&I[..], &expected[..], "\nExpected\n{:?}\nfound\n{:?}", &I[..], &expected[..]);
    }

    #[test]
    fn test_zeros_creation() {
        let zero = Matrix3x3::zeros();

        let expected = Matrix3x3::new([
                                    [0.0, 0.0, 0.0],
                                    [0.0, 0.0, 0.0],
                                    [0.0, 0.0, 0.0],
                                    ]);
        assert_eq!(&zero[..], &expected[..], "\nExpected\n{:?}\nfound\n{:?}", &zero[..], &expected[..]);
    }

    #[test]
    fn test_trace() {
        let m = Matrix3x3::identity();
        assert_eq!(m.trace(), 3.0);
    }

    #[test]
    fn test_norm2() {
        let m = Matrix3x3::new([
                                    [0.0, 1.0, 2.0],
                                    [3.0, 4.0, 5.0],
                                    [6.0, 7.0, 8.0],
                                 ]);
        assert_ulps_eq!(m.norm2(), 14.2828568570857);
    }
}
