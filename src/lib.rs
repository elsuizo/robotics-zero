/// A Robotics crate
///
#[cfg(test)]
#[macro_use]
extern crate approx;

pub mod matrix3x3;
pub mod matrix2x2;
pub mod matrix4x4;

//-------------------------------------------------------------------------
//                        auxiliar functions
//-------------------------------------------------------------------------

//-------------------------------------------------------------------------
//                        tests
//-------------------------------------------------------------------------
#[cfg(test)]
mod test_matrix2x2 {
    use crate::matrix2x2::Matrix2x2;

    #[test]
    fn create_matrix() {
        let matrix = Matrix2x2::new([[0.0, 1.0],
                                     [2.0, 3.0]]);
        assert_eq!(matrix[0][0], 0.0);
    }

    #[test]
    fn test_identity_creation() {
        let expected = Matrix2x2::new([[1.0, 0.0],
                                     [0.0, 1.0]]);
        let I: Matrix2x2<f64> = Matrix2x2::identity();
        assert_eq!(&I[..], &expected[..], "\nExpected\n{:?}\nfound\n{:?}", &I[..], &expected[..]);
    }

    #[test]
    fn add_matrix() {
        let expected = Matrix2x2::new([[6.0, 8.0],
                                     [10.0, 12.0]]);
        let m1 = Matrix2x2::new([[1.0, 2.0],
                                 [3.0, 4.0]]);
        let m2 = Matrix2x2::new([[5.0, 6.0],
                                 [7.0, 8.0]]);
        let m = m1 + m2;
        assert_eq!(&m[..], &expected[..], "\nExpected\n{:?}\nfound\n{:?}", &m[..], &expected[..]);
    }

    #[test]
    fn test_determinant() {
        let m1 = Matrix2x2::new([[1.0, 2.0],
                                 [3.0, 4.0]]);
        let d = m1.det();
        assert_ulps_eq!(d, -2.0);
    }
}

mod test_matrix3x3 {
    use super::*;
    use crate::matrix3x3::Matrix3x3;

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
    fn trace_test() {
        let matrix = Matrix3x3::new([
                                    [0.0, 1.0, 2.0],
                                    [3.0, 4.0, 5.0],
                                    [6.0, 7.0, 8.0],
                                 ]);
        assert_eq!(matrix.trace(), 12.0);
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

//         // NOTE(elsuizo:2019-06-24): comparo usando slides
        assert_eq!(&m3[..], &expected[..], "\nExpected\n{:?}\nfound\n{:?}", &m3[..], &expected[..]);
    }

    #[test]
    fn test_identity_creation() {

        let I: Matrix3x3<f64> = Matrix3x3::identity();

        let expected = Matrix3x3::new([
                                    [1.0, 0.0, 0.0],
                                    [0.0, 1.0, 0.0],
                                    [0.0, 0.0, 1.0],
                                 ]);
        assert_eq!(&I[..], &expected[..], "\nExpected\n{:?}\nfound\n{:?}", &I[..], &expected[..]);
    }

    #[test]
    fn test_zeros_creation() {
        let zero: Matrix3x3<f64> = Matrix3x3::zeros();

        let expected = Matrix3x3::new([
                                    [0.0, 0.0, 0.0],
                                    [0.0, 0.0, 0.0],
                                    [0.0, 0.0, 0.0],
                                    ]);
        assert_eq!(&zero[..], &expected[..], "\nExpected\n{:?}\nfound\n{:?}", &zero[..], &expected[..]);
    }

    #[test]
    fn test_trace() {
        let m: Matrix3x3<f64> = Matrix3x3::identity();
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
mod test_matrix4x4 {
    use super::*;
    use crate::matrix4x4::Matrix4x4;
    #[test]
    fn create_matrix4x4_test() {
        let m = Matrix4x4::new([[1.0, 2.0, 3.0, 4.0],
                                [5.0, 6.0, 7.0, 8.0],
                                [9.0, 10.0, 11.0, 12.0],
                                [13.0, 14.0, 15.0, 16.0]]);

        assert_eq!(m[0][0], 1.0);
    }
}
