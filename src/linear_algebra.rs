//-------------------------------------------------------------------------
// @file linear_algebra.rs
//
// @date 08/07/19 09:37:39
// @author Martin Noblia
// @email mnoblia@disroot.org
//
// @brief
// Common linear algebra methods
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

trait<T> LinearAlgebra {

    fn identity() -> Self;
    fn zeros() -> Self;
    fn trace(&mut self) -> T;
    fn det(&mut self) -> T;
    fn transpose(&mut self) -> Self
}



