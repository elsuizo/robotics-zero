/// A Robotics crate
pub mod matrix2x2; //<---
pub mod matrix3x3; //   |
pub mod matrix4x4; //   |
pub mod matrix5x5;
pub mod matrix6x6; //   |
pub mod vector2;   //   | // NOTE(elsuizo:2020-04-22): todo esto tendria que ir en un crate aparte???
pub mod vector3;   //   |
pub mod vector4;   //   |
pub mod vector5;
pub mod vector6;   //<--
pub mod types;
pub mod errors;
pub mod transformations;
pub mod utils;

pub mod linear_algebra;

//-------------------------------------------------------------------------
//                        tests
//-------------------------------------------------------------------------

#[cfg(test)]
mod test_matrix2x2 {
    use crate::matrix2x2::Matrix2x2;
    use crate::vector2::Vector2;
    use crate::utils::check_assert_matrix2x2;
    use crate::utils::compare_floats;
    use crate::linear_algebra::LinearAlgebra;

    #[test]
    fn create_matrix() {
        let matrix = Matrix2x2::new([[0.0, 1.0],
                                     [2.0, 3.0]]);
        assert_eq!(matrix[(0, 0)], 0.0);
        assert_eq!(matrix[(0, 1)], 1.0);
        assert_eq!(matrix[(1, 0)], 2.0);
        assert_eq!(matrix[(1, 1)], 3.0);
    }

    #[test]
    fn test_identity_creation() {
        let expected = Matrix2x2::new([[1.0, 0.0],
                                     [0.0, 1.0]]);
        let result: Matrix2x2<f64> = Matrix2x2::identity();
        assert_eq!(result.as_vec(), expected.as_vec());
    }

    #[test]
    fn add_matrix() {
        let m1 = Matrix2x2::new([[1.0, 2.0],
                                 [3.0, 4.0]]);
        let m2 = Matrix2x2::new([[5.0, 6.0],
                                 [7.0, 8.0]]);
        let expected = Matrix2x2::new([[6.0, 8.0],
                                       [10.0, 12.0]]);
        let result  = m1 + m2;
        assert_eq!(result.as_vec(), expected.as_vec());
    }

    #[test]
    fn test_determinant() {
        let m1 = Matrix2x2::new([[1.0, 2.0],
                                 [3.0, 4.0]]);
        let d = m1.det();
        assert_eq!(compare_floats(d, -2.0), true);
    }

    #[test]
    fn product_with_vector2_rhs_test() {
        let m1 = Matrix2x2::new([[1.0, 2.0],
                                 [3.0, 4.0]]);
        let v = Vector2::new([1.0, 2.0]);

        let result = m1 * v;
        let expected = Vector2::new([5.0, 11.0]);
        assert_eq!(&result[..], &expected[..], "\nExpected\n{:?}\nfound\n{:?}", &result[..], &expected[..]);
    }

    #[test]
    fn product_with_matrix2x2_rhs_test() {
        let v = Vector2::new([1.0, 2.0]);
        let m1 = Matrix2x2::new([[1.0, 2.0],
                                 [3.0, 4.0]]);
        let result = v * m1;
        let expected = Vector2::new([7.0, 10.0]);
        assert_eq!(&result[..], &expected[..], "\nExpected\n{:?}\nfound\n{:?}", &result[..], &expected[..]);
    }
    #[test]
    fn inverse_test() {
        let m1 = Matrix2x2::new([[1.0, 2.0],
                                 [3.0, 4.0]]);
        let expected = Matrix2x2::new([[-2.0, 1.0],
                                       [1.5, -0.5]]);
        if let Ok(result) = m1.inverse() {
            check_assert_matrix2x2(&expected, &result);
        }
    }
}

#[cfg(test)]
mod test_matrix3x3 {
    use crate::matrix3x3::Matrix3x3;
    use crate::utils::check_assert_matrix3x3;
    use crate::utils::compare_floats;
    use crate::linear_algebra::LinearAlgebra;

    #[test]
    fn create_matrix() {
        let matrix = Matrix3x3::new([[0.0, 1.0, 2.0],
                                     [3.0, 4.0, 5.0],
                                     [6.0, 7.0, 8.0],]);
        assert_eq!(matrix[(0, 0)], 0.0);
        assert_eq!(matrix[(0, 1)], 1.0);
        assert_eq!(matrix[(0, 2)], 2.0);
        assert_eq!(matrix[(1, 0)], 3.0);
        assert_eq!(matrix[(1, 1)], 4.0);
        assert_eq!(matrix[(1, 2)], 5.0);
        assert_eq!(matrix[(2, 0)], 6.0);
        assert_eq!(matrix[(2, 1)], 7.0);
        assert_eq!(matrix[(2, 2)], 8.0);
    }

    #[test]
    fn trace_test() {
        let matrix = Matrix3x3::new([[0.0, 1.0, 2.0],
                                     [3.0, 4.0, 5.0],
                                     [6.0, 7.0, 8.0],]);
        assert_eq!(matrix.trace(), 12.0);
    }

    #[test]
    fn add_matrix() {
        let m1 = Matrix3x3::new([[0.0, 1.0, 2.0],
                                 [3.0, 4.0, 5.0],
                                 [6.0, 7.0, 8.0],]);

        let m2 = Matrix3x3::new([[0.0, 1.0, 2.0],
                                 [3.0, 4.0, 5.0],
                                 [6.0, 7.0, 8.0],]);

        let expected = Matrix3x3::new([[0.0, 2.0, 4.0],
                                       [6.0, 8.0, 10.0],
                                       [12.0, 14.0, 16.0],]);
        let result = m1 + m2;
        check_assert_matrix3x3(&expected, &result);

    }

    #[test]
    fn test_identity_creation() {

        let identity: Matrix3x3<f64> = Matrix3x3::identity();

        let expected = Matrix3x3::new([[1.0, 0.0, 0.0],
                                      [0.0, 1.0, 0.0],
                                      [0.0, 0.0, 1.0],]);
        check_assert_matrix3x3(&identity, &expected);
    }

    #[test]
    fn test_zeros_creation() {
        let zero: Matrix3x3<f64> = Matrix3x3::zeros();

        let expected = Matrix3x3::new([[0.0, 0.0, 0.0],
                                       [0.0, 0.0, 0.0],
                                       [0.0, 0.0, 0.0],]);
        check_assert_matrix3x3(&zero, &expected);
    }

    #[test]
    fn test_trace() {
        let m: Matrix3x3<f64> = Matrix3x3::identity();
        assert_eq!(m.trace(), 3.0);
    }

    #[test]
    fn test_norm2() {
        let m = Matrix3x3::new([[0.0, 1.0, 2.0],
                                [3.0, 4.0, 5.0],
                                [6.0, 7.0, 8.0],]);
        assert_eq!(compare_floats(m.norm2(), 14.2828568570857), true);
    }

    #[test]
    fn determinant_test() {
        let m = Matrix3x3::new([[0.0, 1.0, 2.0],
                                [3.0, 4.0, 5.0],
                                [6.0, 7.0, 9.0],]);
        let expected = -3.0;
        let result = m.det();

        assert_eq!(compare_floats(result, expected), true);
    }

    #[test]
    fn inverse_test() {
        let m = Matrix3x3::new([[1.0, 0.0, 3.0],
                                [2.0, 1.0, 6.0],
                                [1.0, 0.0, 9.0],]);
        // NOTE(elsuizo:2019-09-25): hay que buscar una que tenga una inversa mas facil jasjdfjas
        let expected = Matrix3x3::new([[1.5, 0.0, -0.5],
                                      [-2.0, 1.0, 0.0],
                                      [-0.16666666666666666, 0.0, 0.16666666666666666],]);

        if let Ok(result) = m.inverse() {
            check_assert_matrix3x3(&expected, &result);
        }
    }
}

#[cfg(test)]
mod test_matrix4x4 {
    use crate::matrix4x4::Matrix4x4;
    use crate::matrix3x3::Matrix3x3;
    use crate::utils::check_assert_matrix4x4;
    use crate::utils::check_assert_matrix3x3;
    use crate::utils::compare_floats;
    use crate::linear_algebra::LinearAlgebra;

    #[test]
    fn matrix4x4_create_matrix4x4_test() {
        let m = Matrix4x4::new([[1.0, 2.0, 3.0, 4.0],
                                [5.0, 6.0, 7.0, 8.0],
                                [9.0, 10.0, 11.0, 12.0],
                                [13.0, 14.0, 15.0, 16.0]]);

        assert_eq!(m[(1, 1)], 6.0);
    }

    #[test]
    fn matrix4x4_identity_creation_test() {
        let expected = Matrix4x4::new([[1.0, 0.0, 0.0, 0.0],
                                       [0.0, 1.0, 0.0, 0.0],
                                       [0.0, 0.0, 1.0, 0.0],
                                       [0.0, 0.0, 0.0, 1.0]]);
        let identity: Matrix4x4<f64> = Matrix4x4::identity();
        check_assert_matrix4x4(&identity, &expected);
    }

    #[test]
    fn matrix4x4_sum_test() {
        let m1 = Matrix4x4::new([[1.0, 2.0, 3.0, 4.0],
                                [5.0, 6.0, 7.0, 8.0],
                                [9.0, 10.0, 11.0, 12.0],
                                [13.0, 14.0, 15.0, 16.0]]);

        let m2 = Matrix4x4::new([[1.0, 2.0, 3.0, 4.0],
                                [5.0, 6.0, 7.0, 8.0],
                                [9.0, 10.0, 11.0, 12.0],
                                [13.0, 14.0, 15.0, 16.0]]);

        let expected = Matrix4x4::new([[2.0, 4.0, 6.0, 8.0],
                                [10.0, 12.0, 14.0, 16.0],
                                [18.0, 20.0, 22.0, 24.0],
                                [26.0, 28.0, 30.0, 32.0]]);
        let result = m1 + m2;
        check_assert_matrix4x4(&result, &expected);
    }

    #[test]
    fn matrix4x4_product_test() {
        let m1 = Matrix4x4::new([[1.0, 2.0, 3.0, 4.0],
                                 [5.0, 6.0, 7.0, 8.0],
                                 [9.0, 10.0, 11.0, 12.0],
                                 [13.0, 14.0, 15.0, 16.0]]);

        let m2 = Matrix4x4::new([[1.0, 2.0, 3.0, 4.0],
                                 [5.0, 6.0, 7.0, 8.0],
                                 [9.0, 10.0, 11.0, 12.0],
                                 [13.0, 14.0, 15.0, 16.0]]);

        let expected = Matrix4x4::new([[90.0, 100.0, 110.0, 120.0],
                                       [202.0, 228.0, 254.0, 280.0],
                                       [314.0, 356.0, 398.0, 440.0],
                                       [426.0, 484.0, 542.0, 600.0]]);
        let result = m1 * m2;
        check_assert_matrix4x4(&result, &expected);
    }

    #[test]
    fn matrix4x4_det_test() {
        let m1 = Matrix4x4::new([[1.0, 2.0, 3.0, 1.0],
                                [5.0, 6.0, 7.0, 8.0],
                                [9.0, 0.0, 11.0, 12.0],
                                [13.0, 1.0, 15.0, 16.0]]);

        let expected = 168.0;
        let result = m1.det();
        assert_eq!(result, expected);
    }

    #[test]
    fn matrix4x4_norm_test() {
        let m1 = Matrix4x4::new([[1.0, 2.0, 3.0, 4.0],
                                [5.0, 6.0, 7.0, 8.0],
                                [9.0, 10.0, 11.0, 12.0],
                                [13.0, 14.0, 15.0, 16.0]]);
        // NOTE(elsuizo:2019-08-08): el resultado lo calculo con Julia
        let expected = 38.67815921162743;
        let result = m1.norm2();
        assert_eq!(compare_floats(result, expected), true);
    }

    #[test]
    fn matrix4x4_transpose_test() {
        let m1 = Matrix4x4::new([[1.0, 2.0, 3.0, 4.0],
                                 [5.0, 6.0, 7.0, 8.0],
                                 [9.0, 10.0, 11.0, 12.0],
                                 [13.0, 14.0, 15.0, 16.0]]);

        let expected = Matrix4x4::new([[1.0, 5.0, 9.0, 13.0],
                                       [2.0, 6.0, 10.0, 14.0],
                                       [3.0, 7.0, 11.0, 15.0],
                                       [4.0, 8.0, 12.0, 16.0]]);
        let result = m1.transpose();
        check_assert_matrix4x4(&result, &expected);
    }

    #[test]
    fn matrix4x4_zeros_test() {
        let expected = Matrix4x4::new([[0.0, 0.0, 0.0, 0.0],
                                       [0.0, 0.0, 0.0, 0.0],
                                       [0.0, 0.0, 0.0, 0.0],
                                       [0.0, 0.0, 0.0, 0.0]]);
        let result: Matrix4x4<f64> = Matrix4x4::zeros();
        check_assert_matrix4x4(&result, &expected);
    }

    #[test]
    fn matrix4x4_get_submatrix_test() {

        let m = Matrix4x4::new([[1.0, 2.0, 3.0, 4.0],
                                [5.0, 6.0, 7.0, 8.0],
                                [9.0, 10.0, 11.0, 12.0],
                                [13.0, 14.0, 15.0, 16.0]]);

        let result1 = m.get_submatrix((0, 0));

        let expected1 = Matrix3x3::new([[6.0, 7.0, 8.0],
                                       [10.0, 11.0, 12.0],
                                       [14.0, 15.0, 16.0],]);

        check_assert_matrix3x3(&result1, &expected1);

        let result2 = m.get_submatrix((0, 1));

        let expected2 = Matrix3x3::new([[5.0, 7.0, 8.0],
                                       [9.0, 11.0, 12.0],
                                       [13.0, 15.0, 16.0],]);

        check_assert_matrix3x3(&result2, &expected2);

        let result3 = m.get_submatrix((0, 2));

        let expected3 = Matrix3x3::new([[5.0, 6.0, 8.0],
                                       [9.0, 10.0, 12.0],
                                       [13.0, 14.0, 16.0],]);

        check_assert_matrix3x3(&result3, &expected3);
    }

    #[test]
    fn matrix4x4_inverse_test() {
        let m = Matrix4x4::new([[1.0, 1.0, 1.0, -1.0],
                                [1.0, 1.0, -1.0, 1.0],
                                [1.0, -1.0, 1.0, 1.0],
                                [-1.0, 1.0, 1.0, 1.0]]);

        let expected = Matrix4x4::new([[1.0/4.0, 1.0/4.0, 1.0/4.0, -1.0/4.0],
                                      [1.0/4.0, 1.0/4.0, -1.0/4.0, 1.0/4.0],
                                      [1.0/4.0, -1.0/4.0, 1.0/4.0, 1.0/4.0],
                                      [-1.0/4.0, 1.0/4.0, 1.0/4.0, 1.0/4.0]]);

        if let Ok(result) = m.inverse() {
            check_assert_matrix4x4(&result, &expected);
        }
    }
}

#[cfg(test)]
mod test_matrix5x5 {
    use crate::matrix5x5::Matrix5x5;

    use crate::utils::compare_floats;
    use crate::utils::check_assert_matrix5x5;
    use crate::linear_algebra::LinearAlgebra;
    #[test]
    fn matrix5x5_det_test() {
        let m = Matrix5x5::new([[10.0, 1.0, 7.0,  1.0,  5.0],
                                [ 2.0, 4.0, 8.0,  3.0,  2.0],
                                [ 5.0, 1.0, 2.0,  9.0, 10.0],
                                [ 6.0, 9.0, 9.0,  7.0,  3.0],
                                [ 1.0, 8.0, 8.0, 10.0,  5.0]]);
        let result = m.det();
        let expected = 49.99999999999798;
        assert_eq!(compare_floats(result, expected), true);
    }
    #[test]
    fn matrix5x5_sum_test() {
        let m = Matrix5x5::new([[10.0, 1.0, 7.0,  1.0,  5.0],
                                [ 2.0, 4.0, 8.0,  3.0,  2.0],
                                [ 5.0, 1.0, 2.0,  9.0, 10.0],
                                [ 6.0, 9.0, 9.0,  7.0,  3.0],
                                [ 1.0, 8.0, 8.0, 10.0,  5.0]]);

        let expected = Matrix5x5::new([[20.0,  2.0, 14.0,  2.0, 10.0],
                                       [ 4.0,  8.0, 16.0,  6.0,  4.0],
                                       [10.0,  2.0,  4.0, 18.0, 20.0],
                                       [12.0, 18.0, 18.0, 14.0,  6.0],
                                       [ 2.0, 16.0, 16.0, 20.0, 10.0]]);
        let result = m + m;

        check_assert_matrix5x5(&result, &expected);

    }
    #[test]
    fn matrix5x5_product_test() {
        let m = Matrix5x5::new([[10.0, 1.0, 7.0,  1.0,  5.0],
                                [ 2.0, 4.0, 8.0,  3.0,  2.0],
                                [ 5.0, 1.0, 2.0,  9.0, 10.0],
                                [ 6.0, 9.0, 9.0,  7.0,  3.0],
                                [ 1.0, 8.0, 8.0, 10.0,  5.0]]);
        let result = m * m;
        let expected = Matrix5x5::new([[148.0,  70.0, 141.0, 133.0, 150.0],
                                       [ 88.0,  69.0, 105.0, 127.0, 117.0],
                                       [126.0, 172.0, 208.0, 189.0, 124.0],
                                       [168.0, 138.0, 219.0, 193.0, 174.0],
                                       [131.0, 171.0, 217.0, 217.0, 156.0]]);

        check_assert_matrix5x5(&result, &expected);
    }
    #[test]
    fn matrix5x5_norm2_test() {
        let m = Matrix5x5::new([[10.0, 1.0, 7.0,  1.0,  5.0],
                                [ 2.0, 4.0, 8.0,  3.0,  2.0],
                                [ 5.0, 1.0, 2.0,  9.0, 10.0],
                                [ 6.0, 9.0, 9.0,  7.0,  3.0],
                                [ 1.0, 8.0, 8.0, 10.0,  5.0]]);

        let result = m.norm2();
        let expected = 31.52776554086889;
        assert_eq!(compare_floats(result, expected), true);
    }
    #[test]
    fn matrix5x5_const_product_test() {
        let m = Matrix5x5::new([[10.0, 1.0, 7.0,  1.0,  5.0],
                                [ 2.0, 4.0, 8.0,  3.0,  2.0],
                                [ 5.0, 1.0, 2.0,  9.0, 10.0],
                                [ 6.0, 9.0, 9.0,  7.0,  3.0],
                                [ 1.0, 8.0, 8.0, 10.0,  5.0]]);

        let result = m * 0.5;
        let expected = Matrix5x5::new([[5.0, 0.5, 3.5,  0.5,  2.5],
                                [ 1.0, 2.0, 4.0,  1.5,  1.0],
                                [ 2.5, 0.5, 1.0,  4.5, 5.0],
                                [ 3.0, 4.5, 4.5,  3.5,  1.5],
                                [ 0.5, 4.0, 4.0, 5.0,  2.5]]);
        check_assert_matrix5x5(&result, &expected);
    }
    #[test]
    fn matrix5x5_inv_test() {
        let m = Matrix5x5::new([[10.0, 1.0, 7.0,  1.0,  5.0],
                                [ 2.0, 4.0, 8.0,  3.0,  2.0],
                                [ 5.0, 1.0, 2.0,  9.0, 10.0],
                                [ 6.0, 9.0, 9.0,  7.0,  3.0],
                                [ 1.0, 8.0, 8.0, 10.0,  5.0]]);
        let expected = Matrix5x5::new([[-11.98,   15.64,    9.32,   10.34,  -19.12],
                                       [33.62 , -44.16 , -26.08 , -28.46 ,  53.28 ],
                                       [-9.36 ,  12.48 ,   7.24 ,   7.88 , -14.84 ],
                                       [-37.2 ,   48.6 ,   28.8 ,   31.6 ,  -58.8 ],
                                       [37.98 , -49.64 , -29.32 , -32.34 ,  60.12 ]]);

        if let Ok(result) = m.inverse() {
            check_assert_matrix5x5(&result, &expected);
        }
    }

}

#[cfg(test)]
mod test_matrix6x6 {
    use crate::matrix6x6::Matrix6x6;
    // use crate::matrix3x3::Matrix3x3;
    // use crate::utils::check_assert_matrix4x4;
    use crate::utils::check_assert_matrix6x6;
    use crate::utils::compare_floats;
    use crate::linear_algebra::LinearAlgebra;

    #[test]
    fn matrix6x6_det_test() {
        // let m: Matrix6x6<f64> = Matrix6x6::zeros();
        let m = Matrix6x6::new([[ 1.0,  1.0, 3.0,  4.0,  9.0, 3.0],
                                [10.0, 10.0, 1.0,  2.0,  2.0, 5.0],
                                [ 2.0,  9.0, 6.0, 10.0, 10.0, 9.0],
                                [10.0,  9.0, 9.0,  7.0,  3.0, 6.0],
                                [ 7.0,  6.0, 6.0,  2.0,  9.0, 5.0],
                                [ 3.0,  8.0, 1.0,  4.0,  1.0, 5.0]]);
        let result = m.det();
        let expected = 3271.9999999999723;
        assert_eq!(compare_floats(result, expected), true);
    }
    #[test]
    fn matrix6x6_mul_test() {
        let m = Matrix6x6::new([[0.0 , 1.0 , 2.0 , 3.0,  4.0,  5.0 ],
                                [6.0 , 7.0 , 8.0 , 9.0, 10.0, 11.0 ],
                                [12.0, 13.0, 14.0, 15.0, 16.0, 17.0],
                                [18.0, 19.0, 20.0, 21.0, 22.0, 23.0],
                                [24.0, 25.0, 26.0, 27.0, 28.0, 29.0],
                                [30.0, 31.0, 32.0, 33.0, 34.0, 35.0]]);
        let result = m * m;
        let expected = Matrix6x6::new([[330.0 , 345.0 , 360.0 ,  375.0,   390.0,  405.0 ],
                                       [870.0 , 921.0 , 972.0 ,  1023.0,  1074.0, 1125.0],
                                       [1410.0, 1497.0, 1584.0,  1671.0,  1758.0, 1845.0],
                                       [1950.0, 2073.0, 2196.0,  2319.0,  2442.0, 2565.0],
                                       [2490.0, 2649.0, 2808.0,  2967.0,  3126.0, 3285.0],
                                       [3030.0, 3225.0, 3420.0,  3615.0,  3810.0, 4005.0]]);

        check_assert_matrix6x6(&result, &expected);
    }
    #[test]
    fn matrix6x6_norm2_test() {
        let m = Matrix6x6::new([[0.0 , 1.0 , 2.0 , 3.0,  4.0,  5.0 ],
                                [6.0 , 7.0 , 8.0 , 9.0, 10.0, 11.0 ],
                                [12.0, 13.0, 14.0, 15.0, 16.0, 17.0],
                                [18.0, 19.0, 20.0, 21.0, 22.0, 23.0],
                                [24.0, 25.0, 26.0, 27.0, 28.0, 29.0],
                                [30.0, 31.0, 32.0, 33.0, 34.0, 35.0]]);
        let result = m.norm2();
        let expected = 122.10651088291729;
        assert_eq!(compare_floats(result, expected), true);
    }
    #[test]
    fn matrix6x6_inv_test() {
        let m = Matrix6x6::new([[ 1.0,  1.0, 3.0,  4.0,  9.0, 3.0],
                                [10.0, 10.0, 1.0,  2.0,  2.0, 5.0],
                                [ 2.0,  9.0, 6.0, 10.0, 10.0, 9.0],
                                [10.0,  9.0, 9.0,  7.0,  3.0, 6.0],
                                [ 7.0,  6.0, 6.0,  2.0,  9.0, 5.0],
                                [ 3.0,  8.0, 1.0,  4.0,  1.0, 5.0]]);
        let expected = Matrix6x6::new([[-0.538814,  0.577934,  0.665342, -0.0837408, -0.169621, -1.18215],
                                       [ 2.16076 , -1.52751 , -2.44071 ,  0.44132  ,  0.324572,  3.77017],
                                       [ 0.214548, -0.415037, -0.394254,  0.147922 ,  0.197433,  0.621027],
                                       [ 0.700183, -0.240526, -0.525978,  0.203545 , -0.211797,  0.734719],
                                       [ 0.85055 , -0.471577, -0.827934,  0.110636 ,  0.114609,  1.20416],
                                       [-3.90709 ,  2.46699 ,  4.17115 , -0.870416 , -0.310513, -6.07579]]);

        if let Ok(result) = m.inverse() {
            check_assert_matrix6x6(&result, &expected);
        }
    }
}


#[cfg(test)]
mod vector2_test {
    use crate::vector2::Vector2;

    #[test]
    fn create_vector2_test() {
        let v = Vector2::new([1.0, 2.0]);
        assert_eq!(v[0], 1.0);
        assert_eq!(v[1], 2.0);
    }

    #[test]
    fn zero_vector2_test() {
        let result: Vector2<f64> = Vector2::zeros();
        let expected = Vector2::new([0.0, 0.0]);
        assert_eq!(&result[..], &expected[..], "\nExpected\n{:?}\nfound\n{:?}", &result[..], &expected[..]);
    }

    #[test]
    fn product_test() {
        let v1 = Vector2::new([1.0, 2.0]);
        let v2 = Vector2::new([3.0, 4.0]);
        let result = v1 * v2;
        let expected = 11.0;
        assert_eq!(result, expected);
    }

    #[test]
    fn add_test() {
        let v1 = Vector2::new([1.0, 2.0]);
        let v2 = Vector2::new([3.0, 4.0]);
        let result = v1 + v2;
        let expected = Vector2::new([4.0, 6.0]);
        assert_eq!(&result[..], &expected[..], "\nExpected\n{:?}\nfound\n{:?}", &result[..], &expected[..]);
    }

    #[test]
    fn norm2_test() {
        let v1 = Vector2::new([1.0, 2.0]);
        let expected = 2.23606797749979;
        let result = v1.norm2();
        assert_eq!(result, expected);
    }
}

#[cfg(test)]
mod vector3_test {
    use crate::vector3::Vector3;

    #[test]
    fn create_vector3_test() {
        let v = Vector3::new([1.0, 1.0, 1.0]);
        assert_eq!(v[0], 1.0);
        assert_eq!(v[1], 1.0);
        assert_eq!(v[2], 1.0);
    }

    #[test]
    fn zero_vector3_test() {
        let result: Vector3<f64> = Vector3::zeros();
        let expected = Vector3::new([0.0, 0.0, 0.0]);
        assert_eq!(&result[..], &expected[..], "\nExpected\n{:?}\nfound\n{:?}", &result[..], &expected[..]);
    }

    #[test]
    fn product_test() {
        let v1 = Vector3::new([1.0, 2.0, 3.0]);
        let v2 = Vector3::new([4.0, 5.0, 6.0]);
        let result = v1 * v2;
        let expected = 32.0;
        assert_eq!(result, expected);
    }

    #[test]
    fn add_test() {
        let v1 = Vector3::new([1.0, 2.0, 3.0]);
        let v2 = Vector3::new([4.0, 5.0, 6.0]);
        let result = v1 + v2;
        let expected = Vector3::new([5.0, 7.0, 9.0]);
        assert_eq!(&result[..], &expected[..], "\nExpected\n{:?}\nfound\n{:?}", &result[..], &expected[..]);
    }

    #[test]
    fn norm2_test() {
        let v1 = Vector3::new([1.0, 2.0, 3.0]);
        let expected = 3.7416573867739413;
        let result = v1.norm2();
        assert_eq!(result, expected);
    }
}

#[cfg(test)]
mod vector4_test {
    use crate::vector4::Vector4;
    use crate::matrix4x4::Matrix4x4;

    #[test]
    fn vector4_creation_test() {
        let v = Vector4::new([1, 1, 1, 1]);
        assert_eq!(v[0], 1);
        assert_eq!(v[1], 1);
        assert_eq!(v[2], 1);
        assert_eq!(v[3], 1);
    }
    #[test]
    fn vector4_zeros_test() {
        let result: Vector4<f32> = Vector4::zeros();
        let expected = Vector4::new([0.0, 0.0, 0.0, 0.0]);
        assert_eq!(&result[..], &expected[..], "\nExpected\n{:?}\nfound\n{:?}", &result[..], &expected[..]);
    }
    #[test]
    fn vector4_sum_test() {
        let v1 = Vector4::new([1.0, 2.0, 3.0, 4.0]);
        let v2 = Vector4::new([5.0, 6.0, 7.0, 8.0]);
        let result = v1 + v2;
        let expected = Vector4::new([6.0, 8.0, 10.0, 12.0]);
        assert_eq!(&result[..], &expected[..], "\nExpected\n{:?}\nfound\n{:?}", &result[..], &expected[..]);
    }
    #[test]
    fn vector4_product_test() {
        let v1 = Vector4::new([1.0, 2.0, 3.0, 4.0]);
        let v2 = Vector4::new([5.0, 6.0, 7.0, 8.0]);
        let result = v1 * v2;
        let expected = 70.0;
        assert_eq!(result, expected);
    }
    #[test]
    fn vector4_norm_test() {
        let v1 = Vector4::new([1.0, 2.0, 3.0, 4.0]);
        let result = v1.norm2();
        let expected = 5.477225575051661;
        assert_eq!(result, expected);
    }
    #[test]
    fn vector4_mul_matrix4x4_test() {
        let m = Matrix4x4::new([[1.0, 2.0, 3.0, 4.0],
                                [5.0, 6.0, 7.0, 8.0],
                                [9.0, 10.0, 11.0, 12.0],
                                [13.0, 14.0, 15.0, 16.0]]);

        let v1 = Vector4::new([1.0, 2.0, 3.0, 4.0]);
        let result = v1 * m;
        let expected = Vector4::new([90.0, 100.0, 110.0, 120.0]);
        assert_eq!(&result[..], &expected[..], "\nExpected\n{:?}\nfound\n{:?}", &result[..], &expected[..]);
    }
}

#[cfg(test)]
mod vector5_test {
    use crate::vector5::Vector5;
    use crate::matrix5x5::Matrix5x5;

    #[test]
    fn vector5_creation_test() {
        let v = Vector5::new([1.0, 2.0, 3.0, 4.0, 5.0]);
        assert_eq!(v[0], 1.0);
        assert_eq!(v[1], 2.0);
        assert_eq!(v[2], 3.0);
        assert_eq!(v[3], 4.0);
        assert_eq!(v[4], 5.0);
    }
    #[test]
    fn vector5_zeros_test() {
        let result: Vector5<f32> = Vector5::zeros();
        let expected = Vector5::new([0.0, 0.0, 0.0, 0.0, 0.0]);
        assert_eq!(&result[..], &expected[..], "\nExpected\n{:?}\nfound\n{:?}", &result[..], &expected[..]);
    }
    #[test]
    fn vector5_sum_test() {
        let v1 = Vector5::new([1.0, 2.0, 3.0, 4.0, 5.0]);
        let v2 = Vector5::new([6.0, 7.0, 8.0, 9.0, 10.0]);
        let result = v1 + v2;
        let expected = Vector5::new([7.0, 9.0, 11.0, 13.0, 15.0]);
        assert_eq!(&result[..], &expected[..], "\nExpected\n{:?}\nfound\n{:?}", &result[..], &expected[..]);
    }
    #[test]
    fn vector5_mul_test() {
        let v1 = Vector5::new([1.0, 2.0, 3.0, 4.0, 5.0]);
        let v2 = Vector5::new([6.0, 7.0, 8.0, 9.0, 10.0]);
        let result = v1 * v2;
        let expected = 130.0;
        assert_eq!(result, expected);
    }
    #[test]
    fn vector5_norm_test() {
        let v1 = Vector5::new([1.0, 2.0, 3.0, 4.0, 5.0]);
        let result = v1.norm2();
        let expected = 7.416198487095663;
        assert_eq!(result, expected);
    }
    #[test]
    fn vector5_mul_matrix5x5_test() {
        let v1 = Vector5::new([1.0, 2.0, 3.0, 4.0, 5.0]);
        let m = Matrix5x5::new([[10.0, 1.0, 7.0,  1.0,  5.0],
                                [ 2.0, 4.0, 8.0,  3.0,  2.0],
                                [ 5.0, 1.0, 2.0,  9.0, 10.0],
                                [ 6.0, 9.0, 9.0,  7.0,  3.0],
                                [ 1.0, 8.0, 8.0, 10.0,  5.0]]);
        let result = v1 * m;
        let expected = Vector5::new([58.0, 88.0, 105.0, 112.0, 76.0]);
        assert_eq!(&result[..], &expected[..], "\nExpected\n{:?}\nfound\n{:?}", &result[..], &expected[..]);
    }
}

#[cfg(test)]
mod vector6_tests {
    use crate::vector6::Vector6;
    use crate::matrix6x6::Matrix6x6;

    #[test]
    fn vector6_creation_test() {
        let v = Vector6::new([1.0,2.0,3.0,4.0,5.0,6.0]);
        assert_eq!(v[0], 1.0);
        assert_eq!(v[1], 2.0);
        assert_eq!(v[2], 3.0);
        assert_eq!(v[3], 4.0);
        assert_eq!(v[4], 5.0);
        assert_eq!(v[5], 6.0);
    }
    #[test]
    fn vector6_sum_test() {
        let v        = Vector6::new([1.0,2.0,3.0,4.0,5.0,6.0]);
        let result   = v + v;
        let expected = Vector6::new([2.0,4.0,6.0,8.0,10.0,12.0]);
        assert_eq!(&result[..], &expected[..], "\nExpected\n{:?}\nfound\n{:?}", &result[..], &expected[..]);
    }
    #[test]
    fn product_test() {
        let v        = Vector6::new([1.0,2.0,3.0,4.0,5.0,6.0]);
        let result   = v * v;
        let expected = 91.0;
        assert_eq!(result, expected);
    }
    #[test]
    fn product_matrix6x6_test() {
        let v        = Vector6::new([1.0,2.0,3.0,4.0,5.0,6.0]);

        let m = Matrix6x6::new([[0.0 , 1.0 , 2.0 , 3.0,  4.0,  5.0 ],
                                [6.0 , 7.0 , 8.0 , 9.0, 10.0, 11.0 ],
                                [12.0, 13.0, 14.0, 15.0, 16.0, 17.0],
                                [18.0, 19.0, 20.0, 21.0, 22.0, 23.0],
                                [24.0, 25.0, 26.0, 27.0, 28.0, 29.0],
                                [30.0, 31.0, 32.0, 33.0, 34.0, 35.0]]);
        let result = v * m;
        let expected = Vector6::new([420.0, 441.0, 462.0, 483.0, 504.0, 525.0]);
        assert_eq!(&result[..], &expected[..], "\nExpected\n{:?}\nfound\n{:?}", &result[..], &expected[..]);
    }
}

#[cfg(test)]
mod types_tests {
    use crate::types::{Point2D, Point};

    #[test]
    fn point2d_test() {
        let p = Point2D::new(1, 2, "A");

        assert_eq!(1, p.x);
        assert_eq!(2, p.y);
        assert_eq!("A", p.frame_name);
    }

    #[test]
    fn point_test() {
        let p = Point::new(1, 1, 3, "B");
        assert_eq!(1, p.x);
        assert_eq!(1, p.y);
        assert_eq!(3, p.z);
        assert_eq!("B", p.frame_name);
    }
}

#[cfg(test)]
mod transformations_tests {
    use crate::transformations;
    use crate::utils::{check_assert_matrix3x3, check_assert_matrix4x4, compare_floats, is_rotation};
    use crate::matrix3x3::Matrix3x3;
    use crate::matrix4x4::Matrix4x4;

    #[test]
    fn rotx_test() {
        let rot1 = transformations::rotx(0.0);
        let rot2 = transformations::rotx(360.0);

        let i = Matrix3x3::identity();
        assert_eq!(is_rotation(rot1), true);
        assert_eq!(is_rotation(rot2), true);
        check_assert_matrix3x3(&rot1, &rot2);
        check_assert_matrix3x3(&i, &rot2);
        check_assert_matrix3x3(&i, &rot1);
    }

    #[test]
    fn roty_test() {
        let rot1 = transformations::roty(0.0);
        let rot2 = transformations::roty(360.0);

        let i = Matrix3x3::identity();

        assert_eq!(is_rotation(rot1), true);
        assert_eq!(is_rotation(rot2), true);
        check_assert_matrix3x3(&rot1, &rot2);
        check_assert_matrix3x3(&i, &rot2);
        check_assert_matrix3x3(&i, &rot1);
    }

    #[test]
    fn rotz_test() {
        let rot1 = transformations::rotz(0.0);
        let rot2 = transformations::rotz(360.0);

        let i = Matrix3x3::identity();

        assert_eq!(is_rotation(rot1), true);
        assert_eq!(is_rotation(rot2), true);
        check_assert_matrix3x3(&rot1, &rot2);
        check_assert_matrix3x3(&i, &rot2);
        check_assert_matrix3x3(&i, &rot1);
    }

    #[test]
    fn rot2trans_test() {
        let rot       = transformations::rotx(0.0);
        let rot_trans = transformations::rot2trans(&rot);

        let i = Matrix4x4::identity();

        check_assert_matrix4x4(&i, &rot_trans);
    }

    #[test]
    fn euler_test() {
        let angles_in = (90.0, 30.0, 30.0);
        let rot = transformations::euler2rot(angles_in.0, angles_in.1, angles_in.2);
        let angles_out = transformations::rot2euler(rot);

        assert_eq!(compare_floats(angles_in.0, angles_out.0), true);
        assert_eq!(compare_floats(angles_in.1, angles_out.1), true);
        assert_eq!(compare_floats(angles_in.2, angles_out.2), true);
    }
}
