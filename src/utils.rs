// -------------------------------------------------------------------------
// @file utils.rs
//
// @date 03/24/20 15:32:22
// @author Martin Noblia
// @email mnoblia@disroot.org
//
// @brief
//
// @detail
//
// Licence:
// This program is free software: you can redistribute it and/or modify
// it under the terms of the GNU General Public License as published by
// the Free Software Foundation, either version 3 of the License, or (at
// your option) any later version.
//
// This program is distributed in the hope that it will be useful, but
// WITHOUT ANY WARRANTY; without even the implied warranty of
// MERCHANTABILITY or FITNESS FOR A PARTICULAR PURPOSE.  See the GNU
// General Public License for more details.
//
// You should have received a copy of the GNU General Public License
//---------------------------------------------------------------------------
use crate::matrix2x2::Matrix2x2;
use crate::matrix3x3::Matrix3x3;
use crate::matrix4x4::Matrix4x4;
use crate::matrix6x6::Matrix6x6;
use crate::matrix5x5::Matrix5x5;
use num_traits::{Float};

// NOTE(elsuizo:2020-03-24): nose si esta bien poner estas funciones aca...
//-------------------------------------------------------------------------
//                        auxiliar functions
//-------------------------------------------------------------------------

// NOTE(elsuizo:2020-03-31): tuve que sacar el T::epsilon() porque era muy chico y algunos tests
// no pasan, habria que ver otra solucion...
pub fn compare_floats<T: Float>(num1: T, num2: T) -> bool {
    Float::abs(num1 - num2) < T::from(1e-5).unwrap()
}

// TODO(elsuizo:2020-04-26): todas estas funciones deberian ser una sola como mucho
// - Una de las cosas que podria aprovechar es que se pueden comparar dos iterators element-wise!!!
// - Otra es hacer un trait que y hacer una funcion que acepte un trait object
pub fn check_assert_matrix2x2<T: Float + std::fmt::Debug>(m1: &Matrix2x2<T>, m2: &Matrix2x2<T>) {
    for i in 0..m1.rows() {
        for j in 0..m1.cols() {
            assert_eq!(compare_floats(m1[(i, j)], m2[(i, j)]), true);
        }
    }
}

pub fn check_assert_matrix3x3<T: Float + std::fmt::Debug>(m1: &Matrix3x3<T>, m2: &Matrix3x3<T>) {
    for i in 0..m1.rows() {
        for j in 0..m1.cols() {
            assert_eq!(compare_floats(m1[(i, j)], m2[(i, j)]), true);
        }
    }
}

pub fn check_assert_matrix4x4<T: Float + std::fmt::Debug>(m1: &Matrix4x4<T>, m2: &Matrix4x4<T>) {
    for i in 0..m1.rows() {
        for j in 0..m1.cols() {
            assert_eq!(compare_floats(m1[(i, j)], m2[(i, j)]), true);
        }
    }
}

pub fn check_assert_matrix5x5<T: Float + std::fmt::Debug>(m1: &Matrix5x5<T>, m2: &Matrix5x5<T>) {
    for i in 0..m1.rows() {
        for j in 0..m1.cols() {
            assert_eq!(compare_floats(m1[(i, j)], m2[(i, j)]), true);
        }
    }
}

pub fn check_assert_matrix6x6<T: Float + std::fmt::Debug>(m1: &Matrix6x6<T>, m2: &Matrix6x6<T>) {
    for i in 0..m1.rows() {
        for j in 0..m1.cols() {
            assert_eq!(compare_floats(m1[(i, j)], m2[(i, j)]), true);
        }
    }
}

pub fn is_rotation<T: Float>(r: Matrix3x3<T>) -> bool {
    let r2 = r * r;
    if compare_floats(r.det(), T::one()) && compare_floats(r2.det(), T::one()) {
        true
    } else {
        false
    }
}
