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
use static_math::traits::LinearAlgebra;
use static_math::matrix3x3::M33;
use static_math::utils::nearly_equal;
use num::Float;

//-------------------------------------------------------------------------
//                        auxiliar functions
//-------------------------------------------------------------------------

pub fn is_rotation<T: Float>(r: M33<T>) -> bool {
    let r2 = r * r;
    if nearly_equal(r.det(), T::one(), T::epsilon()) && nearly_equal(r2.det(), T::one(), T::epsilon()) {
        true
    } else {
        false
    }
}
