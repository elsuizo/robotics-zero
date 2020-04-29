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
// TODO(elsuizo:2019-09-12): cosas que faltarian para este type:
// - [ ] Implementar RangeFull para poder hacer matrix[..]
// - [ ] Implementar Iterator
// - [ ] Implementar std::fmt::Display para visualizar cuando imprimimos resultados
use std::ops::{Deref, DerefMut, Index, IndexMut};
use std::ops::{Add, Mul};
use std::fmt;

use num_traits::{One, Zero, Float};
use crate::errors::LinAlgebraError;
// TODO(elsuizo:2019-09-12): estos no los estoy utilizando, deberia???
// use std::ops::{AddAssign, DivAssign, MulAssign, SubAssign};

//-------------------------------------------------------------------------
//                        imports
//-------------------------------------------------------------------------
use crate::vector2::*;

//-------------------------------------------------------------------------
//                        code
//-------------------------------------------------------------------------
#[derive(Copy, Clone, Debug, PartialEq)]
pub struct Matrix2x2<T>([[T; 2]; 2]);


impl<T> Matrix2x2<T> {
    pub fn new(data_input: [[T; 2]; 2]) -> Matrix2x2<T> {
        Matrix2x2(data_input)
    }
    pub fn rows(&self) -> usize {
        self.0.len()
    }
    pub fn cols(&self) -> usize {
        self.rows()
    }
}

impl<T: Float> Matrix2x2<T> {

    pub fn identity() -> Matrix2x2<T> {
        <Matrix2x2<T> as One>::one()
    }

    pub fn zeros() -> Matrix2x2<T> {
        <Matrix2x2<T> as Zero>::zero()
    }

    pub fn trace(&self) -> T {
        return self[(0, 0)] + self[(1, 1)];
    }

    pub fn det(&self) -> T {
        let a = self[(0, 0)];
        let b = self[(0, 1)];
        let c = self[(1, 0)];
        let d = self[(1, 1)];
        (a * d) - (c * b)
    }

    pub fn transpose(&self) -> Matrix2x2<T> {
        let a = self[(0, 0)];
        let b = self[(0, 1)];
        let c = self[(1, 0)];
        let d = self[(1, 1)];
        Matrix2x2::new([
            [a, c],
            [b, d]
        ])
    }

    pub fn norm2(&self) -> T {
        let a = self[(0, 0)];
        let b = self[(0, 1)];
        let c = self[(1, 0)];
        let d = self[(1, 1)];
        T::sqrt(
            a * a + b * b + c * c + d * d
        )
    }

    pub fn as_vec(&self) -> Vec<T> {
        let result: Vec<T> = self.iter().flatten().cloned().collect();
        return result
    }
}

impl<T: Float> Matrix2x2<T> {
    pub fn inverse(&self) -> Result<Matrix2x2<T>, LinAlgebraError> {
        let a = self[(0, 0)];
        let b = self[(0, 1)];
        let c = self[(1, 0)];
        let d = self[(1, 1)];
        let det = self.det();
        if det.abs() > T::epsilon() {
            Ok(Matrix2x2::new([[d/det, -b/det], [-c/det, a/det]]))
        } else {
            Err(LinAlgebraError::DeterminantZero)
        }
    }
}

impl<T: Float> Mul<Vector2<T>> for Matrix2x2<T> {
    type Output = Vector2<T>;

    fn mul(self, rhs: Vector2<T>) -> Vector2<T> {
        let a1 = self[(0, 0)];
        let b1 = self[(0, 1)];
        let c1 = self[(1, 0)];
        let d1 = self[(1, 1)];

        let v1 = rhs[0];
        let v2 = rhs[1];
        Vector2::new([a1 * v1 + b1 * v2, c1 * v1 + d1 * v2])
    }
}

impl<T: Float> Add for Matrix2x2<T> {
    type Output = Self;

    fn add(self, rhs: Self) -> Self {
        let a1 = self[(0, 0)];
        let b1 = self[(0, 1)];
        let c1 = self[(1, 0)];
        let d1 = self[(1, 1)];

        let a2 = rhs[(0, 0)];
        let b2 = rhs[(0, 1)];
        let c2 = rhs[(1, 0)];
        let d2 = rhs[(1, 1)];
        Matrix2x2::new([
            [a1 + a2, b1 + b2],
            [c1 + c2, d1 + d2],
        ])
    }
}

impl<T: Float> Mul<T> for Matrix2x2<T> {
    type Output = Matrix2x2<T>;

    fn mul(self, rhs: T) -> Matrix2x2<T> {
        let a_00 = self[(0, 0)] * rhs;
        let a_01 = self[(0, 1)] * rhs;
        let a_10 = self[(1, 0)] * rhs;
        let a_11 = self[(1, 1)] * rhs;

        Matrix2x2::new([[a_00, a_01],
                        [a_10, a_11]])
    }
}

impl<T: Float> Mul for Matrix2x2<T> {
    type Output = Self;


    fn mul(self, rhs: Self) -> Self {
        let a1 = self[(0, 0)];
        let b1 = self[(0, 1)];
        let c1 = self[(1, 0)];
        let d1 = self[(1, 1)];

        let a2 = rhs[(0, 0)];
        let b2 = rhs[(0, 1)];
        let c2 = rhs[(1, 0)];
        let d2 = rhs[(1, 1)];

        let m00 = a1 * a2 + b1 * c2;
        let m01 = a1 * b2 + b1 * d2;

        let m10 = c1 * a2 + d1 * c2;
        let m11 = c1 * b2 + d1 * d2;
        Matrix2x2::new([
            [m00, m01],
            [m10, m11],
        ])
    }
}

impl<T: Float> Zero for Matrix2x2<T> {
    fn zero() -> Matrix2x2<T> {
        Matrix2x2::new([[T::zero(); 2]; 2])
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
        Matrix2x2::new([[one, zero], [zero, one]])
    }
}

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

impl<T> Index<(usize, usize)> for Matrix2x2<T> {
    type Output = T;
    fn index(&self, index: (usize, usize)) -> &T {
        &self.0[index.0][index.1]
    }
}

impl<T> IndexMut<(usize, usize)> for Matrix2x2<T> {
    fn index_mut(&mut self, index: (usize, usize)) -> &mut T {
            &mut self.0[index.0][index.1]
    }
}

//-------------------------------------------------------------------------
//                        Display for Matrix2x2
//-------------------------------------------------------------------------
impl<T: Float + fmt::Display> fmt::Display for Matrix2x2<T> {
    fn fmt(&self, dest: &mut fmt::Formatter) -> fmt::Result {
                println!("");
                write!(dest, "|{0:<7.2} {1:>7.2}|\n", self[(0, 0)], self[(0, 1)])?;
                write!(dest, "|{0:<7.2} {1:>7.2}|\n", self[(1, 0)], self[(1, 1)])
        }
}
