//-------------------------------------------------------------------------
// @file linear_algebra.rs
//
// @date 08/15/19 10:38:11
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
/// Generic Trait for Matrix operations and Linear Algebra methods
///
use crate::errors::LinAlgebraError;

pub trait LinearAlgebra<T> {

    fn rows(&self) -> usize;

    fn cols(&self) -> usize;

    fn shape(&self) -> (usize, usize) {
        (self.rows(), self.cols())
    }

    fn transpose(&self) -> Self;

    fn trace(&self) -> T;

    fn norm2(&self) -> T;

    fn det(&self) -> T;

    fn inverse(&self) -> Result<Self, LinAlgebraError> where Self: Sized;
}


