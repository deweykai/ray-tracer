use crate::matrix;
use crate::matrix::Matrix4;
use crate::tuple::{Point, Vector};

pub fn translation(x: f64, y: f64, z: f64) -> Matrix4 {
    matrix!([1, 0, 0, x], [0, 1, 0, y], [0, 0, 1, z], [0, 0, 0, 1])
}

pub fn scaling(x: f64, y: f64, z: f64) -> Matrix4 {
    matrix!([x, 0, 0, 0], [0, y, 0, 0], [0, 0, z, 0], [0, 0, 0, 1])
}

pub fn rotation_x(r: f64) -> Matrix4 {
    matrix!(
        [1.0, 0.0, 0.0, 0.0],
        [0.0, r.cos(), -r.sin(), 0.0],
        [0.0, r.sin(), r.cos(), 0.0],
        [0.0, 0.0, 0.0, 1.0]
    )
}

pub fn rotation_y(r: f64) -> Matrix4 {
    matrix!(
        [r.cos(), 0.0, r.sin(), 0.0],
        [0.0, 1.0, 0.0, 0.0],
        [-r.sin(), 0.0, r.cos(), 0.0],
        [0.0, 0.0, 0.0, 1.0]
    )
}

pub fn rotation_z(r: f64) -> Matrix4 {
    matrix!(
        [r.cos(), -r.sin(), 0.0, 0.0],
        [r.sin(), r.cos(), 0.0, 0.0],
        [0.0, 0.0, 1.0, 0.0],
        [0.0, 0.0, 0.0, 1.0]
    )
}

pub fn shearing(x_y: f64, x_z: f64, y_x: f64, y_z: f64, z_x: f64, z_y: f64) -> Matrix4 {
    matrix!(
        [1.0, x_y, x_z, 0.0],
        [y_x, 1.0, y_z, 0.0],
        [z_x, z_y, 1.0, 0.0],
        [0.0, 0.0, 0.0, 1.0]
    )
}

pub fn view_transform(from: Point, to: Point, up: Vector) -> Matrix4 {
    let forward = (to - from).normalize();
    let left = forward.cross(up.normalize());
    let true_up = left.cross(forward);

    let orientation = matrix![
        [left.0.x, left.0.y, left.0.z, 0.0],
        [true_up.0.x, true_up.0.y, true_up.0.z, 0.0],
        [-forward.0.x, -forward.0.y, -forward.0.z, 0.0],
        [0.0, 0.0, 0.0, 1.0],
    ];

    orientation * translation(-from.0.x, -from.0.y, -from.0.z)
}
#[cfg(test)]
mod tests {
    use std::f64::consts::PI;

    use super::*;
    use crate::tuple::{Point, Vector};

    #[test]
    fn multiply_translation_matrix() {
        let transform = translation(5., -3., 2.);
        let p = Point::new(-3., 4., 5.);

        assert_eq!(Point::try_from(transform * p), Ok(Point::new(2., 1., 7.)));
    }

    #[test]
    fn mulitply_inverse_translation_matrix() {
        let transform = translation(5., -3.0, 2.);
        let inv = transform.inverse().unwrap();
        let p = Point::new(-3., 4., 5.);

        assert_eq!(Point::try_from(inv * p), Ok(Point::new(-8.0, 7.0, 3.0)));
    }

    #[test]
    fn multiply_vector_translation_matrix() {
        let transform = translation(5.0, -3.0, 2.0);
        let v = Vector::new(-3.0, 4.0, 5.0);

        assert_eq!(Vector::try_from(transform * v), Ok(v));
    }

    #[test]
    fn multiply_point_by_scaling_matrix() {
        let transform = scaling(2.0, 3.0, 4.0);
        let p = Vector::new(-4.0, 6.0, 8.0);
        assert_eq!(
            Vector::try_from(transform * p),
            Ok(Vector::new(-8.0, 18.0, 32.0))
        );
    }

    #[test]
    fn multiply_vector_by_scaling_matrix() {
        let transform = scaling(2.0, 3.0, 4.0);
        let p = Vector::new(-4.0, 6.0, 8.0);
        assert_eq!(
            Vector::try_from(transform * p),
            Ok(Vector::new(-8.0, 18.0, 32.0))
        );
    }

    #[test]
    fn mulitply_by_inverse_scaling_matrix() {
        let transform = scaling(2.0, 3.0, 4.0);
        let inv = transform.inverse().unwrap();
        let v = Vector::new(-4.0, 6.0, 8.0);

        assert_eq!(Vector::try_from(inv * v), Ok(Vector::new(-2.0, 2.0, 2.0)));
    }

    #[test]
    fn reflection_by_scaling() {
        let transform = scaling(-1.0, 1.0, 1.0);
        let p = Point::new(2.0, 3.0, 4.0);
        assert_eq!(
            Point::try_from(transform * p),
            Ok(Point::new(-2.0, 3.0, 4.0))
        );
    }

    #[test]
    fn rotation_around_x_axis() {
        let p = Point::new(0.0, 1.0, 0.0);
        let half_quarter = rotation_x(PI / 4.0);
        let full_quarter = rotation_x(PI / 2.0);

        assert_eq!(
            Point::try_from(half_quarter * p).unwrap(),
            Point::new(0.0, (2.0_f64).sqrt() / 2.0, 2.0_f64.sqrt() / 2.0)
        );
        assert_eq!(
            Point::try_from(full_quarter * p).unwrap(),
            Point::new(0.0, 0.0, 1.0)
        );
    }

    #[test]
    fn inverse_rotation_around_x_axis() {
        let p = Point::new(0.0, 1.0, 0.0);
        let half_quarter = rotation_x(PI / 4.0);
        let inv = half_quarter.inverse().unwrap();

        assert_eq!(
            Point::try_from(inv * p).unwrap(),
            Point::new(0.0, (2.0_f64).sqrt() / 2.0, -2.0_f64.sqrt() / 2.0)
        );
    }

    #[test]
    fn rotation_around_y_axis() {
        let p = Point::new(0.0, 0.0, 1.0);
        let half_quarter = rotation_y(PI / 4.0);
        let full_quarter = rotation_y(PI / 2.0);

        assert_eq!(
            Point::try_from(half_quarter * p).unwrap(),
            Point::new((2.0_f64).sqrt() / 2.0, 0.0, 2.0_f64.sqrt() / 2.0)
        );
        assert_eq!(
            Point::try_from(full_quarter * p).unwrap(),
            Point::new(1.0, 0.0, 0.0)
        );
    }

    #[test]
    fn rotation_around_z_axis() {
        let p = Point::new(0.0, 1.0, 0.0);
        let half_quarter = rotation_z(PI / 4.0);
        let full_quarter = rotation_z(PI / 2.0);

        assert_eq!(
            Point::try_from(half_quarter * p).unwrap(),
            Point::new(-2.0_f64.sqrt() / 2.0, 2.0_f64.sqrt() / 2.0, 0.0)
        );
        assert_eq!(
            Point::try_from(full_quarter * p).unwrap(),
            Point::new(-1.0, 0.0, 0.0)
        );
    }

    #[test]
    fn shearing_moves_x_in_proportion_to_y() {
        let p = Point::new(2.0, 3.0, 4.0);
        let transform = shearing(1.0, 0.0, 0.0, 0.0, 0.0, 0.0);
        assert_eq!(
            Point::try_from(transform * p).unwrap(),
            Point::new(5.0, 3.0, 4.0)
        );
    }
    #[test]
    fn shearing_moves_x_in_proportion_to_z() {
        let p = Point::new(2.0, 3.0, 4.0);
        let transform = shearing(0.0, 1.0, 0.0, 0.0, 0.0, 0.0);
        assert_eq!(
            Point::try_from(transform * p).unwrap(),
            Point::new(6.0, 3.0, 4.0)
        );
    }
    #[test]
    fn shearing_moves_y_in_proportion_to_x() {
        let p = Point::new(2.0, 3.0, 4.0);
        let transform = shearing(0.0, 0.0, 1.0, 0.0, 0.0, 0.0);
        assert_eq!(
            Point::try_from(transform * p).unwrap(),
            Point::new(2.0, 5.0, 4.0)
        );
    }
    #[test]
    fn shearing_moves_y_in_proportion_to_z() {
        let p = Point::new(2.0, 3.0, 4.0);
        let transform = shearing(0.0, 0.0, 0.0, 1.0, 0.0, 0.0);
        assert_eq!(
            Point::try_from(transform * p).unwrap(),
            Point::new(2.0, 7.0, 4.0)
        );
    }
    #[test]
    fn shearing_moves_z_in_proportion_to_x() {
        let p = Point::new(2.0, 3.0, 4.0);
        let transform = shearing(0.0, 0.0, 0.0, 0.0, 1.0, 0.0);
        assert_eq!(
            Point::try_from(transform * p).unwrap(),
            Point::new(2.0, 3.0, 6.0)
        );
    }
    #[test]
    fn shearing_moves_z_in_proportion_to_y() {
        let p = Point::new(2.0, 3.0, 4.0);
        let transform = shearing(0.0, 0.0, 0.0, 0.0, 0.0, 1.0);
        assert_eq!(
            Point::try_from(transform * p).unwrap(),
            Point::new(2.0, 3.0, 7.0)
        );
    }

    #[test]
    fn indivisual_transformation_in_sequence() {
        let p = Point::new(1.0, 0.0, 1.0);
        let a = rotation_x(PI / 2.0);
        let b = scaling(5.0, 5.0, 5.0);
        let c = translation(10.0, 5.0, 7.0);

        let p2 = a * p;
        assert_eq!(Point::try_from(p2).unwrap(), Point::new(1.0, -1.0, 0.0));

        let p3 = b * p2;
        assert_eq!(Point::try_from(p3).unwrap(), Point::new(5.0, -5.0, 0.0));

        let p4 = c * p3;
        assert_eq!(Point::try_from(p4).unwrap(), Point::new(15.0, 0.0, 7.0));
    }

    #[test]
    fn chained_transformations() {
        let p = Point::new(1.0, 0.0, 1.0);
        let a = rotation_x(PI / 2.0);
        let b = scaling(5.0, 5.0, 5.0);
        let c = translation(10.0, 5.0, 7.0);
        let t = c * b * a;
        assert_eq!(Point::try_from(t * p).unwrap(), Point::new(15.0, 0.0, 7.0));
    }
    #[test]
    fn transformation_matrix_for_default_orientation() {
        let from = Point::new(0.0, 0.0, 0.0);
        let to = Point::new(0.0, 0.0, -1.0);
        let up = Vector::new(0.0, 1.0, 0.0);

        let t = view_transform(from, to, up);
        assert_eq!(t, Matrix4::identity(4));
    }

    #[test]
    fn transformation_matrix_looking_in_positive_z_direction() {
        let from = Point::new(0.0, 0.0, 0.0);
        let to = Point::new(0.0, 0.0, 1.0);
        let up = Vector::new(0.0, 1.0, 0.0);

        let t = view_transform(from, to, up);
        assert_eq!(t, scaling(-1.0, 1.0, -1.0));
    }

    #[test]
    fn view_transformation_moves_the_world() {
        let from = Point::new(0.0, 0.0, 8.0);
        let to = Point::new(0.0, 0.0, 0.0);
        let up = Vector::new(0.0, 1.0, 0.0);

        let t = view_transform(from, to, up);
        assert_eq!(t, translation(0.0, 0.0, -8.0));
    }

    #[test]
    fn arbitrary_view_transformation() {
        let from = Point::new(1.0, 3.0, 2.0);
        let to = Point::new(4.0, -2.0, 8.0);
        let up = Vector::new(1.0, 1.0, 0.0);

        let t = view_transform(from, to, up);
        assert_eq!(
            t,
            matrix![
                [-0.50709, 0.50709, 0.67612, -2.36643],
                [0.76772, 0.60609, 0.12122, -2.82843],
                [-0.35857, 0.59761, -0.71714, 0.00000],
                [0.00000, 0.00000, 0.00000, 1.00000],
            ]
        )
    }
}
