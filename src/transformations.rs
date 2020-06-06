// -------------------------------------------------------------------------
// @file transformations.rs
//
// @date 03/25/20 15:04:27
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
use static_math::matrix2x2::M22;
use static_math::matrix3x3::M33;
use static_math::matrix4x4::M44;
use static_math::utils::nearly_equal;
use static_math::vector3::V3;
use static_math::vector6::V6;
use num::{Float, Zero};

//-------------------------------------------------------------------------
//                        transformations
//-------------------------------------------------------------------------
/// Compute rotation matrix from a angle in degrees
pub fn rot2<T: Float>(angle: T) -> M22<T> {
    let c = angle.to_radians().cos();
    let s = angle.to_radians().sin();
    M22::new([[c, -s], [s, c]])
}

/// brief.
///
/// compute the rotation around the `x` axis(in cartesian coordinates)
///
/// description
///
/// * `angle` - angle of rotation in degrees
///
pub fn rotx<T: Float>(angle: T) -> M33<T> {
    let one = T::one();
    let zero = T::zero();
    let c = angle.to_radians().cos();
    let s = angle.to_radians().sin();
    M33::new([[one, zero, zero], [zero, c, -s], [zero, s, c]])
}

/// Brief.
///
/// Compute the rotation around the `y` axis(in cartesian coordinates)
///
/// Description
///
/// * `angle` - Angle of rotation in degrees
///
pub fn roty<T: Float>(angle: T) -> M33<T> {
    let one = T::one();
    let zero = T::zero();
    let c = angle.to_radians().cos();
    let s = angle.to_radians().sin();
    M33::new([[c, zero, s], [zero, one, zero], [-s, zero, c]])
}

/// Brief.
///
/// Compute the rotation around the `z` axis(in cartesian coordinates)
///
/// Description
///
/// * `angle` - Angle of rotation in degrees
///
pub fn rotz<T: Float>(angle: T) -> M33<T> {
    let one = T::one();
    let zero = T::zero();
    let c = angle.to_radians().cos();
    let s = angle.to_radians().sin();
    M33::new([[c, -s, zero], [s, c, zero], [zero, zero, one]])
}

/// Brief.
///
/// Convert a Rotation Matrix to
///
/// Function arguments:
/// `r`: M33<T>
///
/// Output:
/// R: M44
///
pub fn rot2trans<T: Float>(r: &M33<T>) -> M44<T> {
    let mut result = M44::zero();
    for row in 0..r.rows() {
        for column in 0..r.cols() {
            result[(row, column)] = r[(row, column)]
        }
    }
    result[(3, 3)] = T::one();
    return result;
}

/// Brief.
///
/// Compute the rotation around the `x` axis(in cartesian coordinates)
///
/// Function arguments:
///  `angle`: Float number
///
/// Output:
/// M44<Float>
///
pub fn trotx<T: Float>(angle: T) -> M44<T> {
    rot2trans(&rotx(angle))
}

/// Brief.
///
/// Compute the rotation around the `y` axis(in cartesian coordinates)
///
/// Function arguments:
/// `angle`: Float number
///
/// Output:
/// M44<Float>
///
pub fn troty<T: Float>(angle: T) -> M44<T> {
    rot2trans(&roty(angle))
}

/// Brief.
///
/// Compute the rotation around the `z` axis(in cartesian coordinates)
///
/// Function arguments:
///  `angle`: Float number
///
/// Output:
/// M44<Float>
///
pub fn trotz<T: Float>(angle: T) -> M44<T> {
    rot2trans(&rotz(angle))
}

/// Brief.
///
/// Compute the rotation matrix from euler angles from the convenction(ZYZ)
///
/// Function arguments:
/// phi: first euler angle (Float number)
/// theta: second euler angle (Float number)
/// psi: third euler angle (Float number)
///
/// Output:
/// R: Rotation matrix(M44<Float>)
///
pub fn euler2rot<T: Float>(angle_phi: T, angle_theta: T, angle_psi: T) -> M33<T> {
    rotz(angle_phi) * roty(angle_theta) * rotz(angle_psi)
}

/// Brief.
///
/// Compute the Rotation matrix from an arbitrary axis and angle
///
/// Function arguments:
/// theta:  angle of rotation(Float)
/// vector: axis of rotation(V3<Float>)
///
/// Return:
/// R: Rotation matrix(M33<Float>)
///
pub fn angle_vector2rot<T: Float>(theta: T, vector: V3<T>) -> M33<T> {
    let c = theta.cos();
    let s = theta.sin();
    let comp = T::one() - c;
    let v_x = vector[0];
    let v_y = vector[1];
    let v_z = vector[2];

    M33::new([
        [
            v_x * v_x * comp + c,
            v_y * v_x * comp - v_z * s,
            v_z * v_x * comp + v_y * s,
        ],
        [
            v_x * v_y * comp + v_z * s,
            v_y * v_y * comp + c,
            v_z * v_y * comp - v_x * s,
        ],
        [
            v_x * v_z * comp - v_y * s,
            v_y * v_z * comp + v_x * s,
            v_z * v_z * comp + c,
        ],
    ])
}

// TODO(elsuizo:2020-04-30): ver y explicar bien cuando ocurria una singularidad
/// Brief.
///
/// Compute the euler angles from a Rotation matrix(ZYZ convention)
///
/// Function arguments:
/// `R`: Rotation matrix
///
/// Output:
/// A tuple with the angles: phi, theta, psi
///
pub fn rot2euler<T: Float>(r: M33<T>) -> (T, T, T) {
    if nearly_equal(r[(0, 2)], T::zero(), T::epsilon()) && nearly_equal(r[(1, 2)], T::zero(), T::epsilon()) {
        // singularity
        println!("warning singularity occurs");
        let phi = T::zero();
        let sp = T::zero();
        let cp = T::one();
        let theta = (cp * r[(0, 2)] + sp * r[(1, 2)]).atan2(r[(2, 2)]);
        let psi = (-sp * r[(0, 0)] + cp * r[(1, 0)]).atan2(-sp * r[(0, 1)] + cp * r[(1, 1)]);
        return (phi.to_degrees(), theta.to_degrees(), psi.to_degrees());
    } else {
        // non-singular
        let phi = r[(1, 2)].atan2(r[(0, 2)]);
        let sp = phi.sin();
        let cp = phi.cos();
        let theta = (cp * r[(0, 2)] + sp * r[(1, 2)]).atan2(r[(2, 2)]);
        let psi = (-sp * r[(0, 0)] + cp * r[(1, 0)]).atan2(-sp * r[(0, 1)] + cp * r[(1, 1)]);
        return (phi.to_degrees(), theta.to_degrees(), psi.to_degrees());
    }
}

pub fn rot_euler_zyx<T: Float>(phi: T, theta: T, psi: T) -> M33<T> {
    rotz(phi) * roty(theta) * rotx(psi)
}

/// Brief.
///
/// Compute the rotation matrix from euler angles
///
/// Function arguments:
/// phi: first euler angle(Float)
/// theta: second euler angle(Float)
/// psi: third euler angle(Float)
///
/// Output:
/// R: Rotation matrix(M33<Float>)
///
pub fn euler2trans<T: Float>(phi: T, theta: T, psi: T) -> M44<T> {
    rot2trans(&euler2rot(phi, theta, psi))
}

pub fn skew_from_vec<T: Float>(v: V3<T>) -> M33<T> {
    let zero = T::zero();
    M33::new([
        [zero, -v[2], v[1]],
        [v[2], zero, -v[0]],
        [-v[1], v[0], zero],
    ])
}

pub fn skew_scalar<T: Float>(number: T) -> M22<T> {
    let zero = T::zero();
    M22::new([[zero, -number], [number, zero]])
}

///
/// Create augmented skew-symmetric matrix
pub fn skew_v3<T: Float>(v: V3<T>) -> M33<T> {
    let zero = T::zero();
    M33::new([[zero, -v[2], v[0]], [v[2], zero, v[1]], [zero, zero, zero]])
}

/// Create augmented skew-symmetric matrix
pub fn skew_v6<T: Float>(v: V6<T>) -> M44<T> {
    let zero = T::zero();
    M44::new([
        [zero, -v[5], v[4], v[0]],
        [v[5], zero, -v[3], v[1]],
        [-v[4], v[3], zero, v[2]],
        [zero, zero, zero, zero],
    ])
}

// NOTE(elsuizo:2020-05-01): no me gusta como queda ese unwrap ahi feo...
pub fn vex_m22<T: Float>(m: M22<T>) -> T {
    T::from(0.5).unwrap() * (m[(1, 0)] - m[(0, 1)])
}

pub fn vex_m33<T: Float>(m: M33<T>) -> V3<T> {
    let constant = T::from(0.5).unwrap();
    V3::new([
        m[(2, 1)] - m[(1, 2)],
        m[(0, 2)] - m[(2, 0)],
        m[(1, 0)] - m[(0, 1)],
    ]) * constant
}

/// Create a pose in 2D from a angle(in degrees) and a cartesian position (x, y) values
pub fn ksi<T: Float>(angle: T, x: T, y: T) -> M33<T> {
    let zero = T::zero();
    let one = T::one();
    let c = angle.to_radians().cos();
    let s = angle.to_radians().sin();

    M33::new([[c, -s, x], [s, c, y], [zero, zero, one]])
}

/// Create a pure translation pose
pub fn translation<T: Float>(x: T, y: T) -> M33<T> {
    let zero = T::zero();
    let one = T::one();
    M33::new([[one, zero, x], [zero, one, y], [zero, zero, one]])
}
