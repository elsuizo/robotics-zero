// -------------------------------------------------------------------------
// @file error.rs
//
// @date 03/24/20 16:08:12
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

use std::error;
use std::fmt;

pub type Result<T1> = ::std::result::Result<T1, LinAlgebraError>;

/// Errors for Linear Algebra operations
#[derive(Debug)]
pub enum LinAlgebraError {
    /// the arrays must be equal and dimension 3
    InvalidDimentionOrNotEq {
        len_u: usize,
        len_v: usize,
    },

    InvalidVectorDimention {
        len_v: usize,
    },
    InvalidMatrixShape {
        rows: usize,
        columns: usize,
    },
    DeterminantZero,
    Vector3OrScalar,
}

impl fmt::Display for LinAlgebraError {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match self {
            LinAlgebraError::InvalidDimentionOrNotEq { len_u, len_v } => write!(
                f,
                "The arrays must be of dimention 3: len of u:({}) != len of v:({})",
                len_u, len_v
            ),
            LinAlgebraError::InvalidVectorDimention { len_v } => write!(
                f,
                "The vector must be of dimention 1 or 3: len of vector input is: {}",
                len_v
            ),
            LinAlgebraError::InvalidMatrixShape { rows, columns } => write!(
                f,
                "The matrix must be of rows: 2 or 3 and columns: 2 or 3, got: rows{}, columns{}",
                rows, columns
            ),
            LinAlgebraError::DeterminantZero => write!(f, "The Matrix is not invertible"),
            LinAlgebraError::Vector3OrScalar => write!(f, "The input must be a Vector3 or scalar"),
        }
    }
}

impl error::Error for LinAlgebraError {
    fn description(&self) -> &str {
        "Ha ocurrido un error!!!"
    }

    // TODO(elsuizo:2020-03-24): tendria que ver bien esto
    fn cause(&self) -> Option<&dyn error::Error> {
        None
    }
}
