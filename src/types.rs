//-------------------------------------------------------------------------
// @file types.rs
//
// @date 06/13/19 11:51:30
// @author Martin Noblia
// @email mnoblia@disroot.org
//
// @brief
// A principal types for robotics proposites
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
// NOTE(elsuizo:2019-09-08): lo que quiero hacer es tener un type que sea un punto en dos
// dimensiones y que tenga asociado un frame a el y que solo se pueda operar con otro type si es
// que estan expresados en el mismo frame

// NOTE(elsuizo:2019-09-10): no se si quiero que todos los elementos sean publicos, lo dejo para
// los tests
//
// use crate::matrix3x3::Matrix3x3;
//
// NOTE(elsuizo:2020-04-25): recordemos que los lifetime van antes que los Types
// imports
use crate::matrix2x2::Matrix2x2;
use crate::matrix3x3::Matrix3x3;
use crate::matrix4x4::Matrix4x4;
use crate::matrix5x5::Matrix5x5;
use crate::matrix6x6::Matrix6x6;
use crate::vector2::Vector2;
use crate::vector3::Vector3;
use crate::vector4::Vector4;
use crate::vector5::Vector5;
use crate::vector6::Vector6;
//-------------------------------------------------------------------------
//                        custom types
//-------------------------------------------------------------------------

pub enum VectorTypes<T> {
    Scalar(T),
    V2(Vector2<T>),
    V3(Vector3<T>),
    V4(Vector4<T>),
    V5(Vector5<T>),
    V6(Vector6<T>),
}

pub enum MatrixTypes<T> {
    M22(Matrix2x2<T>),
    M33(Matrix3x3<T>),
    M44(Matrix4x4<T>),
    M55(Matrix5x5<T>),
    M66(Matrix6x6<T>),
}

pub struct Point2D<'a, T> {
    pub x: T,
    pub y: T,
    pub frame_name: &'a str,
}

impl<'a, T> Point2D<'a, T> {
    pub fn new(x: T, y: T, frame_name: &'a str) -> Self {
        Self { x, y, frame_name }
    }
}

pub struct Point<'a, T> {
    pub x: T,
    pub y: T,
    pub z: T,
    pub frame_name: &'a str,
}

impl<'a, T> Point<'a, T> {
    pub fn new(x: T, y: T, z: T, frame_name: &'a str) -> Self {
        Self {
            x,
            y,
            z,
            frame_name,
        }
    }
}
