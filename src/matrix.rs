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
use num_traits::{One, Zero};

use std::ops::{Add, Div, Mul, Sub};
use std::ops::{AddAssign, DivAssign, MulAssign, SubAssign};

#[derive(Copy, Clone, Debug, PartialEq)]
pub struct Matrix3x3([[f64; 3]; 3]);

impl Matrix3x3 {
    pub fn new(data_input: [[f64; 3]; 3]) -> Matrix3x3 {
        Matrix3x3(data_input)
    }
}

impl Add for Matrix3x3 {
    type Output = Self;

    fn add(self, rhs: Self) -> Self {
        Matrix3x3::new([
            [self[0][0] + rhs[0][0], self[0][1] + rhs[0][1], self[0][2] + rhs[0][2]],
            [self[1][0] + rhs[1][0], self[1][1] + rhs[1][1], self[1][2] + rhs[1][2]],
            [self[2][0] + rhs[2][0], self[2][1] + rhs[2][1], self[2][2] + rhs[2][2]],
        ])
    }
}

impl Mul for Matrix3x3 {
    type Output = Self;

    fn mul(self, rhs: Self) -> Self {
        let m00 = self[0][0] * rhs[0][0] + self[0][1] * rhs[1][0] + self[0][2] * rhs[2][0];
        let m01 = self[0][0] * rhs[0][1] + self[0][1] * rhs[1][1] + self[0][2] * rhs[2][1];
        let m02 = self[0][0] * rhs[0][2] + self[0][1] * rhs[1][2] + self[0][2] * rhs[2][2];

        let m10 = self[1][0] * rhs[0][0] + self[1][1] * rhs[1][0] + self[1][2] * rhs[2][0];
        let m11 = self[1][0] * rhs[0][1] + self[1][1] * rhs[1][1] + self[1][2] * rhs[2][1];
        let m12 = self[1][0] * rhs[0][2] + self[1][1] * rhs[1][2] + self[1][2] * rhs[2][2];

        let m20 = self[2][0] * rhs[0][0] + self[2][1] * rhs[1][0] + self[2][2] * rhs[2][0];
        let m21 = self[2][0] * rhs[0][1] + self[2][1] * rhs[1][1] + self[2][2] * rhs[2][1];
        let m22 = self[2][0] * rhs[0][2] + self[2][1] * rhs[1][2] + self[2][2] * rhs[2][2];

        Matrix3x3::new([
            [m00, m01, m02],
            [m10, m11, m12],
            [m20, m21, m22],
        ])
    }
}

impl Zero for Matrix3x3 {
    fn zero() -> Matrix3x3 {
        Matrix3x3::new([[0.0; 3]; 3])
    }

    fn is_zero(&self) -> bool {
        *self == Matrix3x3::zero()
    }
}

impl One for Matrix3x3 {
    /// Create an identity matrix
    fn one() -> Matrix3x3 {
        Matrix3x3::new([[1.0, 0.0, 0.0], [0.0, 1.0, 0.0], [0.0, 0.0, 1.0]])
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

