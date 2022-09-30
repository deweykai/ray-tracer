use crate::ray::Ray;
use crate::sphere::Sphere;
use crate::tuple::{Point, Vector};

#[derive(Debug, Clone, Copy, PartialEq)]
pub struct Intersection<'a> {
    pub t: f64,
    pub object: &'a Sphere,
}

impl<'a> Intersection<'a> {
    pub fn new(t: f64, object: &Sphere) -> Intersection {
        Intersection { t, object }
    }

    pub fn prepare_computations(&self, ray: Ray) -> Computations {
        let t = self.t;
        let point = ray.position(self.t);
        let object = self.object;
        let eyev: Vector = (-ray.direction).try_into().unwrap();
        let mut normal = self.object.normal_at(point);
        let mut inside = false;

        if normal.dot(eyev) < 0.0 {
            inside = true;
            normal = -normal;
        }

        Computations {
            t,
            object,
            point,
            eyev,
            normal,
            inside,
            over_point: point + normal * crate::tuple::EPSILON,
        }
    }
}

pub struct Computations<'a> {
    pub object: &'a Sphere,
    pub t: f64,
    pub point: Point,
    pub eyev: Vector,
    pub normal: Vector,
    pub inside: bool,
    pub over_point: Point,
}

#[derive(Debug, Clone, PartialEq)]
pub struct Intersections<'a>(pub Vec<Intersection<'a>>);

impl<'a> Intersections<'a> {
    pub fn new() -> Intersections<'a> {
        Intersections(Vec::new())
    }

    pub fn push(&mut self, intersection: Intersection<'a>) {
        self.0.push(intersection);
        self.0.sort_by(|a, b| a.t.partial_cmp(&b.t).unwrap());
    }

    pub fn concat(&mut self, mut other: Intersections<'a>) {
        self.0.append(&mut other.0);
        self.0.sort_by(|a, b| a.t.partial_cmp(&b.t).unwrap());
    }

    pub fn hit(&self) -> Option<Intersection> {
        self.0
            .iter()
            .filter(|x| x.t >= 0.0)
            .reduce(|a, b| if a.t < b.t { a } else { b })
            .map(|x| x.clone())
    }
}

impl<'a> From<Vec<Intersection<'a>>> for Intersections<'a> {
    fn from(x: Vec<Intersection<'a>>) -> Intersections {
        Intersections(x)
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::ray::Ray;
    use crate::sphere::Sphere;
    use crate::tuple::{Point, Vector};
    #[test]
    fn intersection_encapsulates_t_and_object() {
        let t = 3.5;
        let s = Sphere::new();
        let intersection = Intersection::new(t, &s);
        assert_eq!(intersection.t, t);
        assert_eq!(intersection.object, &s);
    }
    #[test]
    fn aggregating_intersections() {
        let s = Sphere::new();
        let i1 = Intersection::new(1.0, &s);
        let i2 = Intersection::new(2.0, &s);
        let xs: Intersections = vec![i1, i2].into();

        assert_eq!(xs.0.len(), 2);
        assert_eq!(xs.0[0].object, &s);
        assert_eq!(xs.0[1].object, &s);
    }
    #[test]
    fn intersect_sets_the_object() {
        let r = Ray::new(Point::new(0.0, 0.0, -5.0), Vector::new(0.0, 0.0, 1.0));
        let s = Sphere::new();
        let xs = s.intersect(r).0;

        assert_eq!(xs.len(), 2);
        assert_eq!(xs[0].object, &s);
        assert_eq!(xs[1].object, &s);
    }

    #[test]
    fn hit_all_positive_intersections() {
        let s = Sphere::new();
        let i1 = Intersection::new(1.0, &s);
        let i2 = Intersection::new(2.0, &s);
        let xs: Intersections = vec![i1, i2].into();
        let hit = xs.hit();
        assert_eq!(hit, Some(i1));
    }

    #[test]
    fn hit_some_negative_intersections() {
        let s = Sphere::new();
        let i1 = Intersection::new(-1.0, &s);
        let i2 = Intersection::new(2.0, &s);
        let xs: Intersections = vec![i1, i2].into();
        let hit = xs.hit();
        assert_eq!(hit, Some(i2));
    }

    #[test]
    fn hit_all_negative_intersections() {
        let s = Sphere::new();
        let i1 = Intersection::new(-1.0, &s);
        let i2 = Intersection::new(-2.0, &s);
        let xs: Intersections = vec![i1, i2].into();
        let hit = xs.hit();
        assert_eq!(hit, None);
    }

    #[test]
    fn hit_lowest_nonnegative_intersection() {
        let s = Sphere::new();
        let i1 = Intersection::new(5.0, &s);
        let i2 = Intersection::new(7.0, &s);
        let i3 = Intersection::new(-3.0, &s);
        let i4 = Intersection::new(2.0, &s);
        let xs: Intersections = vec![i1, i2, i3, i4].into();
        let hit = xs.hit();
        assert_eq!(hit, Some(i4));
    }

    #[test]
    fn precompute_state_of_intersection() {
        let r = Ray::new(Point::new(0.0, 0.0, -5.0), Vector::new(0.0, 0.0, 1.0));
        let s = Sphere::new();
        let i = Intersection::new(4.0, &s);
        let comps = i.prepare_computations(r);

        assert_eq!(comps.object, &s);
        assert_eq!(comps.point, Point::new(0.0, 0.0, -1.0));
        assert_eq!(comps.eyev, Vector::new(0.0, 0.0, -1.0));
        assert_eq!(comps.normal, Vector::new(0.0, 0.0, -1.0));
    }

    #[test]
    fn test_hit_on_intersection_on_outside() {
        let r = Ray::new(Point::new(0.0, 0.0, -5.0), Vector::new(0.0, 0.0, 1.0));
        let s = Sphere::new();
        let i = Intersection::new(4.0, &s);
        let comps = i.prepare_computations(r);

        assert!(!comps.inside)
    }
    #[test]
    fn test_hit_on_intersection_on_inside() {
        let r = Ray::new(Point::new(0.0, 0.0, 0.0), Vector::new(0.0, 0.0, 1.0));
        let s = Sphere::new();
        let i = Intersection::new(1.0, &s);
        let comps = i.prepare_computations(r);

        assert_eq!(comps.point, Point::new(0.0, 0.0, 1.0));
        assert_eq!(comps.eyev, Vector::new(0.0, 0.0, -1.0));
        assert_eq!(comps.normal, Vector::new(0.0, 0.0, -1.0));
        assert!(comps.inside)
    }
}
