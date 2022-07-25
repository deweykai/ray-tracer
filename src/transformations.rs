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
    use crate::tuple::{Point, Vector};

    #[test]
    fn multiply_translation_matrix() {
        let transform = translation(5., -3., 2.);
        let p = Point::new(-3., 4., 5.).as_tuple();

        assert_eq!(transform * p, Point::new(2., 1., 7.).as_tuple());
    }

    #[test]
    fn mulitply_inverse_translation_matrix() {
        let transform = translation(5., -3.0, 2.);
        let inv = transform.inverse().unwrap();
        let p = Point::new(-3., 4., 5.).as_tuple();

        assert_eq!(inv * p, Point::new(-8.0, 7.0, 3.0).as_tuple());
    }

    #[test]
    fn multiply_vector_translation_matrix() {
        let transform = translation(5.0, -3.0, 2.0);
        let v = Vector::new(-3.0, 4.0, 5.0).as_tuple();

        assert_eq!(transform * v, v);
    }

    #[test]
    fn multiply_point_by_scaling_matrix() {
        let transform = scaling(2.0, 3.0, 4.0);
        let p = Vector::new(-4.0, 6.0, 8.0).as_tuple();
        assert_eq!(transform * p, Vector::new(-8.0, 18.0, 32.0).as_tuple());
    }

    #[test]
    fn multiply_vector_by_scaling_matrix() {
        let transform = scaling(2.0, 3.0, 4.0);
        let p = Vector::new(-4.0, 6.0, 8.0).as_tuple();
        assert_eq!(transform * p, Vector::new(-8.0, 18.0, 32.0).as_tuple());
    }

    #[test]
    fn mulitply_by_inverse_scaling_matrix() {
        let transform = scaling(2.0, 3.0, 4.0);
        let inv = transform.inverse().unwrap();
        let v = Vector::new(-4.0, 6.0, 8.0).as_tuple();

        assert_eq!(inv * v, Vector::new(-2.0, 2.0, 2.0).as_tuple());
    }

    #[test]
    fn reflection_by_scaling() {
        let transform = scaling(-1.0, 1.0, 1.0);
        let p = Point::new(2.0, 3.0, 4.0).as_tuple();
        assert_eq!(transform * p, Point::new(-2.0, 3.0, 4.0).as_tuple());
    }

    #[test]
    fn rotation_around_x_axis() {
        let p = Point::new(0.0, 1.0, 0.0).as_tuple();
        let half_quarter = rotation_x(PI / 4.0);
        let full_quarter = rotation_x(PI / 2.0);

        assert_eq!(
            half_quarter * p,
            Point::new(0.0, (2.0_f64).sqrt() / 2.0, 2.0_f64.sqrt() / 2.0).as_tuple()
        );
        assert_eq!(full_quarter * p, Point::new(0.0, 0.0, 1.0).as_tuple());
    }

    #[test]
    fn inverse_rotation_around_x_axis() {
        let p = Point::new(0.0, 1.0, 0.0).as_tuple();
        let half_quarter = rotation_x(PI / 4.0);
        let inv = half_quarter.inverse().unwrap();

        assert_eq!(
            inv * p,
            Point::new(0.0, (2.0_f64).sqrt() / 2.0, -2.0_f64.sqrt() / 2.0).as_tuple()
        );
    }

    #[test]
    fn rotation_around_y_axis() {
        let p = Point::new(0.0, 0.0, 1.0).as_tuple();
        let half_quarter = rotation_y(PI / 4.0);
        let full_quarter = rotation_y(PI / 2.0);

        assert_eq!(
            half_quarter * p,
            Point::new((2.0_f64).sqrt() / 2.0, 0.0, 2.0_f64.sqrt() / 2.0).as_tuple()
        );
        assert_eq!(full_quarter * p, Point::new(1.0, 0.0, 0.0).as_tuple());
    }

    #[test]
    fn rotation_around_z_axis() {
        let p = Point::new(0.0, 1.0, 0.0).as_tuple();
        let half_quarter = rotation_z(PI / 4.0);
        let full_quarter = rotation_z(PI / 2.0);

        assert_eq!(
            half_quarter * p,
            Point::new(-2.0_f64.sqrt() / 2.0, 2.0_f64.sqrt() / 2.0, 0.0).as_tuple()
        );
        assert_eq!(full_quarter * p, Point::new(-1.0, 0.0, 0.0).as_tuple());
    }

    #[test]
    fn shearing_moves_x_in_proportion_to_y() {
        let p = Point::new(2.0, 3.0, 4.0).as_tuple();
        let transform = shearing(1.0, 0.0, 0.0, 0.0, 0.0, 0.0);
        assert_eq!(transform * p, Point::new(5.0, 3.0, 4.0).as_tuple());
    }
    #[test]
    fn shearing_moves_x_in_proportion_to_z() {
        let p = Point::new(2.0, 3.0, 4.0).as_tuple();
        let transform = shearing(0.0, 1.0, 0.0, 0.0, 0.0, 0.0);
        assert_eq!(transform * p, Point::new(6.0, 3.0, 4.0).as_tuple());
    }
    #[test]
    fn shearing_moves_y_in_proportion_to_x() {
        let p = Point::new(2.0, 3.0, 4.0).as_tuple();
        let transform = shearing(0.0, 0.0, 1.0, 0.0, 0.0, 0.0);
        assert_eq!(transform * p, Point::new(2.0, 5.0, 4.0).as_tuple());
    }
    #[test]
    fn shearing_moves_y_in_proportion_to_z() {
        let p = Point::new(2.0, 3.0, 4.0).as_tuple();
        let transform = shearing(0.0, 0.0, 0.0, 1.0, 0.0, 0.0);
        assert_eq!(transform * p, Point::new(2.0, 7.0, 4.0).as_tuple());
    }
    #[test]
    fn shearing_moves_z_in_proportion_to_x() {
        let p = Point::new(2.0, 3.0, 4.0).as_tuple();
        let transform = shearing(0.0, 0.0, 0.0, 0.0, 1.0, 0.0);
        assert_eq!(transform * p, Point::new(2.0, 3.0, 6.0).as_tuple());
    }
    #[test]
    fn shearing_moves_z_in_proportion_to_y() {
        let p = Point::new(2.0, 3.0, 4.0).as_tuple();
        let transform = shearing(0.0, 0.0, 0.0, 0.0, 0.0, 1.0);
        assert_eq!(transform * p, Point::new(2.0, 3.0, 7.0).as_tuple());
    }

    #[test]
    fn indivisual_transformation_in_sequence() {
        let p = Point::new(1.0, 0.0, 1.0).as_tuple();
        let a = rotation_x(PI / 2.0);
        let b = scaling(5.0, 5.0, 5.0);
        let c = translation(10.0, 5.0, 7.0);

        let p2 = a * p;
        assert_eq!(p2, Point::new(1.0, -1.0, 0.0).as_tuple());

        let p3 = b * p2;
        assert_eq!(p3, Point::new(5.0, -5.0, 0.0).as_tuple());

        let p4 = c * p3;
        assert_eq!(p4, Point::new(15.0, 0.0, 7.0).as_tuple());
    }

    #[test]
    fn chained_transformations() {
        let p = Point::new(1.0, 0.0, 1.0).as_tuple();
        let a = rotation_x(PI / 2.0);
        let b = scaling(5.0, 5.0, 5.0);
        let c = translation(10.0, 5.0, 7.0);
        let t = c * b * a;
        assert_eq!(t * p, Point::new(15.0, 0.0, 7.0).as_tuple());
    }
}
