/// A Robotics crate
pub mod matrix2x2; //<---
pub mod matrix3x3; //   |
pub mod matrix4x4; //   |
pub mod matrix6x6; //   |
pub mod vector2;   //   | // NOTE(elsuizo:2020-04-22): todo esto tendria que ir en un crate aparte???
pub mod vector3;   //   |
pub mod vector4;   //   |
pub mod vector6;   //<--
pub mod types;
pub mod errors;
pub mod transformations;
pub mod utils;

//-------------------------------------------------------------------------
//                        tests
//-------------------------------------------------------------------------

mod test_matrix2x2 {
    use crate::matrix2x2::Matrix2x2;
    use crate::vector2::Vector2;
    use crate::utils::check_assert_matrix2x2;
    use crate::utils::compare_floats;


    #[test]
    fn create_matrix() {
        let matrix = Matrix2x2::new([[0.0, 1.0],
                                     [2.0, 3.0]]);
        assert_eq!(matrix[(0, 0)], 0.0);
    }

    #[test]
    fn test_identity_creation() {
        let expected = Matrix2x2::new([[1.0, 0.0],
                                     [0.0, 1.0]]);
        let identity: Matrix2x2<f64> = Matrix2x2::identity();
        check_assert_matrix2x2(&expected, &identity);
    }

    #[test]
    fn add_matrix() {
        let m1 = Matrix2x2::new([[1.0, 2.0],
                                 [3.0, 4.0]]);
        let m2 = Matrix2x2::new([[5.0, 6.0],
                                 [7.0, 8.0]]);
        let expected = Matrix2x2::new([[6.0, 8.0],
                                       [10.0, 12.0]]);
        let m = m1 + m2;

        check_assert_matrix2x2(&expected, &m);
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

mod test_matrix3x3 {
    use crate::matrix3x3::Matrix3x3;
    use crate::utils::check_assert_matrix3x3;
    use crate::utils::compare_floats;

    #[test]
    fn create_matrix() {
        let matrix = Matrix3x3::new([[0.0, 1.0, 2.0],
                                     [3.0, 4.0, 5.0],
                                     [6.0, 7.0, 8.0],]);
        assert_eq!(matrix[(0, 2)], 2.0);
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

mod test_matrix4x4 {
    use crate::matrix4x4::Matrix4x4;
    use crate::matrix3x3::Matrix3x3;
    use crate::utils::check_assert_matrix4x4;
    use crate::utils::check_assert_matrix3x3;
    use crate::utils::compare_floats;

    #[test]
    fn create_matrix4x4_test() {
        let m = Matrix4x4::new([[1.0, 2.0, 3.0, 4.0],
                                [5.0, 6.0, 7.0, 8.0],
                                [9.0, 10.0, 11.0, 12.0],
                                [13.0, 14.0, 15.0, 16.0]]);

        assert_eq!(m[(0, 0)], 1.0);
    }

    #[test]
    fn identity_creation_test() {
        let expected = Matrix4x4::new([[1.0, 0.0, 0.0, 0.0],
                                       [0.0, 1.0, 0.0, 0.0],
                                       [0.0, 0.0, 1.0, 0.0],
                                       [0.0, 0.0, 0.0, 1.0]]);
        let identity: Matrix4x4<f64> = Matrix4x4::identity();
        check_assert_matrix4x4(&identity, &expected);
    }

    #[test]
    fn sum_test() {
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
    fn product_test() {
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
    fn det_test() {
        let m1 = Matrix4x4::new([[1.0, 2.0, 3.0, 1.0],
                                [5.0, 6.0, 7.0, 8.0],
                                [9.0, 0.0, 11.0, 12.0],
                                [13.0, 1.0, 15.0, 16.0]]);

        let expected = 168.0;
        let result = m1.det();
        assert_eq!(result, expected);
    }

    #[test]
    fn norm_test() {
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
    fn transpose_test() {
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
    fn zeros_test() {
        let expected = Matrix4x4::new([[0.0, 0.0, 0.0, 0.0],
                                       [0.0, 0.0, 0.0, 0.0],
                                       [0.0, 0.0, 0.0, 0.0],
                                       [0.0, 0.0, 0.0, 0.0]]);
        let result: Matrix4x4<f64> = Matrix4x4::zeros();
        check_assert_matrix4x4(&result, &expected);
    }

    #[test]
    fn get_submatrix_test() {

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
    fn inverse_test() {
        let m = Matrix4x4::new([[1.0, 1.0, 1.0, -1.0],
                                [1.0, 1.0, -1.0, 1.0],
                                [1.0, -1.0, 1.0, 1.0],
                                [-1.0, 1.0, 1.0, 1.0]]);

        let expected = Matrix4x4::new([[1.0/4.0, 1.0/4.0, 1.0/4.0, -1.0/4.0],
                                      [1.0/4.0, 1.0/4.0, -1.0/4.0, 1.0/4.0],
                                      [1.0/4.0, -1.0/4.0, 1.0/4.0, 1.0/4.0],
                                      [-1.0/4.0, 1.0/4.0, 1.0/4.0, 1.0/4.0]]);
        let result = m.inverse().unwrap();
        check_assert_matrix4x4(&result, &expected);
    }
}

mod test_matrix6x6 {
    use crate::matrix6x6::Matrix6x6;
    // use crate::matrix3x3::Matrix3x3;
    // use crate::utils::check_assert_matrix4x4;
    use crate::utils::check_assert_matrix6x6;

    #[test]
    fn matrix6x6_det_test() {
        // let m: Matrix6x6<f64> = Matrix6x6::zeros();
        let m = Matrix6x6::new([[0.0 , 1.0 , 2.0 , 3.0,  4.0,  5.0 ],
                                [6.0 , 7.0 , 8.0 , 9.0, 10.0, 11.0 ],
                                [12.0, 13.0, 14.0, 15.0, 16.0, 17.0],
                                [18.0, 19.0, 20.0, 21.0, 22.0, 23.0],
                                [24.0, 25.0, 26.0, 27.0, 28.0, 29.0],
                                [30.0, 31.0, 32.0, 33.0, 34.0, 35.0]]);
        let result = m.det();
        assert_eq!(result, 0.0);
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
}

mod vector2_test {
    use crate::vector2::Vector2;

    #[test]
    fn create_vector2_test() {
        let v = Vector2::new([1.0, 1.0]);
        assert_eq!(v[0], 1.0);
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

mod vector3_test {
    use crate::vector3::Vector3;

    #[test]
    fn create_vector3_test() {
        let v = Vector3::new([1.0, 1.0, 1.0]);
        let v_bool = Vector3::new([true, true, true]);
        assert_eq!(v[0], 1.0);
        assert_eq!(v_bool[0], true);
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

mod vector4_test {
    use crate::vector4::Vector4;

    #[test]
    fn vector4_creation_test() {
        let v = Vector4::new([1, 1, 1, 1]);
        assert_eq!(v[0], 1);
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
}

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
    fn product_Matrix6x6_test() {
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
mod types_tests {
    use crate::types::{Point2D};

    #[test]
    fn point2d_test() {
        let p = Point2D::new(1, 2, "A");

        assert_eq!(1, p.x);
    }
}

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
