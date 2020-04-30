// -------------------------------------------------------------------------
// @file vector4.rs
//
// @date 09/13/19 20:36:22
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
// imports
use num_traits::{Float, Zero};
use std::ops::{Deref, DerefMut};

use std::ops::{Add, Mul};

use crate::matrix4x4::Matrix4x4;

//-------------------------------------------------------------------------
//                        code
//-------------------------------------------------------------------------
#[derive(Copy, Clone, Debug, PartialEq)]
pub struct Vector4<T>([T; 4]);

impl<T> Vector4<T> {
    pub fn new(input: [T; 4]) -> Vector4<T> {
        Vector4(input)
    }
}

impl<T: Float> Vector4<T> {
    pub fn zeros() -> Vector4<T> {
        <Vector4<T> as Zero>::zero()
    }

    pub fn norm2(&self) -> T {
        let a = self[0];
        let b = self[1];
        let c = self[2];
        let d = self[3];
        T::sqrt(a * a + b * b + c * c + d * d)
    }
}

impl<T: Float> Mul for Vector4<T> {
    type Output = T;

    fn mul(self, rhs: Self) -> T {
        let a1 = self[0];
        let a2 = self[1];
        let a3 = self[2];
        let a4 = self[3];

        let b1 = rhs[0];
        let b2 = rhs[1];
        let b3 = rhs[2];
        let b4 = rhs[3];

        a1 * b1 + a2 * b2 + a3 * b3 + a4 * b4
    }
}

impl<T: Float> Mul<Matrix4x4<T>> for Vector4<T> {
    type Output = Vector4<T>;

    fn mul(self, rhs: Matrix4x4<T>) -> Vector4<T> {
        let a11 = rhs[(0, 0)];
        let a12 = rhs[(0, 1)];
        let a13 = rhs[(0, 2)];
        let a14 = rhs[(0, 3)];
        let a21 = rhs[(1, 0)];
        let a22 = rhs[(1, 1)];
        let a23 = rhs[(1, 2)];
        let a24 = rhs[(1, 3)];
        let a31 = rhs[(2, 0)];
        let a32 = rhs[(2, 1)];
        let a33 = rhs[(2, 2)];
        let a34 = rhs[(2, 3)];
        let a41 = rhs[(3, 0)];
        let a42 = rhs[(3, 1)];
        let a43 = rhs[(3, 2)];
        let a44 = rhs[(3, 3)];

        let v1 = self[0];
        let v2 = self[1];
        let v3 = self[2];
        let v4 = self[3];

        Vector4::new([
            v1 * a11 + v2 * a21 + v3 * a31 + v4 * a41,
            v1 * a12 + v2 * a22 + v3 * a32 + v4 * a42,
            v1 * a13 + v2 * a23 + v3 * a33 + v4 * a43,
            v1 * a14 + v2 * a24 + v3 * a34 + v4 * a44,
        ])
    }
}

impl<T: Float> Add for Vector4<T> {
    type Output = Self;

    fn add(self, rhs: Self) -> Self {
        let v1 = self[0];
        let v2 = self[1];
        let v3 = self[2];
        let v4 = self[3];

        let a1 = rhs[0];
        let a2 = rhs[1];
        let a3 = rhs[2];
        let a4 = rhs[3];

        Vector4::new([v1 + a1, v2 + a2, v3 + a3, v4 + a4])
    }
}

impl<T: Float> Zero for Vector4<T> {
    fn zero() -> Vector4<T> {
        Vector4::new([T::zero(); 4])
    }

    fn is_zero(&self) -> bool {
        *self == Vector4::zero()
    }
}

impl<T> Deref for Vector4<T> {
    type Target = [T; 4];
    #[inline]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}

impl<T> DerefMut for Vector4<T> {
    #[inline]
    fn deref_mut(&mut self) -> &mut Self::Target {
        &mut self.0
    }
}

impl<T> From<[T; 4]> for Vector4<T> {
    fn from(data: [T; 4]) -> Vector4<T> {
        Vector4(data)
    }
}
