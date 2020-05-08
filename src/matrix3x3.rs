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

use std::fmt;
use std::ops::{Add, Mul};
use std::ops::{Deref, DerefMut, Index, IndexMut};

use crate::errors::LinAlgebraError;
use crate::linear_algebra::LinearAlgebra;
use num_traits::{Float, One, Zero, Num};

//-------------------------------------------------------------------------
//                        code
//-------------------------------------------------------------------------

#[derive(Copy, Clone, Debug, PartialEq)]
pub struct Matrix3x3<T>([[T; 3]; 3]);

impl<T: Float> LinearAlgebra<T> for Matrix3x3<T> {
    fn rows(&self) -> usize {
        self.0.len()
    }

    fn cols(&self) -> usize {
        self.rows()
    }

    fn transpose(&self) -> Matrix3x3<T> {
        Matrix3x3::new([
            [self[(0, 0)], self[(1, 0)], self[(2, 0)]],
            [self[(0, 1)], self[(1, 1)], self[(2, 1)]],
            [self[(0, 2)], self[(1, 2)], self[(2, 2)]],
        ])
    }

    fn trace(&self) -> T {
        return self[(0, 0)] + self[(1, 1)] + self[(2, 2)];
    }

    fn norm2(&self) -> T {
        T::sqrt(
            self[(0, 0)] * self[(0, 0)]
                + self[(1, 0)] * self[(1, 0)]
                + self[(2, 0)] * self[(2, 0)]
                + self[(0, 1)] * self[(0, 1)]
                + self[(1, 1)] * self[(1, 1)]
                + self[(2, 1)] * self[(2, 1)]
                + self[(0, 2)] * self[(0, 2)]
                + self[(1, 2)] * self[(1, 2)]
                + self[(2, 2)] * self[(2, 2)],
        )
    }

    fn det(&self) -> T {
        self[(0, 0)] * (self[(1, 1)] * self[(2, 2)] - self[(2, 1)] * self[(1, 2)])
            - self[(0, 1)] * (self[(1, 0)] * self[(2, 2)] - self[(1, 2)] * self[(2, 0)])
            + self[(0, 2)] * (self[(1, 0)] * self[(2, 1)] - self[(1, 1)] * self[(2, 0)])
    }

    fn inverse(&self) -> Result<Matrix3x3<T>, LinAlgebraError> {
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

impl<T: Num + Copy> Matrix3x3<T> {
    pub fn identity() -> Matrix3x3<T> {
        <Matrix3x3<T> as One>::one()
    }

    pub fn zeros() -> Matrix3x3<T> {
        <Matrix3x3<T> as Zero>::zero()
    }

    pub fn as_vec(&self) -> Vec<T> {
        let result: Vec<T> = self.iter().flatten().cloned().collect();
        return result;
    }
}

impl<T: Num + Copy> Add for Matrix3x3<T> {
    type Output = Self;

    fn add(self, rhs: Self) -> Self {
        Matrix3x3::new([
            [
                self[(0, 0)] + rhs[(0, 0)],
                self[(0, 1)] + rhs[(0, 1)],
                self[(0, 2)] + rhs[(0, 2)],
            ],
            [
                self[(1, 0)] + rhs[(1, 0)],
                self[(1, 1)] + rhs[(1, 1)],
                self[(1, 2)] + rhs[(1, 2)],
            ],
            [
                self[(2, 0)] + rhs[(2, 0)],
                self[(2, 1)] + rhs[(2, 1)],
                self[(2, 2)] + rhs[(2, 2)],
            ],
        ])
    }
}

impl<T: Num + Copy> Mul<T> for Matrix3x3<T> {
    type Output = Matrix3x3<T>;

    fn mul(self, rhs: T) -> Self::Output {
        let a_00 = self[(0, 0)] * rhs;
        let a_01 = self[(0, 1)] * rhs;
        let a_02 = self[(0, 2)] * rhs;
        let a_10 = self[(1, 0)] * rhs;
        let a_11 = self[(1, 1)] * rhs;
        let a_12 = self[(1, 2)] * rhs;
        let a_20 = self[(2, 0)] * rhs;
        let a_21 = self[(2, 1)] * rhs;
        let a_22 = self[(2, 2)] * rhs;

        Matrix3x3::new([[a_00, a_01, a_02], [a_10, a_11, a_12], [a_20, a_21, a_22]])
    }
}

impl<T: Num + Copy> Mul for Matrix3x3<T> {
    type Output = Self;

    fn mul(self, rhs: Self) -> Self {
        let m00 =
            self[(0, 0)] * rhs[(0, 0)] + self[(0, 1)] * rhs[(1, 0)] + self[(0, 2)] * rhs[(2, 0)];
        let m01 =
            self[(0, 0)] * rhs[(0, 1)] + self[(0, 1)] * rhs[(1, 1)] + self[(0, 2)] * rhs[(2, 1)];
        let m02 =
            self[(0, 0)] * rhs[(0, 2)] + self[(0, 1)] * rhs[(1, 2)] + self[(0, 2)] * rhs[(2, 2)];

        let m10 =
            self[(1, 0)] * rhs[(0, 0)] + self[(1, 1)] * rhs[(1, 0)] + self[(1, 2)] * rhs[(2, 0)];
        let m11 =
            self[(1, 0)] * rhs[(0, 1)] + self[(1, 1)] * rhs[(1, 1)] + self[(1, 2)] * rhs[(2, 1)];
        let m12 =
            self[(1, 0)] * rhs[(0, 2)] + self[(1, 1)] * rhs[(1, 2)] + self[(1, 2)] * rhs[(2, 2)];

        let m20 =
            self[(2, 0)] * rhs[(0, 0)] + self[(2, 1)] * rhs[(1, 0)] + self[(2, 2)] * rhs[(2, 0)];
        let m21 =
            self[(2, 0)] * rhs[(0, 1)] + self[(2, 1)] * rhs[(1, 1)] + self[(2, 2)] * rhs[(2, 1)];
        let m22 =
            self[(2, 0)] * rhs[(0, 2)] + self[(2, 1)] * rhs[(1, 2)] + self[(2, 2)] * rhs[(2, 2)];

        Matrix3x3::new([[m00, m01, m02], [m10, m11, m12], [m20, m21, m22]])
    }
}

impl<T: Num + Copy> Zero for Matrix3x3<T> {
    fn zero() -> Matrix3x3<T> {
        Matrix3x3::new([[T::zero(); 3]; 3])
    }

    fn is_zero(&self) -> bool {
        *self == Matrix3x3::zero()
    }
}

impl<T: Num + Copy> One for Matrix3x3<T> {
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

//-------------------------------------------------------------------------
//                        Display for Matrix3x3
//-------------------------------------------------------------------------
impl<T: Num + fmt::Display> fmt::Display for Matrix3x3<T> {
    fn fmt(&self, dest: &mut fmt::Formatter) -> fmt::Result {
        println!("");
        write!(
            dest,
            "|{0:<7.2} {1:^7.2} {2:>7.2}|\n",
            self[(0, 0)],
            self[(0, 1)],
            self[(0, 2)]
        )?;
        write!(
            dest,
            "|{0:<7.2} {1:^7.2} {2:>7.2}|\n",
            self[(1, 0)],
            self[(1, 1)],
            self[(1, 2)]
        )?;
        write!(
            dest,
            "|{0:<7.2} {1:^7.2} {2:>7.2}|\n",
            self[(2, 0)],
            self[(2, 1)],
            self[(2, 2)]
        )
    }
}
