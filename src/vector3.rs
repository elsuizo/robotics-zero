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
use num_traits::{Float, Zero};
use std::ops::{Deref, DerefMut};

use std::ops::{Add, Mul};
// use std::ops::{AddAssign, DivAssign, MulAssign, SubAssign};

use crate::matrix3x3::Matrix3x3;
//-------------------------------------------------------------------------
//                        code
//-------------------------------------------------------------------------
#[derive(Copy, Clone, Debug, PartialEq)]
pub struct Vector3<T>([T; 3]);

impl<T> Vector3<T> {
    pub fn new(input: [T; 3]) -> Vector3<T> {
        Vector3(input)
    }
}

impl<T: Float> Vector3<T> {
    pub fn zeros() -> Vector3<T> {
        <Vector3<T> as Zero>::zero()
    }

    pub fn norm2(&self) -> T {
        let a = self[0];
        let b = self[1];
        let c = self[2];
        T::sqrt(a * a + b * b + c * c)
    }
}

impl<T: Float> Mul for Vector3<T> {
    type Output = T;

    fn mul(self, rhs: Self) -> T {
        let a1 = self[0];
        let a2 = self[1];
        let a3 = self[2];

        let b1 = rhs[0];
        let b2 = rhs[1];
        let b3 = rhs[2];

        a1 * b1 + a2 * b2 + a3 * b3
    }
}

impl<T: Float> Mul<Matrix3x3<T>> for Vector3<T> {
    type Output = Vector3<T>;

    fn mul(self, rhs: Matrix3x3<T>) -> Vector3<T> {
        let a11 = rhs[(0, 0)];
        let a12 = rhs[(0, 1)];
        let a13 = rhs[(0, 2)];
        let a21 = rhs[(1, 0)];
        let a22 = rhs[(1, 1)];
        let a23 = rhs[(1, 2)];
        let a31 = rhs[(2, 0)];
        let a32 = rhs[(2, 1)];
        let a33 = rhs[(2, 2)];

        let v1 = self[0];
        let v2 = self[1];
        let v3 = self[2];

        Vector3::new([
            a11 * v1 + a12 * v2 + a13 * v3,
            a21 * v1 + a22 * v2 + a23 * v3,
            a31 * v1 + a32 * v2 + a33 * v3,
        ])
    }
}

impl<T: Float> Add for Vector3<T> {
    type Output = Self;

    fn add(self, rhs: Self) -> Self {
        let v1 = self[0];
        let v2 = self[1];
        let v3 = self[2];

        let a1 = rhs[0];
        let a2 = rhs[1];
        let a3 = rhs[2];

        Vector3::new([v1 + a1, v2 + a2, v3 + a3])
    }
}

impl<T: Float> Zero for Vector3<T> {
    fn zero() -> Vector3<T> {
        Vector3::new([T::zero(); 3])
    }

    fn is_zero(&self) -> bool {
        *self == Vector3::zero()
    }
}

impl<T> Deref for Vector3<T> {
    type Target = [T; 3];
    #[inline]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}

impl<T> DerefMut for Vector3<T> {
    #[inline]
    fn deref_mut(&mut self) -> &mut Self::Target {
        &mut self.0
    }
}

impl<T> From<[T; 3]> for Vector3<T> {
    fn from(data: [T; 3]) -> Vector3<T> {
        Vector3(data)
    }
}
