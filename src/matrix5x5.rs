//-------------------------------------------------------------------------
// @file matrix5x5.rs
//
// @date 04/25/20 22:49:51
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
use num_traits::{Float, One, Zero};
use std::fmt;
use std::ops::{Add, Mul};
use std::ops::{Deref, DerefMut, Index, IndexMut};

use crate::errors::LinAlgebraError;
use crate::linear_algebra::LinearAlgebra;
use crate::matrix4x4::Matrix4x4;

//-------------------------------------------------------------------------
//                        code
//-------------------------------------------------------------------------

#[derive(Copy, Clone, Debug, PartialEq)]
pub struct Matrix5x5<T>([[T; 5]; 5]);

impl<T: Float + std::iter::Sum> LinearAlgebra<T> for Matrix5x5<T> {
    fn rows(&self) -> usize {
        self.0.len()
    }

    fn cols(&self) -> usize {
        self.rows()
    }

    fn transpose(&self) -> Self {
        let a_00 = self[(0, 0)];
        let a_01 = self[(0, 1)];
        let a_02 = self[(0, 2)];
        let a_03 = self[(0, 3)];
        let a_04 = self[(0, 4)];
        let a_10 = self[(1, 0)];
        let a_11 = self[(1, 1)];
        let a_12 = self[(1, 2)];
        let a_13 = self[(1, 3)];
        let a_14 = self[(1, 4)];
        let a_20 = self[(2, 0)];
        let a_21 = self[(2, 1)];
        let a_22 = self[(2, 2)];
        let a_23 = self[(2, 3)];
        let a_24 = self[(2, 4)];
        let a_30 = self[(3, 0)];
        let a_31 = self[(3, 1)];
        let a_32 = self[(3, 2)];
        let a_33 = self[(3, 3)];
        let a_34 = self[(3, 4)];
        let a_40 = self[(4, 0)];
        let a_41 = self[(4, 1)];
        let a_42 = self[(4, 2)];
        let a_43 = self[(4, 3)];
        let a_44 = self[(4, 4)];

        Matrix5x5::new([
            [a_00, a_10, a_20, a_30, a_40],
            [a_01, a_11, a_21, a_31, a_41],
            [a_02, a_12, a_22, a_32, a_42],
            [a_03, a_13, a_23, a_33, a_43],
            [a_04, a_14, a_24, a_34, a_44],
        ])
    }

    fn trace(&self) -> T {
        self[(0, 0)] + self[(1, 1)] + self[(2, 2)] + self[(3, 3)] + self[(4, 4)]
    }

    fn norm2(&self) -> T {
        T::sqrt(
            self.iter()
                .flatten()
                .cloned()
                .map(|element| element * element)
                .sum(),
        )
    }

    fn det(&self) -> T {
        let a_00 = self[(0, 0)];
        let a_01 = self[(0, 1)];
        let a_02 = self[(0, 2)];
        let a_03 = self[(0, 3)];
        let a_04 = self[(0, 4)];
        let a_10 = self[(1, 0)];
        let a_11 = self[(1, 1)];
        let a_12 = self[(1, 2)];
        let a_13 = self[(1, 3)];
        let a_14 = self[(1, 4)];
        let a_20 = self[(2, 0)];
        let a_21 = self[(2, 1)];
        let a_22 = self[(2, 2)];
        let a_23 = self[(2, 3)];
        let a_24 = self[(2, 4)];
        let a_30 = self[(3, 0)];
        let a_31 = self[(3, 1)];
        let a_32 = self[(3, 2)];
        let a_33 = self[(3, 3)];
        let a_34 = self[(3, 4)];
        let a_40 = self[(4, 0)];
        let a_41 = self[(4, 1)];
        let a_42 = self[(4, 2)];
        let a_43 = self[(4, 3)];
        let a_44 = self[(4, 4)];

        a_00 * a_11 * a_22 * a_33 * a_44
            - a_00 * a_11 * a_22 * a_34 * a_43
            - a_00 * a_11 * a_23 * a_32 * a_44
            + a_00 * a_11 * a_23 * a_34 * a_42
            + a_00 * a_11 * a_24 * a_32 * a_43
            - a_00 * a_11 * a_24 * a_33 * a_42
            - a_00 * a_12 * a_21 * a_33 * a_44
            + a_00 * a_12 * a_21 * a_34 * a_43
            + a_00 * a_12 * a_23 * a_31 * a_44
            - a_00 * a_12 * a_23 * a_34 * a_41
            - a_00 * a_12 * a_24 * a_31 * a_43
            + a_00 * a_12 * a_24 * a_33 * a_41
            + a_00 * a_13 * a_21 * a_32 * a_44
            - a_00 * a_13 * a_21 * a_34 * a_42
            - a_00 * a_13 * a_22 * a_31 * a_44
            + a_00 * a_13 * a_22 * a_34 * a_41
            + a_00 * a_13 * a_24 * a_31 * a_42
            - a_00 * a_13 * a_24 * a_32 * a_41
            - a_00 * a_14 * a_21 * a_32 * a_43
            + a_00 * a_14 * a_21 * a_33 * a_42
            + a_00 * a_14 * a_22 * a_31 * a_43
            - a_00 * a_14 * a_22 * a_33 * a_41
            - a_00 * a_14 * a_23 * a_31 * a_42
            + a_00 * a_14 * a_23 * a_32 * a_41
            - a_01 * a_10 * a_22 * a_33 * a_44
            + a_01 * a_10 * a_22 * a_34 * a_43
            + a_01 * a_10 * a_23 * a_32 * a_44
            - a_01 * a_10 * a_23 * a_34 * a_42
            - a_01 * a_10 * a_24 * a_32 * a_43
            + a_01 * a_10 * a_24 * a_33 * a_42
            + a_01 * a_12 * a_20 * a_33 * a_44
            - a_01 * a_12 * a_20 * a_34 * a_43
            - a_01 * a_12 * a_23 * a_30 * a_44
            + a_01 * a_12 * a_23 * a_34 * a_40
            + a_01 * a_12 * a_24 * a_30 * a_43
            - a_01 * a_12 * a_24 * a_33 * a_40
            - a_01 * a_13 * a_20 * a_32 * a_44
            + a_01 * a_13 * a_20 * a_34 * a_42
            + a_01 * a_13 * a_22 * a_30 * a_44
            - a_01 * a_13 * a_22 * a_34 * a_40
            - a_01 * a_13 * a_24 * a_30 * a_42
            + a_01 * a_13 * a_24 * a_32 * a_40
            + a_01 * a_14 * a_20 * a_32 * a_43
            - a_01 * a_14 * a_20 * a_33 * a_42
            - a_01 * a_14 * a_22 * a_30 * a_43
            + a_01 * a_14 * a_22 * a_33 * a_40
            + a_01 * a_14 * a_23 * a_30 * a_42
            - a_01 * a_14 * a_23 * a_32 * a_40
            + a_02 * a_10 * a_21 * a_33 * a_44
            - a_02 * a_10 * a_21 * a_34 * a_43
            - a_02 * a_10 * a_23 * a_31 * a_44
            + a_02 * a_10 * a_23 * a_34 * a_41
            + a_02 * a_10 * a_24 * a_31 * a_43
            - a_02 * a_10 * a_24 * a_33 * a_41
            - a_02 * a_11 * a_20 * a_33 * a_44
            + a_02 * a_11 * a_20 * a_34 * a_43
            + a_02 * a_11 * a_23 * a_30 * a_44
            - a_02 * a_11 * a_23 * a_34 * a_40
            - a_02 * a_11 * a_24 * a_30 * a_43
            + a_02 * a_11 * a_24 * a_33 * a_40
            + a_02 * a_13 * a_20 * a_31 * a_44
            - a_02 * a_13 * a_20 * a_34 * a_41
            - a_02 * a_13 * a_21 * a_30 * a_44
            + a_02 * a_13 * a_21 * a_34 * a_40
            + a_02 * a_13 * a_24 * a_30 * a_41
            - a_02 * a_13 * a_24 * a_31 * a_40
            - a_02 * a_14 * a_20 * a_31 * a_43
            + a_02 * a_14 * a_20 * a_33 * a_41
            + a_02 * a_14 * a_21 * a_30 * a_43
            - a_02 * a_14 * a_21 * a_33 * a_40
            - a_02 * a_14 * a_23 * a_30 * a_41
            + a_02 * a_14 * a_23 * a_31 * a_40
            - a_03 * a_10 * a_21 * a_32 * a_44
            + a_03 * a_10 * a_21 * a_34 * a_42
            + a_03 * a_10 * a_22 * a_31 * a_44
            - a_03 * a_10 * a_22 * a_34 * a_41
            - a_03 * a_10 * a_24 * a_31 * a_42
            + a_03 * a_10 * a_24 * a_32 * a_41
            + a_03 * a_11 * a_20 * a_32 * a_44
            - a_03 * a_11 * a_20 * a_34 * a_42
            - a_03 * a_11 * a_22 * a_30 * a_44
            + a_03 * a_11 * a_22 * a_34 * a_40
            + a_03 * a_11 * a_24 * a_30 * a_42
            - a_03 * a_11 * a_24 * a_32 * a_40
            - a_03 * a_12 * a_20 * a_31 * a_44
            + a_03 * a_12 * a_20 * a_34 * a_41
            + a_03 * a_12 * a_21 * a_30 * a_44
            - a_03 * a_12 * a_21 * a_34 * a_40
            - a_03 * a_12 * a_24 * a_30 * a_41
            + a_03 * a_12 * a_24 * a_31 * a_40
            + a_03 * a_14 * a_20 * a_31 * a_42
            - a_03 * a_14 * a_20 * a_32 * a_41
            - a_03 * a_14 * a_21 * a_30 * a_42
            + a_03 * a_14 * a_21 * a_32 * a_40
            + a_03 * a_14 * a_22 * a_30 * a_41
            - a_03 * a_14 * a_22 * a_31 * a_40
            + a_04 * a_10 * a_21 * a_32 * a_43
            - a_04 * a_10 * a_21 * a_33 * a_42
            - a_04 * a_10 * a_22 * a_31 * a_43
            + a_04 * a_10 * a_22 * a_33 * a_41
            + a_04 * a_10 * a_23 * a_31 * a_42
            - a_04 * a_10 * a_23 * a_32 * a_41
            - a_04 * a_11 * a_20 * a_32 * a_43
            + a_04 * a_11 * a_20 * a_33 * a_42
            + a_04 * a_11 * a_22 * a_30 * a_43
            - a_04 * a_11 * a_22 * a_33 * a_40
            - a_04 * a_11 * a_23 * a_30 * a_42
            + a_04 * a_11 * a_23 * a_32 * a_40
            + a_04 * a_12 * a_20 * a_31 * a_43
            - a_04 * a_12 * a_20 * a_33 * a_41
            - a_04 * a_12 * a_21 * a_30 * a_43
            + a_04 * a_12 * a_21 * a_33 * a_40
            + a_04 * a_12 * a_23 * a_30 * a_41
            - a_04 * a_12 * a_23 * a_31 * a_40
            - a_04 * a_13 * a_20 * a_31 * a_42
            + a_04 * a_13 * a_20 * a_32 * a_41
            + a_04 * a_13 * a_21 * a_30 * a_42
            - a_04 * a_13 * a_21 * a_32 * a_40
            - a_04 * a_13 * a_22 * a_30 * a_41
            + a_04 * a_13 * a_22 * a_31 * a_40
    }
    ///
    /// Calculate the inverse of the Matrix6x6 via tha Adjoint Matrix:
    /// A^(-1) = 1/det Adj
    /// where Adj = Cofactor.Transpose()
    /// Cofactor = (-1)^(i+j) M(i, j).det()
    ///
    fn inverse(&self) -> Result<Matrix5x5<T>, LinAlgebraError> {
        let det = self.det();
        if det.abs() > T::epsilon() {
            let mut cofactors: Matrix5x5<T> = Matrix5x5::zeros();
            for i in 0..self.rows() {
                for j in 0..self.cols() {
                    let sign = (-T::one()).powi((i + j) as i32);
                    cofactors[(i, j)] = sign * self.get_submatrix((i, j)).det();
                }
            }
            Ok(cofactors.transpose() * (T::one() / det))
        } else {
            Err(LinAlgebraError::DeterminantZero)
        }
    }
}
impl<T> Matrix5x5<T> {
    pub fn new(data_input: [[T; 5]; 5]) -> Self {
        Self(data_input)
    }
    pub fn rows(&self) -> usize {
        self.0.len()
    }
    pub fn cols(&self) -> usize {
        self.rows()
    }
}

impl<T: Float> Add for Matrix5x5<T> {
    type Output = Self;

    fn add(self, rhs: Self) -> Self {
        let a_00 = self[(0, 0)];
        let a_01 = self[(0, 1)];
        let a_02 = self[(0, 2)];
        let a_03 = self[(0, 3)];
        let a_04 = self[(0, 4)];
        let a_10 = self[(1, 0)];
        let a_11 = self[(1, 1)];
        let a_12 = self[(1, 2)];
        let a_13 = self[(1, 3)];
        let a_14 = self[(1, 4)];
        let a_20 = self[(2, 0)];
        let a_21 = self[(2, 1)];
        let a_22 = self[(2, 2)];
        let a_23 = self[(2, 3)];
        let a_24 = self[(2, 4)];
        let a_30 = self[(3, 0)];
        let a_31 = self[(3, 1)];
        let a_32 = self[(3, 2)];
        let a_33 = self[(3, 3)];
        let a_34 = self[(3, 4)];
        let a_40 = self[(4, 0)];
        let a_41 = self[(4, 1)];
        let a_42 = self[(4, 2)];
        let a_43 = self[(4, 3)];
        let a_44 = self[(4, 4)];

        let b_00 = rhs[(0, 0)];
        let b_01 = rhs[(0, 1)];
        let b_02 = rhs[(0, 2)];
        let b_03 = rhs[(0, 3)];
        let b_04 = rhs[(0, 4)];
        let b_10 = rhs[(1, 0)];
        let b_11 = rhs[(1, 1)];
        let b_12 = rhs[(1, 2)];
        let b_13 = rhs[(1, 3)];
        let b_14 = rhs[(1, 4)];
        let b_20 = rhs[(2, 0)];
        let b_21 = rhs[(2, 1)];
        let b_22 = rhs[(2, 2)];
        let b_23 = rhs[(2, 3)];
        let b_24 = rhs[(2, 4)];
        let b_30 = rhs[(3, 0)];
        let b_31 = rhs[(3, 1)];
        let b_32 = rhs[(3, 2)];
        let b_33 = rhs[(3, 3)];
        let b_34 = rhs[(3, 4)];
        let b_40 = rhs[(4, 0)];
        let b_41 = rhs[(4, 1)];
        let b_42 = rhs[(4, 2)];
        let b_43 = rhs[(4, 3)];
        let b_44 = rhs[(4, 4)];

        Matrix5x5::new([
            [
                a_00 + b_00,
                a_01 + b_01,
                a_02 + b_02,
                a_03 + b_03,
                a_04 + b_04,
            ],
            [
                a_10 + b_10,
                a_11 + b_11,
                a_12 + b_12,
                a_13 + b_13,
                a_14 + b_14,
            ],
            [
                a_20 + b_20,
                a_21 + b_21,
                a_22 + b_22,
                a_23 + b_23,
                a_24 + b_24,
            ],
            [
                a_30 + b_30,
                a_31 + b_31,
                a_32 + b_32,
                a_33 + b_33,
                a_34 + b_34,
            ],
            [
                a_40 + b_40,
                a_41 + b_41,
                a_42 + b_42,
                a_43 + b_43,
                a_44 + b_44,
            ],
        ])
    }
}

// NOTE(elsuizo:2020-04-27): primero a pedal
// despues si anda con for loops o como sea
impl<T: Float> Mul<T> for Matrix5x5<T> {
    type Output = Matrix5x5<T>;

    fn mul(self, rhs: T) -> Matrix5x5<T> {
        let a_00 = self[(0, 0)] * rhs;
        let a_01 = self[(0, 1)] * rhs;
        let a_02 = self[(0, 2)] * rhs;
        let a_03 = self[(0, 3)] * rhs;
        let a_04 = self[(0, 4)] * rhs;
        let a_10 = self[(1, 0)] * rhs;
        let a_11 = self[(1, 1)] * rhs;
        let a_12 = self[(1, 2)] * rhs;
        let a_13 = self[(1, 3)] * rhs;
        let a_14 = self[(1, 4)] * rhs;
        let a_20 = self[(2, 0)] * rhs;
        let a_21 = self[(2, 1)] * rhs;
        let a_22 = self[(2, 2)] * rhs;
        let a_23 = self[(2, 3)] * rhs;
        let a_24 = self[(2, 4)] * rhs;
        let a_30 = self[(3, 0)] * rhs;
        let a_31 = self[(3, 1)] * rhs;
        let a_32 = self[(3, 2)] * rhs;
        let a_33 = self[(3, 3)] * rhs;
        let a_34 = self[(3, 4)] * rhs;
        let a_40 = self[(4, 0)] * rhs;
        let a_41 = self[(4, 1)] * rhs;
        let a_42 = self[(4, 2)] * rhs;
        let a_43 = self[(4, 3)] * rhs;
        let a_44 = self[(4, 4)] * rhs;

        Matrix5x5::new([
            [a_00, a_01, a_02, a_03, a_04],
            [a_10, a_11, a_12, a_13, a_14],
            [a_20, a_21, a_22, a_23, a_24],
            [a_30, a_31, a_32, a_33, a_34],
            [a_40, a_41, a_42, a_43, a_44],
        ])
    }
}

impl<T: Float> Mul for Matrix5x5<T> {
    type Output = Self;

    fn mul(self, rhs: Self) -> Self {
        let a_00 = self[(0, 0)];
        let a_01 = self[(0, 1)];
        let a_02 = self[(0, 2)];
        let a_03 = self[(0, 3)];
        let a_04 = self[(0, 4)];
        let a_10 = self[(1, 0)];
        let a_11 = self[(1, 1)];
        let a_12 = self[(1, 2)];
        let a_13 = self[(1, 3)];
        let a_14 = self[(1, 4)];
        let a_20 = self[(2, 0)];
        let a_21 = self[(2, 1)];
        let a_22 = self[(2, 2)];
        let a_23 = self[(2, 3)];
        let a_24 = self[(2, 4)];
        let a_30 = self[(3, 0)];
        let a_31 = self[(3, 1)];
        let a_32 = self[(3, 2)];
        let a_33 = self[(3, 3)];
        let a_34 = self[(3, 4)];
        let a_40 = self[(4, 0)];
        let a_41 = self[(4, 1)];
        let a_42 = self[(4, 2)];
        let a_43 = self[(4, 3)];
        let a_44 = self[(4, 4)];

        let b_00 = rhs[(0, 0)];
        let b_01 = rhs[(0, 1)];
        let b_02 = rhs[(0, 2)];
        let b_03 = rhs[(0, 3)];
        let b_04 = rhs[(0, 4)];
        let b_10 = rhs[(1, 0)];
        let b_11 = rhs[(1, 1)];
        let b_12 = rhs[(1, 2)];
        let b_13 = rhs[(1, 3)];
        let b_14 = rhs[(1, 4)];
        let b_20 = rhs[(2, 0)];
        let b_21 = rhs[(2, 1)];
        let b_22 = rhs[(2, 2)];
        let b_23 = rhs[(2, 3)];
        let b_24 = rhs[(2, 4)];
        let b_30 = rhs[(3, 0)];
        let b_31 = rhs[(3, 1)];
        let b_32 = rhs[(3, 2)];
        let b_33 = rhs[(3, 3)];
        let b_34 = rhs[(3, 4)];
        let b_40 = rhs[(4, 0)];
        let b_41 = rhs[(4, 1)];
        let b_42 = rhs[(4, 2)];
        let b_43 = rhs[(4, 3)];
        let b_44 = rhs[(4, 4)];

        Matrix5x5::new([
            [
                a_00 * b_00 + a_01 * b_10 + a_02 * b_20 + a_03 * b_30 + a_04 * b_40,
                a_00 * b_01 + a_01 * b_11 + a_02 * b_21 + a_03 * b_31 + a_04 * b_41,
                a_00 * b_02 + a_01 * b_12 + a_02 * b_22 + a_03 * b_32 + a_04 * b_42,
                a_00 * b_03 + a_01 * b_13 + a_02 * b_23 + a_03 * b_33 + a_04 * b_43,
                a_00 * b_04 + a_01 * b_14 + a_02 * b_24 + a_03 * b_34 + a_04 * b_44,
            ],
            [
                a_10 * b_00 + a_11 * b_10 + a_12 * b_20 + a_13 * b_30 + a_14 * b_40,
                a_10 * b_01 + a_11 * b_11 + a_12 * b_21 + a_13 * b_31 + a_14 * b_41,
                a_10 * b_02 + a_11 * b_12 + a_12 * b_22 + a_13 * b_32 + a_14 * b_42,
                a_10 * b_03 + a_11 * b_13 + a_12 * b_23 + a_13 * b_33 + a_14 * b_43,
                a_10 * b_04 + a_11 * b_14 + a_12 * b_24 + a_13 * b_34 + a_14 * b_44,
            ],
            [
                a_20 * b_00 + a_21 * b_10 + a_22 * b_20 + a_23 * b_30 + a_24 * b_40,
                a_20 * b_01 + a_21 * b_11 + a_22 * b_21 + a_23 * b_31 + a_24 * b_41,
                a_20 * b_02 + a_21 * b_12 + a_22 * b_22 + a_23 * b_32 + a_24 * b_42,
                a_20 * b_03 + a_21 * b_13 + a_22 * b_23 + a_23 * b_33 + a_24 * b_43,
                a_20 * b_04 + a_21 * b_14 + a_22 * b_24 + a_23 * b_34 + a_24 * b_44,
            ],
            [
                a_30 * b_00 + a_31 * b_10 + a_32 * b_20 + a_33 * b_30 + a_34 * b_40,
                a_30 * b_01 + a_31 * b_11 + a_32 * b_21 + a_33 * b_31 + a_34 * b_41,
                a_30 * b_02 + a_31 * b_12 + a_32 * b_22 + a_33 * b_32 + a_34 * b_42,
                a_30 * b_03 + a_31 * b_13 + a_32 * b_23 + a_33 * b_33 + a_34 * b_43,
                a_30 * b_04 + a_31 * b_14 + a_32 * b_24 + a_33 * b_34 + a_34 * b_44,
            ],
            [
                a_40 * b_00 + a_41 * b_10 + a_42 * b_20 + a_43 * b_30 + a_44 * b_40,
                a_40 * b_01 + a_41 * b_11 + a_42 * b_21 + a_43 * b_31 + a_44 * b_41,
                a_40 * b_02 + a_41 * b_12 + a_42 * b_22 + a_43 * b_32 + a_44 * b_42,
                a_40 * b_03 + a_41 * b_13 + a_42 * b_23 + a_43 * b_33 + a_44 * b_43,
                a_40 * b_04 + a_41 * b_14 + a_42 * b_24 + a_43 * b_34 + a_44 * b_44,
            ],
        ])
    }
}

impl<T: Float> Zero for Matrix5x5<T> {
    fn zero() -> Matrix5x5<T> {
        Matrix5x5::new([[T::zero(); 5]; 5])
    }

    fn is_zero(&self) -> bool {
        *self == Matrix5x5::zero()
    }
}

impl<T: Float> One for Matrix5x5<T> {
    /// Create an identity matrix
    fn one() -> Matrix5x5<T> {
        let one = T::one();
        let zero = T::zero();
        Matrix5x5::new([
            [one, zero, zero, zero, zero],
            [zero, one, zero, zero, zero],
            [zero, zero, one, zero, zero],
            [zero, zero, zero, one, zero],
            [zero, zero, zero, zero, one],
        ])
    }
}

// NOTE(elsuizo:2020-04-26): poniendo ese Trait anda el norm2 funcional
impl<T: Float + std::iter::Sum> Matrix5x5<T> {
    pub fn identity() -> Matrix5x5<T> {
        <Matrix5x5<T> as One>::one()
    }

    pub fn zeros() -> Matrix5x5<T> {
        <Matrix5x5<T> as Zero>::zero()
    }

    pub fn as_vec(&self) -> Vec<T> {
        let result: Vec<T> = self.iter().flatten().cloned().collect();
        return result;
    }

    /// get the a submatrix from discard row `i` and column `j`
    ///
    fn get_submatrix(&self, selected: (usize, usize)) -> Matrix4x4<T> {
        let mut values: Vec<T> = Vec::new();
        let mut result: Matrix4x4<T> = Matrix4x4::zeros();
        for i in 0..self.rows() {
            for j in 0..self.cols() {
                if !(i == selected.0 || j == selected.1) {
                    // get the values from the Matrix4x4
                    values.push(self[(i, j)]);
                }
            }
        }
        let mut i = 0;
        for r in 0..result.rows() {
            for c in 0..result.cols() {
                result[(r, c)] = values[i];
                i += 1;
            }
        }
        return result;
    }
}

impl<T> Deref for Matrix5x5<T> {
    type Target = [[T; 5]; 5];
    #[inline]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}

impl<T> DerefMut for Matrix5x5<T> {
    #[inline]
    fn deref_mut(&mut self) -> &mut Self::Target {
        &mut self.0
    }
}

impl<T> From<[[T; 5]; 5]> for Matrix5x5<T> {
    fn from(data: [[T; 5]; 5]) -> Matrix5x5<T> {
        Matrix5x5(data)
    }
}

impl<T> Index<(usize, usize)> for Matrix5x5<T> {
    type Output = T;
    fn index(&self, index: (usize, usize)) -> &T {
        &self.0[index.0][index.1]
    }
}

impl<T> IndexMut<(usize, usize)> for Matrix5x5<T> {
    fn index_mut(&mut self, index: (usize, usize)) -> &mut T {
        &mut self.0[index.0][index.1]
    }
}

//-------------------------------------------------------------------------
//                        Display for Matrix5x5
//-------------------------------------------------------------------------
impl<T: Float + fmt::Display> fmt::Display for Matrix5x5<T> {
    fn fmt(&self, dest: &mut fmt::Formatter) -> fmt::Result {
        println!("");
        write!(
            dest,
            "|{0:<7.2} {1:^7.2} {2:^7.2} {3:^7.2} {4:>7.2}|\n",
            self[(0, 0)],
            self[(0, 1)],
            self[(0, 2)],
            self[(0, 3)],
            self[(0, 4)]
        )?;
        write!(
            dest,
            "|{0:<7.2} {1:^7.2} {2:^7.2} {3:^7.2} {4:>7.2}|\n",
            self[(1, 0)],
            self[(1, 1)],
            self[(1, 2)],
            self[(1, 3)],
            self[(1, 4)]
        )?;
        write!(
            dest,
            "|{0:<7.2} {1:^7.2} {2:^7.2} {3:^7.2} {4:>7.2}|\n",
            self[(2, 0)],
            self[(2, 1)],
            self[(2, 2)],
            self[(2, 3)],
            self[(2, 4)]
        )?;
        write!(
            dest,
            "|{0:<7.2} {1:^7.2} {2:^7.2} {3:^7.2} {4:>7.2}|\n",
            self[(3, 0)],
            self[(3, 1)],
            self[(3, 2)],
            self[(3, 3)],
            self[(3, 4)]
        )?;
        write!(
            dest,
            "|{0:<7.2} {1:^7.2} {2:^7.2} {3:^7.2} {4:>7.2}|\n",
            self[(4, 0)],
            self[(4, 1)],
            self[(4, 2)],
            self[(4, 3)],
            self[(4, 4)]
        )
    }
}
