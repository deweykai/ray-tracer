use crate::tuple::{Point, Tuple, Vector};

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

#[derive(Debug, Clone, Copy)]
pub struct Sphere {
    id: u32,
}

impl Sphere {
    pub fn new(id: u32) -> Sphere {
        Sphere { id }
    }
}

pub struct Intersection {
    t: f64,
    object: u32,
}

pub trait Object {
    fn id(&self) -> u32;
}

impl PartialEq for dyn Object {
    fn eq(&self, other: &Self) -> bool {
        self.id() == other.id()
    }
}

impl Object for Sphere {
    fn id(&self) -> u32 {
        self.id
    }
}

impl Intersection {
    pub fn new<T: Object>(t: f64, object: T) -> Intersection {
        Intersection {
            t,
            object: object.id(),
        }
    }
}

fn intersect(ray: Ray, sphere: Sphere) -> Vec<f64> {
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
    vec![t1, t2]
}

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
        let s = Sphere::new(0);
        let xs = intersect(r, s);
        assert_eq!(xs.len(), 2);
        assert_eq!(xs[0], 4.0);
        assert_eq!(xs[1], 6.0);
    }
    #[test]
    fn ray_intersects_sphere_at_tangent() {
        let r = Ray::new(Point::new(0.0, 1.0, -5.0), Vector::new(0.0, 0.0, 1.0));
        let s = Sphere::new(0);
        let xs = intersect(r, s);
        assert_eq!(xs.len(), 2);
        assert_eq!(xs[0], 5.0);
        assert_eq!(xs[1], 5.0);
    }
    #[test]
    fn ray_misses_sphere() {
        let r = Ray::new(Point::new(0.0, 2.0, -5.0), Vector::new(0.0, 0.0, 1.0));
        let s = Sphere::new(0);
        let xs = intersect(r, s);
        assert_eq!(xs.len(), 0);
    }
    #[test]
    fn ray_originates_inside_sphere() {
        let r = Ray::new(Point::new(0.0, 0.0, 0.0), Vector::new(0.0, 0.0, 1.0));
        let s = Sphere::new(0);
        let xs = intersect(r, s);
        assert_eq!(xs.len(), 2);
        assert_eq!(xs[0], -1.0);
        assert_eq!(xs[1], 1.0);
    }
    #[test]
    fn sphere_is_behind_ray() {
        let r = Ray::new(Point::new(0.0, 0.0, 5.0), Vector::new(0.0, 0.0, 1.0));
        let s = Sphere::new(0);
        let xs = intersect(r, s);
        assert_eq!(xs.len(), 2);
        assert_eq!(xs[0], -6.0);
        assert_eq!(xs[1], -4.0);
    }

    #[test]
    fn intersection_encapsulates_t_and_object() {
        let t = 3.5;
        let s = Sphere::new(0);
        let intersection = Intersection::new(t, s);
        assert_eq!(intersection.t, t);
        assert_eq!(intersection.object, s.id);
    }
}
