use crate::sphere::Sphere;

#[derive(Debug, Clone, Copy, PartialEq)]
pub struct Intersection {
    pub t: f64,
    pub object: Sphere,
}

impl Intersection {
    pub fn new(t: f64, object: Sphere) -> Intersection {
        Intersection { t, object }
    }
}

#[derive(Debug, Clone, PartialEq)]
pub struct Intersections(Vec<Intersection>);

impl Intersections {
    pub fn new(xs: &[Intersection]) -> Intersections {
        Intersections(xs.to_vec())
    }

    pub fn hit(&self) -> Option<Intersection> {
        self.0
            .iter()
            .filter(|x| x.t >= 0.0)
            .reduce(|a, b| if a.t < b.t { a } else { b })
            .map(|x| x.clone())
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
        let intersection = Intersection::new(t, s);
        assert_eq!(intersection.t, t);
        assert_eq!(intersection.object, s);
    }
    #[test]
    fn aggregating_intersections() {
        let s = Sphere::new();
        let i1 = Intersection::new(1.0, s);
        let i2 = Intersection::new(2.0, s);
        let xs = Intersections::new(&[i1, i2]);

        assert_eq!(xs.0.len(), 2);
        assert_eq!(xs.0[0].object, s);
        assert_eq!(xs.0[1].object, s);
    }
    #[test]
    fn intersect_sets_the_object() {
        let r = Ray::new(Point::new(0.0, 0.0, -5.0), Vector::new(0.0, 0.0, 1.0));
        let s = Sphere::new();
        let xs = s.intersect(r);

        assert_eq!(xs.len(), 2);
        assert_eq!(xs[0].object, s);
        assert_eq!(xs[1].object, s);
    }

    #[test]
    fn hit_all_positive_intersections() {
        let s = Sphere::new();
        let i1 = Intersection::new(1.0, s);
        let i2 = Intersection::new(2.0, s);
        let xs = Intersections::new(&[i1, i2]);
        let hit = xs.hit();
        assert_eq!(hit, Some(i1));
    }

    #[test]
    fn hit_some_negative_intersections() {
        let s = Sphere::new();
        let i1 = Intersection::new(-1.0, s);
        let i2 = Intersection::new(2.0, s);
        let xs = Intersections::new(&[i1, i2]);
        let hit = xs.hit();
        assert_eq!(hit, Some(i2));
    }

    #[test]
    fn hit_all_negative_intersections() {
        let s = Sphere::new();
        let i1 = Intersection::new(-1.0, s);
        let i2 = Intersection::new(-2.0, s);
        let xs = Intersections::new(&[i1, i2]);
        let hit = xs.hit();
        assert_eq!(hit, None);
    }

    #[test]
    fn hit_lowest_nonnegative_intersection() {
        let s = Sphere::new();
        let i1 = Intersection::new(5.0, s);
        let i2 = Intersection::new(7.0, s);
        let i3 = Intersection::new(-3.0, s);
        let i4 = Intersection::new(2.0, s);
        let xs = Intersections::new(&[i1, i2, i3, i4]);
        let hit = xs.hit();
        assert_eq!(hit, Some(i4));
    }
}
