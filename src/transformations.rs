// -------------------------------------------------------------------------
// @file transformations.rs
//
// @date 03/25/20 15:04:27
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

use num_traits::{One, Zero, Float};
use crate::matrix3x3::Matrix3x3;
/// brief.
///
/// compute the rotation around the `x` axis(in cartesian coordinates)
///
/// description
///
/// * `angle` - angle of rotation in degrees
///
pub fn rotx<T: Float>(angle: T) -> Matrix3x3<T> {
    let one = T::one();
    let zero = T::zero();
    let c = angle.to_radians().cos();
    let s = angle.to_radians().sin();
    Matrix3x3::new([[one, zero, zero],
                    [zero, c, -s],
                    [zero, s,  c]])

