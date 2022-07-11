use crate::matrix;
use crate::matrix::Matrix;

pub fn translation(x: f64, y: f64, z: f64) -> Matrix {
    matrix!([1, 0, 0, x], [0, 1, 0, y], [0, 0, 1, z], [0, 0, 0, 1])
}

pub fn scaling(x: f64, y: f64, z: f64) -> Matrix {
    matrix!([x, 0, 0, 0], [0, y, 0, 0], [0, 0, z, 0], [0, 0, 0, 1])
}

pub fn rotation_x(r: f64) -> Matrix {
    matrix!(
        [1.0, 0.0, 0.0, 0.0],
        [0.0, r.cos(), -r.sin(), 0.0],
        [0.0, r.sin(), r.cos(), 0.0],
        [0.0, 0.0, 0.0, 1.0]
    )
}

pub fn rotation_y(r: f64) -> Matrix {
    matrix!(
        [r.cos(), 0.0, r.sin(), 0.0],
        [0.0, 1.0, 0.0, 0.0],
        [-r.sin(), 0.0, r.cos(), 0.0],
        [0.0, 0.0, 0.0, 1.0]
    )
}

pub fn rotation_z(r: f64) -> Matrix {
    matrix!(
        [r.cos(), -r.sin(), 0.0, 0.0],
        [r.sin(), r.cos(), 0.0, 0.0],
        [0.0, 0.0, 1.0, 0.0],
        [0.0, 0.0, 0.0, 1.0]
    )
}

pub fn shearing(x_y: f64, x_z: f64, y_x: f64, y_z: f64, z_x: f64, z_y: f64) -> Matrix {
    matrix!(
        [1.0, x_y, x_z, 0.0],
        [y_x, 1.0, y_z, 0.0],
        [z_x, z_y, 1.0, 0.0],
        [0.0, 0.0, 0.0, 1.0]
    )
}

#[cfg(test)]
mod tests {
    use std::f64::consts::PI;

    use super::*;
    use crate::tuple::Tuple;

    #[test]
    fn multiply_translation_matrix() {
        let transform = translation(5., -3., 2.);
        let p = Tuple::new_point(-3., 4., 5.);

        assert_eq!(transform * p, Tuple::new_point(2., 1., 7.));
    }

    #[test]
    fn mulitply_inverse_translation_matrix() {
        let transform = translation(5., -3.0, 2.);
        let inv = transform.inverse().unwrap();
        let p = Tuple::new_point(-3., 4., 5.);

        assert_eq!(inv * p, Tuple::new_point(-8.0, 7.0, 3.0));
    }

    #[test]
    fn multiply_vector_translation_matrix() {
        let transform = translation(5.0, -3.0, 2.0);
        let v = Tuple::new_vector(-3.0, 4.0, 5.0);

        assert_eq!(transform * v, v);
    }

    #[test]
    fn multiply_point_by_scaling_matrix() {
        let transform = scaling(2.0, 3.0, 4.0);
        let p = Tuple::new_point(-4.0, 6.0, 8.0);
        assert_eq!(transform * p, Tuple::new_point(-8.0, 18.0, 32.0));
    }

    #[test]
    fn multiply_vector_by_scaling_matrix() {
        let transform = scaling(2.0, 3.0, 4.0);
        let p = Tuple::new_vector(-4.0, 6.0, 8.0);
        assert_eq!(transform * p, Tuple::new_vector(-8.0, 18.0, 32.0));
    }

    #[test]
    fn mulitply_by_inverse_scaling_matrix() {
        let transform = scaling(2.0, 3.0, 4.0);
        let inv = transform.inverse().unwrap();
        let v = Tuple::new_vector(-4.0, 6.0, 8.0);

        assert_eq!(inv * v, Tuple::new_vector(-2.0, 2.0, 2.0));
    }

    #[test]
    fn reflection_by_scaling() {
        let transform = scaling(-1.0, 1.0, 1.0);
        let p = Tuple::new_point(2.0, 3.0, 4.0);
        assert_eq!(transform * p, Tuple::new_point(-2.0, 3.0, 4.0));
    }

    #[test]
    fn rotation_around_x_axis() {
        let p = Tuple::new_point(0.0, 1.0, 0.0);
        let half_quarter = rotation_x(PI / 4.0);
        let full_quarter = rotation_x(PI / 2.0);

        assert_eq!(
            half_quarter * p,
            Tuple::new_point(0.0, (2.0_f64).sqrt() / 2.0, 2.0_f64.sqrt() / 2.0)
        );
        assert_eq!(full_quarter * p, Tuple::new_point(0.0, 0.0, 1.0));
    }

    #[test]
    fn inverse_rotation_around_x_axis() {
        let p = Tuple::new_point(0.0, 1.0, 0.0);
        let half_quarter = rotation_x(PI / 4.0);
        let inv = half_quarter.inverse().unwrap();

        assert_eq!(
            inv * p,
            Tuple::new_point(0.0, (2.0_f64).sqrt() / 2.0, -2.0_f64.sqrt() / 2.0)
        );
    }

    #[test]
    fn rotation_around_y_axis() {
        let p = Tuple::new_point(0.0, 0.0, 1.0);
        let half_quarter = rotation_y(PI / 4.0);
        let full_quarter = rotation_y(PI / 2.0);

        assert_eq!(
            half_quarter * p,
            Tuple::new_point((2.0_f64).sqrt() / 2.0, 0.0, 2.0_f64.sqrt() / 2.0)
        );
        assert_eq!(full_quarter * p, Tuple::new_point(1.0, 0.0, 0.0));
    }

    #[test]
    fn rotation_around_z_axis() {
        let p = Tuple::new_point(0.0, 1.0, 0.0);
        let half_quarter = rotation_z(PI / 4.0);
        let full_quarter = rotation_z(PI / 2.0);

        assert_eq!(
            half_quarter * p,
            Tuple::new_point(-2.0_f64.sqrt() / 2.0, 2.0_f64.sqrt() / 2.0, 0.0)
        );
        assert_eq!(full_quarter * p, Tuple::new_point(-1.0, 0.0, 0.0));
    }

    #[test]
    fn shearing_moves_x_in_proportion_to_y() {
        let p = Tuple::new_point(2.0, 3.0, 4.0);
        let transform = shearing(1.0, 0.0, 0.0, 0.0, 0.0, 0.0);
        assert_eq!(transform * p, Tuple::new_point(5.0, 3.0, 4.0));
    }
    #[test]
    fn shearing_moves_x_in_proportion_to_z() {
        let p = Tuple::new_point(2.0, 3.0, 4.0);
        let transform = shearing(0.0, 1.0, 0.0, 0.0, 0.0, 0.0);
        assert_eq!(transform * p, Tuple::new_point(6.0, 3.0, 4.0));
    }
    #[test]
    fn shearing_moves_y_in_proportion_to_x() {
        let p = Tuple::new_point(2.0, 3.0, 4.0);
        let transform = shearing(0.0, 0.0, 1.0, 0.0, 0.0, 0.0);
        assert_eq!(transform * p, Tuple::new_point(2.0, 5.0, 4.0));
    }
    #[test]
    fn shearing_moves_y_in_proportion_to_z() {
        let p = Tuple::new_point(2.0, 3.0, 4.0);
        let transform = shearing(0.0, 0.0, 0.0, 1.0, 0.0, 0.0);
        assert_eq!(transform * p, Tuple::new_point(2.0, 7.0, 4.0));
    }
    #[test]
    fn shearing_moves_z_in_proportion_to_x() {
        let p = Tuple::new_point(2.0, 3.0, 4.0);
        let transform = shearing(0.0, 0.0, 0.0, 0.0, 1.0, 0.0);
        assert_eq!(transform * p, Tuple::new_point(2.0, 3.0, 6.0));
    }
    #[test]
    fn shearing_moves_z_in_proportion_to_y() {
        let p = Tuple::new_point(2.0, 3.0, 4.0);
        let transform = shearing(0.0, 0.0, 0.0, 0.0, 0.0, 1.0);
        assert_eq!(transform * p, Tuple::new_point(2.0, 3.0, 7.0));
    }

    #[test]
    fn indivisual_transformation_in_sequence() {
        let p = Tuple::new_point(1.0, 0.0, 1.0);
        let a = rotation_x(PI / 2.0);
        let b = scaling(5.0, 5.0, 5.0);
        let c = translation(10.0, 5.0, 7.0);

        let p2 = a * p;
        assert_eq!(p2, Tuple::new_point(1.0, -1.0, 0.0));

        let p3 = b * p2;
        assert_eq!(p3, Tuple::new_point(5.0, -5.0, 0.0));

        let p4 = c * p3;
        assert_eq!(p4, Tuple::new_point(15.0, 0.0, 7.0));
    }

    #[test]
    fn chained_transformations() {
        let p = Tuple::new_point(1.0, 0.0, 1.0);
        let a = rotation_x(PI / 2.0);
        let b = scaling(5.0, 5.0, 5.0);
        let c = translation(10.0, 5.0, 7.0);
        let t = c * b * a;
        assert_eq!(t * p, Tuple::new_point(15.0, 0.0, 7.0));
    }
}
