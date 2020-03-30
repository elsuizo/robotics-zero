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
extern crate assert_approx_eq;

use crate::matrix2x2::Matrix2x2;
use crate::matrix3x3::Matrix3x3;
use crate::matrix4x4::Matrix4x4;
use num_traits::{Float};
use assert_approx_eq::assert_approx_eq;

// NOTE(elsuizo:2020-03-24): nose si esta bien poner estas funciones aca...
//-------------------------------------------------------------------------
//                        auxiliar functions
//-------------------------------------------------------------------------
pub fn check_assert_matrix2x2<T: Float + std::fmt::Debug>(m1: &Matrix2x2<T>, m2: &Matrix2x2<T>) {
    for i in 0..m1.rows() {
        for j in 0..m1.cols() {
            assert_approx_eq!(m1[(i, j)], m2[(i, j)]);
        }
    }
}

pub fn check_assert_matrix3x3<T: Float + std::fmt::Debug>(m1: &Matrix3x3<T>, m2: &Matrix3x3<T>) {
    for i in 0..m1.rows() {
        for j in 0..m1.cols() {
            assert_approx_eq!(m1[(i, j)], m2[(i, j)]);
        }
    }
}

pub fn check_assert_matrix4x4<T: Float + std::fmt::Debug>(m1: &Matrix4x4<T>, m2: &Matrix4x4<T>) {
    for i in 0..m1.rows() {
        for j in 0..m1.cols() {
            assert_approx_eq!(m1[(i, j)], m2[(i, j)]);
        }
    }
}

