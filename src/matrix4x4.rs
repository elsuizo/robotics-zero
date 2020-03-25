//-------------------------------------------------------------------------
// @file matrix4x4.rs
//
// @date 08/06/19 09:59:09
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
use std::ops::{Deref, DerefMut, Index, IndexMut};
use num_traits::{One, Zero, Float};

use std::ops::{Add, Mul};
use crate::matrix3x3::*;
use crate::errors::LinAlgebraError;
// use std::ops::{AddAssign, DivAssign, MulAssign, SubAssign};

#[derive(Copy, Clone, Debug, PartialEq)]
pub struct Matrix4x4<T>([[T; 4]; 4]);

impl<T> Matrix4x4<T> {

    pub fn new(data_input: [[T; 4]; 4]) -> Matrix4x4<T> {
        Matrix4x4(data_input)
    }

    pub fn rows(&self) -> usize {
        self.0.len()
    }
    // NOTE(elsuizo:2019-09-13): si ya se es medio...
    pub fn cols(&self) -> usize {
        self.rows()
    }

}

impl<T: Float> Matrix4x4<T> {

    pub fn convert2vector(&self) -> [T; 16] {
        let a1  = self[(0, 0)];
        let a2  = self[(0, 1)];
        let a3  = self[(0, 2)];
        let a4  = self[(0, 3)];
        let a5  = self[(1, 0)];
        let a6  = self[(1, 1)];
        let a7  = self[(1, 2)];
        let a8  = self[(1, 3)];
        let a9  = self[(2, 0)];
        let a10 = self[(2, 1)];
        let a11 = self[(2, 2)];
        let a12 = self[(2, 3)];
        let a13 = self[(3, 0)];
        let a14 = self[(3, 1)];
        let a15 = self[(3, 2)];
        let a16 = self[(3, 3)];
        [a1, a2, a3, a4, a5, a6, a7, a8, a9, a10, a11, a12, a13, a14, a15, a16]

    }

    pub fn identity() -> Matrix4x4<T> {
        <Matrix4x4<T> as One>::one()
    }

    pub fn zeros() -> Matrix4x4<T> {
        <Matrix4x4<T> as Zero>::zero()
    }

    pub fn trace(&self) -> T {
        return self[(0, 0)] + self[(1, 1)] + self[(2, 2)] + self[(3, 3)];
    }

    pub fn det(&self) -> Option<T> {
        let a1  = self[(0, 0)];
        let a2  = self[(0, 1)];
        let a3  = self[(0, 2)];
        let a4  = self[(0, 3)];
        let a5  = self[(1, 0)];
        let a6  = self[(1, 1)];
        let a7  = self[(1, 2)];
        let a8  = self[(1, 3)];
        let a9  = self[(2, 0)];
        let a10 = self[(2, 1)];
        let a11 = self[(2, 2)];
        let a12 = self[(2, 3)];
        let a13 = self[(3, 0)];
        let a14 = self[(3, 1)];
        let a15 = self[(3, 2)];
        let a16 = self[(3, 3)];

        let det = a1*a10*a15*a8 - a1*a10*a16*a7 - a1*a11*a14*a8 + a1*a11*a16*a6 + a1*a12*a14*a7 - a1*a12*a15*a6 - a10*a13*a3*a8 + a10*a13*a4*a7 - a10*a15*a4*a5 + a10*a16*a3*a5 + a11*a13*a2
        *a8 - a11*a13*a4*a6 + a11*a14*a4*a5 - a11*a16*a2*a5 - a12*a13*a2*a7 + a12*a13*a3*a6 - a12*a14*a3*a5 + a12*a15*a2*a5 + a14*a3*a8*a9 - a14*a4*a7*a9 - a15*a2*a8*a9 + a15*a4*
        a6*a9 + a16*a2*a7*a9 - a16*a3*a6*a9;

        if det.abs() > T::epsilon() {
            Some(det)
        } else {
            None
        }
    }

    pub fn inverse(&self) -> Result<Matrix4x4<T>, LinAlgebraError> {
        match self.det() {
            None      => Err(LinAlgebraError::DeterminantZero),
            Some(det) => {
                let a1 = self.get_submatrix((0, 0)).det().unwrap();
                let a2 = -self.get_submatrix((0, 1)).det().unwrap();
                let a3 = self.get_submatrix((0, 2)).det().unwrap();
                let a4 = -self.get_submatrix((0, 3)).det().unwrap();

                let a5 = -self.get_submatrix((1, 0)).det().unwrap();
                let a6 = self.get_submatrix((1, 1)).det().unwrap();
                let a7 = -self.get_submatrix((1, 2)).det().unwrap();
                let a8 = self.get_submatrix((1, 3)).det().unwrap();

                let a9 = self.get_submatrix((2, 0)).det().unwrap();
                let a10 = -self.get_submatrix((2, 1)).det().unwrap();
                let a11 = self.get_submatrix((2, 2)).det().unwrap();
                let a12 = -self.get_submatrix((2, 3)).det().unwrap();

                let a13 = -self.get_submatrix((3, 0)).det().unwrap();
                let a14 = self.get_submatrix((3, 1)).det().unwrap();
                let a15 = -self.get_submatrix((3, 2)).det().unwrap();
                let a16 = self.get_submatrix((3, 3)).det().unwrap();

                Ok(Matrix4x4::new([[a1/det, a5/det, a9/det, a13/det],
                                   [a2/det, a6/det, a10/det, a14/det],
                                   [a3/det, a7/det, a11/det, a15/det],
                                   [a4/det, a8/det, a12/det, a16/det]]))
            }
        }
    }
    pub fn transpose(&self) -> Matrix4x4<T> {
        let a1  = self[(0, 0)];
        let a2  = self[(0, 1)];
        let a3  = self[(0, 2)];
        let a4  = self[(0, 3)];
        let a5  = self[(1, 0)];
        let a6  = self[(1, 1)];
        let a7  = self[(1, 2)];
        let a8  = self[(1, 3)];
        let a9  = self[(2, 0)];
        let a10 = self[(2, 1)];
        let a11 = self[(2, 2)];
        let a12 = self[(2, 3)];
        let a13 = self[(3, 0)];
        let a14 = self[(3, 1)];
        let a15 = self[(3, 2)];
        let a16 = self[(3, 3)];

        Matrix4x4::new([[a1, a5, a9, a13], [a2, a6, a10, a14], [a3, a7, a11, a15], [a4, a8, a12, a16]])
    }

    pub fn norm2(&self) -> T {
        let a1  = self[(0, 0)];
        let a2  = self[(0, 1)];
        let a3  = self[(0, 2)];
        let a4  = self[(0, 3)];
        let a5  = self[(1, 0)];
        let a6  = self[(1, 1)];
        let a7  = self[(1, 2)];
        let a8  = self[(1, 3)];
        let a9  = self[(2, 0)];
        let a10 = self[(2, 1)];
        let a11 = self[(2, 2)];
        let a12 = self[(2, 3)];
        let a13 = self[(3, 0)];
        let a14 = self[(3, 1)];
        let a15 = self[(3, 2)];
        let a16 = self[(3, 3)];

        T::sqrt(a1 * a1 + a2 * a2 + a3 * a3 + a4 * a4 + a5 * a5 + a6 * a6 + a7 * a7
                + a8 * a8 + a9 * a9 + a10 * a10 + a11 * a11 + a12 * a12 + a13 * a13
                + a14 * a14 + a15 * a15 + a16 * a16)
    }

    // TODO(elsuizo:2020-03-24): tendria que verificar que la tupla que ingresan es valida
    pub fn get_submatrix(&self, selected: (usize, usize)) -> Matrix3x3<T> {
        let mut values: Vec<T> = Vec::new();
        let mut result: Matrix3x3<T> = Matrix3x3::zeros();
        for i in 0..self.rows() {
            for j in 0..self.cols() {
                if !(i == selected.0 || j == selected.1) {
                    // get the values from the Matrix3x3
                    values.push(self[(i, j)]);
                    // println!("m[({:}, {:})]: {:}", i, j, m[(i, j)]);
                }
            }
        }
        let mut i = 0;
        for r in 0..result.rows() {
            for c in 0..result.cols() {
                // println!("r:{:}, c:{}, i:{:}", r, c, i);
                result[(r, c)] = values[i];
                i += 1;
            }
        }

        return result;
    }
}

impl<T: Float> Add for Matrix4x4<T> {
    type Output = Self;

    fn add(self, rhs: Self) -> Self {
        let a1  = self[(0, 0)];
        let a2  = self[(0, 1)];
        let a3  = self[(0, 2)];
        let a4  = self[(0, 3)];
        let a5  = self[(1, 0)];
        let a6  = self[(1, 1)];
        let a7  = self[(1, 2)];
        let a8  = self[(1, 3)];
        let a9  = self[(2, 0)];
        let a10 = self[(2, 1)];
        let a11 = self[(2, 2)];
        let a12 = self[(2, 3)];
        let a13 = self[(3, 0)];
        let a14 = self[(3, 1)];
        let a15 = self[(3, 2)];
        let a16 = self[(3, 3)];
        let b1  = rhs[(0, 0)];
        let b2  = rhs[(0, 1)];
        let b3  = rhs[(0, 2)];
        let b4  = rhs[(0, 3)];
        let b5  = rhs[(1, 0)];
        let b6  = rhs[(1, 1)];
        let b7  = rhs[(1, 2)];
        let b8  = rhs[(1, 3)];
        let b9  = rhs[(2, 0)];
        let b10 = rhs[(2, 1)];
        let b11 = rhs[(2, 2)];
        let b12 = rhs[(2, 3)];
        let b13 = rhs[(3, 0)];
        let b14 = rhs[(3, 1)];
        let b15 = rhs[(3, 2)];
        let b16 = rhs[(3, 3)];
        Matrix4x4::new([ [a1 + b1,   a2 + b2,   a3 + b3,   a4 + b4],
                         [a5 + b5,   a6 + b6,   a7 + b7,   a8 + b8],
                         [a9 + b9,   a10 + b10, a11 + b11, a12 + b12],
                         [a13 + b13, a14 + b14, a15 + b15, a16 + b16]])
    }

}

impl<T: Float> Mul for Matrix4x4<T> {
    type Output = Self;

    fn mul(self, rhs: Self) -> Self {
        let a1  = self[(0, 0)];
        let a2  = self[(0, 1)];
        let a3  = self[(0, 2)];
        let a4  = self[(0, 3)];
        let a5  = self[(1, 0)];
        let a6  = self[(1, 1)];
        let a7  = self[(1, 2)];
        let a8  = self[(1, 3)];
        let a9  = self[(2, 0)];
        let a10 = self[(2, 1)];
        let a11 = self[(2, 2)];
        let a12 = self[(2, 3)];
        let a13 = self[(3, 0)];
        let a14 = self[(3, 1)];
        let a15 = self[(3, 2)];
        let a16 = self[(3, 3)];
        let b1  = rhs[(0, 0)];
        let b2  = rhs[(0, 1)];
        let b3  = rhs[(0, 2)];
        let b4  = rhs[(0, 3)];
        let b5  = rhs[(1, 0)];
        let b6  = rhs[(1, 1)];
        let b7  = rhs[(1, 2)];
        let b8  = rhs[(1, 3)];
        let b9  = rhs[(2, 0)];
        let b10 = rhs[(2, 1)];
        let b11 = rhs[(2, 2)];
        let b12 = rhs[(2, 3)];
        let b13 = rhs[(3, 0)];
        let b14 = rhs[(3, 1)];
        let b15 = rhs[(3, 2)];
        let b16 = rhs[(3, 3)];
        Matrix4x4::new([[a1*b1 + a2*b5 + a3*b9 + a4*b13, a1*b2 + a2*b6 + a3*b10 + a4*b14, a1*b3 + a2*b7 + a3*b11 + a4*b15, a1*b4 + a2*b8 + a3*b12 + a4*b16],
                        [a5*b1 + a6*b5 + a7*b9 + a8*b13, a5*b2 + a6*b6 + a7*b10 + a8*b14, a5*b3 + a6*b7 + a7*b11 + a8*b15, a5*b4 + a6*b8 + a7*b12 + a8*b16],
                        [a10*b5 + a11*b9 + a12*b13 + a9*b1, a10*b6 + a11*b10 + a12*b14 + a9*b2, a10*b7 + a11*b11 + a12*b15 + a9*b3, a10*b8 + a11*b12 + a12*b16 + a9*b4],
                        [a13*b1 + a14*b5 + a15*b9 + a16*b13, a13*b2 + a14*b6 + a15*b10 + a16*b14, a13*b3 + a14*b7 +a15*b11 + a16*b15, a13*b4 + a14*b8 + a15*b12 + a16*b16]])
    }
}

impl<T: Float> Zero for Matrix4x4<T> {
    fn zero() -> Matrix4x4<T> {
        Matrix4x4::new([[T::zero(); 4]; 4])
    }

    fn is_zero(&self) -> bool {
        *self == Matrix4x4::zero()
    }
}

impl<T: Float> One for Matrix4x4<T> {
    /// Create an identity matrix
    fn one() -> Matrix4x4<T> {
        let one = T::one();
        let zero = T::zero();
        Matrix4x4::new([[one, zero, zero, zero], [zero, one, zero, zero], [zero, zero, one, zero], [zero, zero, zero, one]])
    }
}
//
impl<T> Deref for Matrix4x4<T> {
    type Target = [[T; 4]; 4];
    #[inline]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}

impl<T> DerefMut for Matrix4x4<T> {
    #[inline]
    fn deref_mut(&mut self) -> &mut Self::Target {
        &mut self.0
    }
}

impl<T> From<[[T; 4]; 4]> for Matrix4x4<T> {
    fn from(data: [[T; 4]; 4]) -> Matrix4x4<T> {
        Matrix4x4(data)
    }
}

impl<T> Index<(usize, usize)> for Matrix4x4<T> {
    type Output = T;
    fn index(&self, index: (usize, usize)) -> &T {
        &self.0[index.0][index.1]
    }
}

impl<T> IndexMut<(usize, usize)> for Matrix4x4<T> {
    fn index_mut(&mut self, index: (usize, usize)) -> &mut T {
            &mut self.0[index.0][index.1]
    }
}
