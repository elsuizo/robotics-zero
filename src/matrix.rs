//-------------------------------------------------------------------------
// @file matrix.rs
//
// @date 06/14/19 09:56:01
// @author Martin Noblia
// @email mnoblia@disroot.org
//
// @brief
// Matrix types implementations
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

use std::ops::{Deref, DerefMut};

#[derive(Copy, Clone, Debug, PartialEq)]
pub struct Matrix3x3([[f64; 3]; 3]);

impl Matrix3x3 {
    pub fn new(data_input: [[f64; 3]; 3]) -> Matrix3x3 {
        Matrix3x3(data_input)
    }
}

impl Deref for Matrix3x3 {
    type Target = [[f64; 3]; 3];
    #[inline]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}

impl DerefMut for Matrix3x3 {
    #[inline]
    fn deref_mut(&mut self) -> &mut Self::Target {
        &mut self.0
    }
}

impl From<[[f64; 3]; 3]> for Matrix3x3 {
    fn from(data: [[f64; 3]; 3]) -> Matrix3x3 {
        Matrix3x3(data)
    }
}

