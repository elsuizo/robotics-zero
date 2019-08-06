//-------------------------------------------------------------------------
// @file matrix2x2.rs
//
// @date 08/06/19 10:14:08
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
use std::ops::{Deref, DerefMut};
use num_traits::{One, Zero, Float};

use std::ops::{Add, Div, Mul, Sub};
use std::ops::{AddAssign, DivAssign, MulAssign, SubAssign};

#[derive(Copy, Clone, Debug, PartialEq)]
pub struct Matrix2x2<T>([[T; 2]; 2]);

impl<T: Float> Matrix2x2<T> {

    pub fn new(data_input: [[T; 2]; 2]) -> Matrix2x2<T> {
        Matrix2x2(data_input)
    }

    pub fn identity() -> Matrix2x2<T> {
        <Matrix2x2<T> as One>::one()
    }

    pub fn zeros() -> Matrix2x2<T> {
        <Matrix2x2<T> as Zero>::zero()
    }

    pub fn trace(&self) -> T {
        return self[0][0] + self[1][1];
    }

    pub fn det(&self) -> T {
        let a = self[0][0];
        let b = self[0][1];
        let c = self[1][0];
        let d = self[1][1];
        return (a * d) - (c * b)
    }

    pub fn transpose(&self) -> Matrix2x2<T> {
        Matrix2x2::new([
            [self[0][0], self[1][0], self[2][0]],
            [self[0][1], self[1][1], self[2][1]],
            [self[0][2], self[1][2], self[2][2]],
        ])
    }

    pub fn norm2(&self) -> T {
        T::sqrt(
            self[0][0] * self[0][0] + self[1][0] * self[1][0] + self[2][0] * self[2][0] +
            self[0][1] * self[0][1] + self[1][1] * self[1][1] + self[2][1] * self[2][1] +
            self[0][2] * self[0][2] + self[1][2] * self[1][2] + self[2][2] * self[2][2]
        )
    }
}

impl<T: Float> Add for Matrix2x2<T> {
    type Output = Self;

    fn add(self, rhs: Self) -> Self {
        Matrix2x2::new([
            [self[0][0] + rhs[0][0], self[0][1] + rhs[0][1], self[0][2] + rhs[0][2]],
            [self[1][0] + rhs[1][0], self[1][1] + rhs[1][1], self[1][2] + rhs[1][2]],
            [self[2][0] + rhs[2][0], self[2][1] + rhs[2][1], self[2][2] + rhs[2][2]],
        ])
    }
}

impl<T: Float> Mul for Matrix2x2<T> {
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

        Matrix2x2::new([
            [m00, m01, m02],
            [m10, m11, m12],
            [m20, m21, m22],
        ])
    }
}

impl<T: Float> Zero for Matrix2x2<T> {
    fn zero() -> Matrix2x2<T> {
        Matrix2x2::new([[T::zero(); 3]; 3])
    }

    fn is_zero(&self) -> bool {
        *self == Matrix2x2::zero()
    }
}

impl<T: Float> One for Matrix2x2<T> {
    /// Create an identity matrix
    fn one() -> Matrix2x2<T> {
        let one = T::one();
        let zero = T::zero();
        Matrix2x2::new([[one, zero, zero], [zero, one, zero], [zero, zero, one]])
    }
}
//
impl<T> Deref for Matrix2x2<T> {
    type Target = [[T; 2]; 2];
    #[inline]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}

impl<T> DerefMut for Matrix2x2<T> {
    #[inline]
    fn deref_mut(&mut self) -> &mut Self::Target {
        &mut self.0
    }
}

impl<T> From<[[T; 2]; 2]> for Matrix2x2<T> {
    fn from(data: [[T; 2]; 2]) -> Matrix2x2<T> {
        Matrix2x2(data)
    }
}
