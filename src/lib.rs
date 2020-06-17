/// A Robotics crate
#[macro_use]
extern crate static_math;
pub mod errors;
pub mod transformations;
pub mod types;
pub mod utils;

//-------------------------------------------------------------------------
//                        tests
//-------------------------------------------------------------------------

#[cfg(test)]
mod types_tests {
    use crate::types::{Point, Point2D};

    #[test]
    fn point2d_test() {
        let p = Point2D::new(1, 2, "A");

        assert_eq!(1, p.x);
        assert_eq!(2, p.y);
        assert_eq!("A", p.frame_name);
    }

    #[test]
    fn point_test() {
        let p = Point::new(1, 1, 3, "B");
        assert_eq!(1, p.x);
        assert_eq!(1, p.y);
        assert_eq!(3, p.z);
        assert_eq!("B", p.frame_name);
    }
}

#[cfg(test)]
mod transformations_tests {
    use static_math::matrix4x4::M44;
    use crate::transformations;
    use crate::utils::is_rotation;
    use static_math::utils::{nearly_equal, compare_vecs};

    const EPS: f32 = 1e-7;

    #[test]
    fn rotx_test() {

        use super::transformations_tests::EPS;

        let rot1 = transformations::rotx(30.0);
        let rot2 = transformations::rotx(30.0 + 360.0);

        assert!(is_rotation(rot2));
        assert!(is_rotation(rot1));
        assert!(compare_vecs(&rot1.as_vec(), &rot2.as_vec(), EPS));
    }

    #[test]
    fn roty_test() {

        use super::transformations_tests::EPS;

        let rot1 = transformations::roty(30.0);
        let rot2 = transformations::roty(30.0 + 360.0);


        assert!(is_rotation(rot1));
        assert!(is_rotation(rot2));
        assert!(compare_vecs(&rot1.as_vec(), &rot2.as_vec(), EPS));
    }

    #[test]
    fn rotz_test() {

        use super::transformations_tests::EPS;

        let rot1 = transformations::rotz(30.0);
        let rot2 = transformations::rotz(30.0 + 360.0);

        assert!(is_rotation(rot1));
        assert!(is_rotation(rot2));
        assert!(compare_vecs(&rot1.as_vec(), &rot2.as_vec(), EPS));
    }

    #[test]
    fn rot2trans_test() {

        use super::transformations_tests::EPS;

        let rot = transformations::rotx(0.0);
        let rot_trans = transformations::rot2trans(&rot);

        let i = M44::identity();

        assert!(compare_vecs(&i.as_vec(), &rot_trans.as_vec(), EPS));
    }

    #[test]
    fn euler_test() {

        use super::transformations_tests::EPS;

        let angles_in = (90.0, 30.0, 30.0);
        let rot = transformations::euler2rot(angles_in.0, angles_in.1, angles_in.2);
        let angles_out = transformations::rot2euler(rot);

        assert!(nearly_equal(angles_in.0, angles_out.0, EPS));
        assert!(nearly_equal(angles_in.1, angles_out.1, EPS));
        assert!(nearly_equal(angles_in.2, angles_out.2, EPS));
    }

    // TODO(elsuizo:2020-04-30): hay que ver bien porque no anda este test
    #[test]
    #[ignore]
    fn euler_singularity_test() {

        use super::transformations_tests::EPS;

        let angles_in = (90.0, 0.0, 0.0);
        let rot = transformations::euler2rot(angles_in.0, angles_in.1, angles_in.2);
        let angles_out = transformations::rot2euler(rot);

        println!("angles_out: {:?}", angles_out);

        assert!(nearly_equal(angles_in.0, angles_out.0, EPS));
        assert!(nearly_equal(angles_in.1, angles_out.1, EPS));
        assert!(nearly_equal(angles_in.2, angles_out.2, EPS));
    }
}
