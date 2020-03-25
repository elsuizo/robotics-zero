//-------------------------------------------------------------------------
// @file vector2.rs
//
// @date 08/13/19 10:33:16
// @author Martin Noblia
// @email mnoblia@disroot.org
//
// @brief
// Static Vector of two dimentions
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
use num_traits::{Zero, Float};

use std::ops::{Add, Mul};
// use std::ops::{AddAssign, DivAssign, MulAssign, SubAssign};

use crate::matrix2x2::*;

//-------------------------------------------------------------------------
//                        code
//-------------------------------------------------------------------------
#[derive(Copy, Clone, Debug, PartialEq)]
pub struct Vector2<T>([T; 2]);

// NOTE(elsuizo:2019-09-08): podemos crear cualquier vector de cualquier type T
impl<T> Vector2<T> {
    pub fn new(input: [T; 2]) -> Vector2<T> {
        Vector2(input)
    }
}

impl<T: Float> Vector2<T> {

    pub fn zeros() -> Vector2<T> {
        <Vector2<T> as Zero>::zero()
    }

    pub fn norm2(&self) -> T {
        let a = self[0];
        let b = self[1];

        T::sqrt(a * a + b * b)
    }
}

impl<T: Float> Mul for Vector2<T> {
    type Output = T;

    fn mul(self, rhs: Self) -> T {
        let a1 = self[0];
        let a2 = self[1];

        let b1 = rhs[0];
        let b2 = rhs[1];

        a1 * b1 + a2 * b2
    }
}

impl<T: Float> Mul<Matrix2x2<T>> for Vector2<T> {
    type Output = Vector2<T>;

    fn mul(self, rhs: Matrix2x2<T>) -> Vector2<T> {
        let a1 = rhs[(0, 0)];
        let b1 = rhs[(0, 1)];
        let c1 = rhs[(1, 0)];
        let d1 = rhs[(1, 1)];

        let v1 = self[0];
        let v2 = self[1];
        Vector2::new([v1 * a1 + v2 * c1, v1 * b1 + v2 * d1])
    }
}

impl<T: Float> Add for Vector2<T> {
    type Output = Self;

    fn add(self, rhs: Self) -> Self {
        let a1 = self[0];
        let a2 = self[1];

        let b1 = rhs[0];
        let b2 = rhs[1];

        Vector2::new([a1 + b1, a2 + b2])
    }
}

impl<T: Float> Zero for Vector2<T> {
    fn zero() -> Vector2<T> {
        Vector2::new([T::zero(); 2])
    }

    fn is_zero(&self) -> bool {
        *self == Vector2::zero()
    }

}

impl<T> Deref for Vector2<T> {
    type Target = [T; 2];
    #[inline]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}

impl<T> DerefMut for Vector2<T> {
    #[inline]
    fn deref_mut(&mut self) -> &mut Self::Target {
        &mut self.0
    }
}

impl<T> From<[T; 2]> for Vector2<T> {
    fn from(data: [T; 2]) -> Vector2<T> {
        Vector2(data)
    }
}
