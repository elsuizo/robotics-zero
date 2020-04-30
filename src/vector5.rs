//-------------------------------------------------------------------------
// @file vector5.rs
//
// @date 04/27/20 11:34:07
// @author Martin Noblia
// @email mnoblia@disroot.org
//
// @brief
//
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
use num_traits::{Float, Zero};
use std::ops::{Deref, DerefMut};

use std::ops::{Add, Mul};

use crate::matrix5x5::Matrix5x5;
//-------------------------------------------------------------------------
//                        code
//-------------------------------------------------------------------------
#[derive(Copy, Clone, Debug, PartialEq)]
pub struct Vector5<T>([T; 5]);

impl<T> Vector5<T> {
    pub fn new(input: [T; 5]) -> Self {
        Self(input)
    }
}

impl<T: Float> Vector5<T> {
    pub fn zeros() -> Self {
        <Vector5<T> as Zero>::zero()
    }

    pub fn norm2(&self) -> T {
        let a = self[0];
        let b = self[1];
        let c = self[2];
        let d = self[3];
        let e = self[4];
        T::sqrt(a * a + b * b + c * c + d * d + e * e)
    }
}

impl<T: Float> Mul for Vector5<T> {
    type Output = T;

    fn mul(self, rhs: Self) -> T {
        let a0 = self[0];
        let a1 = self[1];
        let a2 = self[2];
        let a3 = self[3];
        let a4 = self[4];

        let b0 = rhs[0];
        let b1 = rhs[1];
        let b2 = rhs[2];
        let b3 = rhs[3];
        let b4 = rhs[4];

        a0 * b0 + a1 * b1 + a2 * b2 + a3 * b3 + a4 * b4
    }
}

// TODO(elsuizo:2020-04-22): faltaria Matrix5x5 * Vector5
impl<T: Float> Mul<Matrix5x5<T>> for Vector5<T> {
    type Output = Vector5<T>;

    fn mul(self, rhs: Matrix5x5<T>) -> Vector5<T> {
        let v0 = self[0];
        let v1 = self[1];
        let v2 = self[2];
        let v3 = self[3];
        let v4 = self[4];

        let a_00 = rhs[(0, 0)];
        let a_01 = rhs[(0, 1)];
        let a_02 = rhs[(0, 2)];
        let a_03 = rhs[(0, 3)];
        let a_04 = rhs[(0, 4)];
        let a_10 = rhs[(1, 0)];
        let a_11 = rhs[(1, 1)];
        let a_12 = rhs[(1, 2)];
        let a_13 = rhs[(1, 3)];
        let a_14 = rhs[(1, 4)];
        let a_20 = rhs[(2, 0)];
        let a_21 = rhs[(2, 1)];
        let a_22 = rhs[(2, 2)];
        let a_23 = rhs[(2, 3)];
        let a_24 = rhs[(2, 4)];
        let a_30 = rhs[(3, 0)];
        let a_31 = rhs[(3, 1)];
        let a_32 = rhs[(3, 2)];
        let a_33 = rhs[(3, 3)];
        let a_34 = rhs[(3, 4)];
        let a_40 = rhs[(4, 0)];
        let a_41 = rhs[(4, 1)];
        let a_42 = rhs[(4, 2)];
        let a_43 = rhs[(4, 3)];
        let a_44 = rhs[(4, 4)];

        Vector5::new([
            a_00 * v0 + a_10 * v1 + a_20 * v2 + a_30 * v3 + a_40 * v4,
            a_01 * v0 + a_11 * v1 + a_21 * v2 + a_31 * v3 + a_41 * v4,
            a_02 * v0 + a_12 * v1 + a_22 * v2 + a_32 * v3 + a_42 * v4,
            a_03 * v0 + a_13 * v1 + a_23 * v2 + a_33 * v3 + a_43 * v4,
            a_04 * v0 + a_14 * v1 + a_24 * v2 + a_34 * v3 + a_44 * v4,
        ])
    }
}

impl<T: Float> Add for Vector5<T> {
    type Output = Self;
    fn add(self, rhs: Self) -> Self {
        let v0 = self[0];
        let v1 = self[1];
        let v2 = self[2];
        let v3 = self[3];
        let v4 = self[4];

        let a0 = rhs[0];
        let a1 = rhs[1];
        let a2 = rhs[2];
        let a3 = rhs[3];
        let a4 = rhs[4];

        Vector5::new([v0 + a0, v1 + a1, v2 + a2, v3 + a3, v4 + a4])
    }
}

impl<T: Float> Zero for Vector5<T> {
    fn zero() -> Vector5<T> {
        Vector5::new([T::zero(); 5])
    }

    fn is_zero(&self) -> bool {
        *self == Vector5::zero()
    }
}

impl<T> Deref for Vector5<T> {
    type Target = [T; 5];
    #[inline]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}

impl<T> DerefMut for Vector5<T> {
    #[inline]
    fn deref_mut(&mut self) -> &mut Self::Target {
        &mut self.0
    }
}

impl<T> From<[T; 5]> for Vector5<T> {
    fn from(data: [T; 5]) -> Vector5<T> {
        Vector5(data)
    }
}
