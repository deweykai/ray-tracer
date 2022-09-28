use crate::intersection::{Intersection, Intersections};
use crate::matrix::Matrix;
use crate::ray::Ray;
use crate::tuple::Point;
use std::sync::atomic::{AtomicU32, Ordering};

#[derive(Debug, PartialEq)]
pub struct Sphere {
    id: u32,
    transform: Matrix,
}

impl Sphere {
    pub fn new() -> Sphere {
        static COUNT: AtomicU32 = AtomicU32::new(0);
        Sphere {
            id: COUNT.fetch_add(1, Ordering::Relaxed),
            transform: Matrix::identity(4),
        }
    }

    pub fn set_transform(mut self, transform: Matrix) -> Sphere {
        self.transform = transform;
        self
    }

    pub fn intersect(&self, ray: Ray) -> Intersections {
        let ray = ray.transform(
            &self
                .transform
                .inverse()
                .expect("could not inverse ray transform"),
        );
        let origin = ray.origin.as_tuple();
        let direction = ray.direction.as_tuple();

        let sphere_to_ray = origin - Point::new(0.0, 0.0, 0.0).as_tuple();

        let a = direction.dot(direction);
        let b = 2.0 * direction.dot(sphere_to_ray);
        let c = sphere_to_ray.dot(sphere_to_ray) - 1.0;

        let discriminant = b * b - 4.0 * a * c;
        if discriminant < 0.0 {
            return Intersections::new(&[]);
        }

        let t1 = (-b - discriminant.sqrt()) / (2.0 * a);
        let t2 = (-b + discriminant.sqrt()) / (2.0 * a);

        let i1 = Intersection::new(t1, self);
        let i2 = Intersection::new(t2, self);
        Intersections::new(&[i1, i2])
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::transformations::{scaling, translation};
    use crate::tuple::Vector;

    #[test]
    fn ray_intersects_sphere_at_2_points() {
        let r = Ray::new(Point::new(0.0, 0.0, -5.0), Vector::new(0.0, 0.0, 1.0));
        let s = Sphere::new();
        let xs = s.intersect(r).0;
        assert_eq!(xs.len(), 2);
        assert_eq!(xs[0].t, 4.0);
        assert_eq!(xs[1].t, 6.0);
    }
    #[test]
    fn ray_intersects_sphere_at_tangent() {
        let r = Ray::new(Point::new(0.0, 1.0, -5.0), Vector::new(0.0, 0.0, 1.0));
        let s = Sphere::new();
        let xs = s.intersect(r).0;
        assert_eq!(xs.len(), 2);
        assert_eq!(xs[0].t, 5.0);
        assert_eq!(xs[1].t, 5.0);
    }
    #[test]
    fn ray_misses_sphere() {
        let r = Ray::new(Point::new(0.0, 2.0, -5.0), Vector::new(0.0, 0.0, 1.0));
        let s = Sphere::new();
        let xs = s.intersect(r).0;
        assert_eq!(xs.len(), 0);
    }
    #[test]
    fn ray_originates_inside_sphere() {
        let r = Ray::new(Point::new(0.0, 0.0, 0.0), Vector::new(0.0, 0.0, 1.0));
        let s = Sphere::new();
        let xs = s.intersect(r).0;
        assert_eq!(xs.len(), 2);
        assert_eq!(xs[0].t, -1.0);
        assert_eq!(xs[1].t, 1.0);
    }
    #[test]
    fn sphere_is_behind_ray() {
        let r = Ray::new(Point::new(0.0, 0.0, 5.0), Vector::new(0.0, 0.0, 1.0));
        let s = Sphere::new();
        let xs = s.intersect(r).0;
        assert_eq!(xs.len(), 2);
        assert_eq!(xs[0].t, -6.0);
        assert_eq!(xs[1].t, -4.0);
    }

    #[test]
    fn sphere_default_transformation() {
        let s = Sphere::new();
        assert_eq!(s.transform, Matrix::identity(4));
    }

    #[test]
    fn change_sphere_transform() {
        let t = translation(2.0, 3.0, 4.0);
        let s = Sphere::new().set_transform(t.clone());
        assert_eq!(s.transform, t);
    }

    #[test]
    fn intersecting_scaled_sphere_with_ray() {
        let r = Ray::new(Point::new(0.0, 0.0, -5.0), Vector::new(0.0, 0.0, 1.0));
        let s = Sphere::new().set_transform(scaling(2.0, 2.0, 2.0));
        let xs = s.intersect(r).0;

        assert_eq!(xs.len(), 2);
        assert_eq!(xs[0].t, 3.0);
        assert_eq!(xs[1].t, 7.0);
    }

    #[test]
    fn intersecting_translated_sphere_with_ray() {
        let r = Ray::new(Point::new(0.0, 0.0, -5.0), Vector::new(0.0, 0.0, 1.0));
        let s = Sphere::new().set_transform(translation(5.0, 0.0, 0.0));
        let xs = s.intersect(r).0;

        assert_eq!(xs.len(), 0);
    }
}
