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

pub struct Point2D<T> {
    pub x: T,
    pub y: T,
    pub frame_name: String,
}

impl<T> Point2D<T> {
    pub fn new(x: T, y: T, name: &str) -> Self {
        Point2D {
            x: x,
            y: y,
            frame_name: name.to_string(),
        }
    }
}

pub struct Point<T> {
    pub x: T,
    pub y: T,
    pub z: T,
    pub frame_name: String,
}

impl<T> Point<T> {
    pub fn new(x: T, y: T, z: T, name: &str) -> Self {
        Point {
            x: x,
            y: y,
            z: z,
            frame_name: name.to_string(),
        }
    }
}
