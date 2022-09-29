use crate::intersection::{Intersection, Intersections};
use crate::material::Material;
use crate::matrix::Matrix;
use crate::ray::Ray;
use crate::tuple::{Point, Vector};
use std::sync::atomic::{AtomicU32, Ordering};

#[derive(Debug, PartialEq)]
pub struct Sphere {
    id: u32,
    pub transform: Matrix,
    pub inv_transform: Matrix,
    pub material: Material,
}

impl Sphere {
    pub fn new() -> Sphere {
        static COUNT: AtomicU32 = AtomicU32::new(0);
        Sphere {
            id: COUNT.fetch_add(1, Ordering::Relaxed),
            transform: Matrix::identity(4),
            inv_transform: Matrix::identity(4),
            material: Default::default(),
        }
    }

    pub fn set_transform(mut self, transform: Matrix) -> Sphere {
        self.inv_transform = transform
            .inverse()
            .expect("Fail to inverse sphere transform");
        self.transform = transform;
        self
    }

    pub fn set_material(mut self, material: Material) -> Sphere {
        self.material = material;
        self
    }

    pub fn intersect(&self, ray: Ray) -> Intersections {
        let ray = ray.transform(&self.inv_transform);
        let origin = ray.origin;
        let direction = ray.direction;

        let sphere_to_ray = origin - Point::new(0.0, 0.0, 0.0);

        let a = direction.dot(direction);
        let b = 2.0 * direction.dot(sphere_to_ray);
        let c = sphere_to_ray.dot(sphere_to_ray) - 1.0;

        let discriminant = b * b - 4.0 * a * c;
        if discriminant < 0.0 {
            return Intersections::new();
        }

        let t1 = (-b - discriminant.sqrt()) / (2.0 * a);
        let t2 = (-b + discriminant.sqrt()) / (2.0 * a);

        let i1 = Intersection::new(t1, self);
        let i2 = Intersection::new(t2, self);
        vec![i1, i2].into()
    }

    pub fn normal_at(&self, world_p: Point) -> Vector {
        let object_p = &self.inv_transform * world_p;

        let object_normal = Point::try_from(object_p).unwrap() - Point::new(0.0, 0.0, 0.0);

        let mut world_normal = &self.inv_transform.transpose() * object_normal;
        // something something about multiplying by the inverse
        // of 3x3 submatrix of transform which can be skipped by
        // setting w to 0.
        world_normal.w = 0.0;
        world_normal.normalize().try_into().unwrap()
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::transformations::{self, scaling, translation};
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

    #[test]
    fn normal_on_sphere_x_axis() {
        let s = Sphere::new();
        let n = s.normal_at(Point::new(1.0, 0.0, 0.0));
        assert_eq!(n, Vector::new(1.0, 0.0, 0.0));
    }

    #[test]
    fn normal_on_sphere_y_axis() {
        let s = Sphere::new();
        let n = s.normal_at(Point::new(0.0, 1.0, 0.0));
        assert_eq!(n, Vector::new(0.0, 1.0, 0.0));
    }

    #[test]
    fn normal_on_sphere_z_axis() {
        let s = Sphere::new();
        let n = s.normal_at(Point::new(0.0, 0.0, 1.0));
        assert_eq!(n, Vector::new(0.0, 0.0, 1.0));
    }

    #[test]
    fn normal_on_sphere_not_at_axis() {
        let s = Sphere::new();
        let n = s.normal_at(Point::new(
            3f64.sqrt() / 3.0,
            3f64.sqrt() / 3.0,
            3f64.sqrt() / 3.0,
        ));
        assert_eq!(
            n,
            Vector::new(3f64.sqrt() / 3.0, 3f64.sqrt() / 3.0, 3f64.sqrt() / 3.0,)
        );
    }

    #[test]
    fn normal_on_translated_sphere() {
        let s = Sphere::new().set_transform(translation(0.0, 1.0, 0.0));
        let n = s.normal_at(Point::new(0.0, 1.70711, -0.70711));
        assert_eq!(n, Vector::new(0.0, 0.70711, -0.70711));
    }

    #[test]
    fn normal_on_transformed_sphere() {
        let m = scaling(1.0, 0.5, 1.0) * transformations::rotation_z(std::f64::consts::PI / 5.0);
        let s = Sphere::new().set_transform(m);
        let n = s.normal_at(Point::new(0.0, 2f64.sqrt() / 2.0, -2f64.sqrt() / 2.0));
        assert_eq!(n, Vector::new(0.0, 0.97014, -0.24254));
    }

    #[test]
    fn sphere_has_default_material() {
        let s = Sphere::new();
        assert_eq!(s.material, Default::default());
    }

    #[test]
    fn sphere_assigned_material() {
        let m = Material {
            ambient: 1.0,
            ..Default::default()
        };
        let s = Sphere::new().set_material(m);
        assert_eq!(s.material, m);
    }
}
