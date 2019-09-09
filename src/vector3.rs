//-------------------------------------------------------------------------
// @file vector3.rs
//
// @date 09/08/19 21:55:08
// @author Martin Noblia
// @email mnoblia@disroot.org
//
// @brief
// Static vector of three dimentions
// @detail
//
//  Licence:
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
//--------------------------------------------------------------------------
// imports
use std::ops::{Deref, DerefMut};
use num_traits::{One, Zero, Float};

use std::ops::{Add, Div, Mul, Sub};
use std::ops::{AddAssign, DivAssign, MulAssign, SubAssign};

use crate::matrix3x3::*;

//-------------------------------------------------------------------------
//                        code
//-------------------------------------------------------------------------
pub struct Vector3<T>([T; 3]);

impl<T> Vector3<T> {
    pub fn new(input: [T; 3]) -> Vector3<T> {
        Vector3(input)
    }
}
