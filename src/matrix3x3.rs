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

use std::ops::{Deref, DerefMut, Index, IndexMut};
use num_traits::{One, Zero, Float};

use std::ops::{Add, Mul};
// use std::ops::{AddAssign, DivAssign, MulAssign, SubAssign};
use crate::errors::LinAlgebraError;

#[derive(Copy, Clone, Debug, PartialEq)]
pub struct Matrix3x3<T>([[T; 3]; 3]);

impl<T> Matrix3x3<T> {
    pub fn new(data_input: [[T; 3]; 3]) -> Matrix3x3<T> {
        Matrix3x3(data_input)
    }

    pub fn rows(&self) -> usize {
        self.0.len()
    }
    pub fn cols(&self) -> usize {
        self.rows()
    }
}

impl<T: Float> Matrix3x3<T> {

    pub fn identity() -> Matrix3x3<T> {
        <Matrix3x3<T> as One>::one()
    }

    pub fn zeros() -> Matrix3x3<T> {
        <Matrix3x3<T> as Zero>::zero()
    }

    pub fn trace(&self) -> T {
        return self[(0, 0)] + self[(1, 1)] + self[(2, 2)];
    }

    // TODO(elsuizo:2019-09-25): aca tendria que devolver un error
    pub fn det(&self) -> T {
        let det = self[(0, 0)] * (self[(1, 1)] * self[(2, 2)] - self[(2, 1)] * self[(1, 2)])
                  - self[(0, 1)] * (self[(1, 0)] * self[(2, 2)] - self[(1, 2)] * self[(2, 0)])
                  + self[(0, 2)] * (self[(1, 0)] * self[(2, 1)] - self[(1, 1)] * self[(2, 0)]);
        return det
    }

    pub fn transpose(&self) -> Matrix3x3<T> {
        Matrix3x3::new([
            [self[(0, 0)], self[(1, 0)], self[(2, 0)]],
            [self[(0, 1)], self[(1, 1)], self[(2, 1)]],
            [self[(0, 2)], self[(1, 2)], self[(2, 2)]],
        ])
    }

    pub fn norm2(&self) -> T {
        T::sqrt(
            self[(0, 0)] * self[(0, 0)] + self[(1, 0)] * self[(1, 0)] + self[(2, 0)] * self[(2, 0)] +
            self[(0, 1)] * self[(0, 1)] + self[(1, 1)] * self[(1, 1)] + self[(2, 1)] * self[(2, 1)] +
            self[(0, 2)] * self[(0, 2)] + self[(1, 2)] * self[(1, 2)] + self[(2, 2)] * self[(2, 2)]
        )
    }
}

impl<T: Float> Matrix3x3<T> {

    pub fn inverse(&self) -> Result<Matrix3x3<T>, LinAlgebraError> {
        let det = self.det();
        if det.abs() > T::epsilon() {
            let invdet = T::one() / det;
            let mut res = Matrix3x3::zero();
            res[(0, 0)] = (self[(1, 1)] * self[(2, 2)] - self[(2, 1)] * self[(1, 2)]) * invdet;
            res[(0, 1)] = (self[(0, 2)] * self[(2, 1)] - self[(0, 1)] * self[(2, 2)]) * invdet;
            res[(0, 2)] = (self[(0, 1)] * self[(1, 2)] - self[(0, 2)] * self[(1, 1)]) * invdet;
            res[(1, 0)] = (self[(1, 2)] * self[(2, 0)] - self[(1, 0)] * self[(2, 2)]) * invdet;
            res[(1, 1)] = (self[(0, 0)] * self[(2, 2)] - self[(0, 2)] * self[(2, 0)]) * invdet;
            res[(1, 2)] = (self[(1, 0)] * self[(0, 2)] - self[(0, 0)] * self[(1, 2)]) * invdet;
            res[(2, 0)] = (self[(1, 0)] * self[(2, 1)] - self[(2, 0)] * self[(1, 1)]) * invdet;
            res[(2, 1)] = (self[(2, 0)] * self[(0, 1)] - self[(0, 0)] * self[(2, 1)]) * invdet;
            res[(2, 2)] = (self[(0, 0)] * self[(1, 1)] - self[(1, 0)] * self[(0, 1)]) * invdet;
            Ok(res)
        } else {
            Err(LinAlgebraError::DeterminantZero)
        }
    }
}

impl<T: Float> Add for Matrix3x3<T> {
    type Output = Self;

    fn add(self, rhs: Self) -> Self {
        Matrix3x3::new([
            [self[(0, 0)] + rhs[(0, 0)], self[(0, 1)] + rhs[(0, 1)], self[(0, 2)] + rhs[(0, 2)]],
            [self[(1, 0)] + rhs[(1, 0)], self[(1, 1)] + rhs[(1, 1)], self[(1, 2)] + rhs[(1, 2)]],
            [self[(2, 0)] + rhs[(2, 0)], self[(2, 1)] + rhs[(2, 1)], self[(2, 2)] + rhs[(2, 2)]],
        ])
    }
}

impl<T: Float> Mul for Matrix3x3<T> {
    type Output = Self;

    fn mul(self, rhs: Self) -> Self {
        let m00 = self[(0, 0)] * rhs[(0, 0)] + self[(0, 1)] * rhs[(1, 0)] + self[(0, 2)] * rhs[(2, 0)];
        let m01 = self[(0, 0)] * rhs[(0, 1)] + self[(0, 1)] * rhs[(1, 1)] + self[(0, 2)] * rhs[(2, 1)];
        let m02 = self[(0, 0)] * rhs[(0, 2)] + self[(0, 1)] * rhs[(1, 2)] + self[(0, 2)] * rhs[(2, 2)];

        let m10 = self[(1, 0)] * rhs[(0, 0)] + self[(1, 1)] * rhs[(1, 0)] + self[(1, 2)] * rhs[(2, 0)];
        let m11 = self[(1, 0)] * rhs[(0, 1)] + self[(1, 1)] * rhs[(1, 1)] + self[(1, 2)] * rhs[(2, 1)];
        let m12 = self[(1, 0)] * rhs[(0, 2)] + self[(1, 1)] * rhs[(1, 2)] + self[(1, 2)] * rhs[(2, 2)];

        let m20 = self[(2, 0)] * rhs[(0, 0)] + self[(2, 1)] * rhs[(1, 0)] + self[(2, 2)] * rhs[(2, 0)];
        let m21 = self[(2, 0)] * rhs[(0, 1)] + self[(2, 1)] * rhs[(1, 1)] + self[(2, 2)] * rhs[(2, 1)];
        let m22 = self[(2, 0)] * rhs[(0, 2)] + self[(2, 1)] * rhs[(1, 2)] + self[(2, 2)] * rhs[(2, 2)];

        Matrix3x3::new([
            [m00, m01, m02],
            [m10, m11, m12],
            [m20, m21, m22],
        ])
    }
}

impl<T: Float> Zero for Matrix3x3<T> {
    fn zero() -> Matrix3x3<T> {
        Matrix3x3::new([[T::zero(); 3]; 3])
    }

    fn is_zero(&self) -> bool {
        *self == Matrix3x3::zero()
    }
}

impl<T: Float> One for Matrix3x3<T> {
    /// Create an identity matrix
    fn one() -> Matrix3x3<T> {
        let one = T::one();
        let zero = T::zero();
        Matrix3x3::new([[one, zero, zero], [zero, one, zero], [zero, zero, one]])
    }
}
//
impl<T> Deref for Matrix3x3<T> {
    type Target = [[T; 3]; 3];
    #[inline]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}

impl<T> DerefMut for Matrix3x3<T> {
    #[inline]
    fn deref_mut(&mut self) -> &mut Self::Target {
        &mut self.0
    }
}

impl<T> From<[[T; 3]; 3]> for Matrix3x3<T> {
    fn from(data: [[T; 3]; 3]) -> Matrix3x3<T> {
        Matrix3x3(data)
    }
}

impl<T> Index<(usize, usize)> for Matrix3x3<T> {
    type Output = T;
    fn index(&self, index: (usize, usize)) -> &T {
        &self.0[index.0][index.1]
    }
}

impl<T> IndexMut<(usize, usize)> for Matrix3x3<T> {
    fn index_mut(&mut self, index: (usize, usize)) -> &mut T {
            &mut self.0[index.0][index.1]
    }
}
