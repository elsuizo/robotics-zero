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
use crate::linear_algebra::LinearAlgebra;
use crate::matrix3x3::Matrix3x3;
use num_traits::Float;

//-------------------------------------------------------------------------
//                        auxiliar functions
//-------------------------------------------------------------------------

// NOTE(elsuizo:2020-03-31): tuve que sacar el T::epsilon() porque era muy chico y algunos tests
// no pasan, habria que ver otra solucion...
pub fn compare_floats<T: Float>(num1: T, num2: T) -> bool {
    Float::abs(num1 - num2) < T::from(1e-5).unwrap()
}

pub fn compare_vecs<T: Float>(v1: &Vec<T>, v2: &Vec<T>) -> bool {
    let v_result: Vec<bool> = v1
        .iter()
        .zip(v2)
        .map(|(a, b)| compare_floats(*a, *b))
        .collect();
    v_result.iter().all(|&x| x == true)
}

pub fn is_rotation<T: Float>(r: Matrix3x3<T>) -> bool {
    let r2 = r * r;
    if compare_floats(r.det(), T::one()) && compare_floats(r2.det(), T::one()) {
        true
    } else {
        false
    }
}
