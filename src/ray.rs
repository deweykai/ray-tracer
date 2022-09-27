use crate::tuple::{Point, Tuple, Vector};
use std::sync::atomic::{AtomicU32, Ordering};

#[derive(Debug, Copy, Clone, PartialEq)]
pub struct Ray {
    origin: Point,
    direction: Vector,
}

impl Ray {
    pub fn new(origin: Point, direction: Vector) -> Ray {
        Ray { origin, direction }
    }

    pub fn position(&self, t: f64) -> Point {
        (Tuple::from(self.origin) + Tuple::from(self.direction) * t)
            .try_into()
            .unwrap()
    }
}

pub trait Object {
    fn id(&self) -> u32;
}

impl PartialEq for dyn Object {
    fn eq(&self, other: &Self) -> bool {
        self.id() == other.id()
    }
}

#[derive(Debug, Clone, Copy)]
pub struct Sphere {
    id: u32,
}

impl Object for Sphere {
    fn id(&self) -> u32 {
        self.id
    }
}

impl Sphere {
    pub fn new() -> Sphere {
        static COUNT: AtomicU32 = AtomicU32::new(0);
        Sphere {
            id: COUNT.fetch_add(1, Ordering::Relaxed),
        }
    }
}

pub struct Intersection {
    t: f64,
    object: Box<dyn Object>,
}

impl Intersection {
    pub fn new<T: Object + 'static>(t: f64, object: T) -> Intersection {
        Intersection {
            t,
            object: Box::new(object),
        }
    }
}

pub fn intersect(sphere: Sphere, ray: Ray) -> Vec<Intersection> {
    let origin = ray.origin.as_tuple();
    let direction = ray.direction.as_tuple();

    let sphere_to_ray = origin - Point::new(0.0, 0.0, 0.0).as_tuple();

    let a = direction.dot(direction);
    let b = 2.0 * direction.dot(sphere_to_ray);
    let c = sphere_to_ray.dot(sphere_to_ray) - 1.0;

    let discriminant = b * b - 4.0 * a * c;
    if discriminant < 0.0 {
        return vec![];
    }

    let t1 = (-b - discriminant.sqrt()) / (2.0 * a);
    let t2 = (-b + discriminant.sqrt()) / (2.0 * a);

    let i1 = Intersection::new(t1, sphere);
    let i2 = Intersection::new(t2, sphere);
    vec![i1, i2]
}

pub struct Intersections(Vec<Intersection>);

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn create_and_query_ray() {
        let origin = Point::new(1.0, 2.0, 3.0);
        let direction = Vector::new(4.0, 5.0, 6.0);
        let ray = Ray::new(origin, direction);

        assert_eq!(ray.origin, origin);
        assert_eq!(ray.direction, direction);
    }

    #[test]
    fn compute_a_point() {
        let r = Ray::new(Point::new(2.0, 3.0, 4.0), Vector::new(1.0, 0.0, 0.0));
        assert_eq!(r.position(0.0), Point::new(2.0, 3.0, 4.0));
        assert_eq!(r.position(1.0), Point::new(3.0, 3.0, 4.0));
        assert_eq!(r.position(-1.0), Point::new(1.0, 3.0, 4.0));
        assert_eq!(r.position(2.5), Point::new(4.5, 3.0, 4.0));
    }

    #[test]
    fn ray_intersects_sphere_at_2_points() {
        let r = Ray::new(Point::new(0.0, 0.0, -5.0), Vector::new(0.0, 0.0, 1.0));
        let s = Sphere::new();
        let xs = intersect(s, r);
        assert_eq!(xs.len(), 2);
        assert_eq!(xs[0].t, 4.0);
        assert_eq!(xs[1].t, 6.0);
    }
    #[test]
    fn ray_intersects_sphere_at_tangent() {
        let r = Ray::new(Point::new(0.0, 1.0, -5.0), Vector::new(0.0, 0.0, 1.0));
        let s = Sphere::new();
        let xs = intersect(s, r);
        assert_eq!(xs.len(), 2);
        assert_eq!(xs[0].t, 5.0);
        assert_eq!(xs[1].t, 5.0);
    }
    #[test]
    fn ray_misses_sphere() {
        let r = Ray::new(Point::new(0.0, 2.0, -5.0), Vector::new(0.0, 0.0, 1.0));
        let s = Sphere::new();
        let xs = intersect(s, r);
        assert_eq!(xs.len(), 0);
    }
    #[test]
    fn ray_originates_inside_sphere() {
        let r = Ray::new(Point::new(0.0, 0.0, 0.0), Vector::new(0.0, 0.0, 1.0));
        let s = Sphere::new();
        let xs = intersect(s, r);
        assert_eq!(xs.len(), 2);
        assert_eq!(xs[0].t, -1.0);
        assert_eq!(xs[1].t, 1.0);
    }
    #[test]
    fn sphere_is_behind_ray() {
        let r = Ray::new(Point::new(0.0, 0.0, 5.0), Vector::new(0.0, 0.0, 1.0));
        let s = Sphere::new();
        let xs = intersect(s, r);
        assert_eq!(xs.len(), 2);
        assert_eq!(xs[0].t, -6.0);
        assert_eq!(xs[1].t, -4.0);
    }
    #[test]
    fn intersection_encapsulates_t_and_object() {
        let t = 3.5;
        let s = Sphere::new();
        let intersection = Intersection::new(t, s);
        assert_eq!(intersection.t, t);
        assert_eq!(intersection.object.id(), s.id());
    }
    #[test]
    fn aggregating_intersections() {
        let s = Sphere::new();
        let i1 = Intersection::new(1.0, s);
        let i2 = Intersection::new(2.0, s);
        let xs = Intersections(vec![i1, i2]);

        assert_eq!(xs.0.len(), 2);
        assert_eq!(xs.0[0].object.id(), s.id());
        assert_eq!(xs.0[1].object.id(), s.id());
    }
    #[test]
    fn intersect_sets_the_object() {
        let r = Ray::new(Point::new(0.0, 0.0, -5.0), Vector::new(0.0, 0.0, 1.0));
        let s = Sphere::new();
        let xs = intersect(s, r);

        assert_eq!(xs.len(), 2);
        assert_eq!(xs[0].object.id(), s.id());
        assert_eq!(xs[1].object.id(), s.id());
    }
}
