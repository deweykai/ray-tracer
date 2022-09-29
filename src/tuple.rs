#[derive(Debug, Copy, Clone)]
pub struct Tuple {
    pub x: f64,
    pub y: f64,
    pub z: f64,
    pub w: f64,
}

const EPSILON: f64 = 1e-5;

impl Tuple {
    pub fn new(x: f64, y: f64, z: f64, w: f64) -> Tuple {
        Tuple { x, y, z, w }
    }

    pub fn zero() -> Tuple {
        Tuple::new(0., 0., 0., 0.)
    }

    pub fn is_point(&self) -> bool {
        (self.w - 1.0).abs() < EPSILON
    }

    pub fn is_vector(&self) -> bool {
        self.w.abs() < EPSILON
    }

    pub fn magnitude(self) -> f64 {
        (self.x.powi(2) + self.y.powi(2) + self.z.powi(2) + self.w.powi(2)).sqrt()
    }

    pub fn normalize(self) -> Tuple {
        self / self.magnitude()
    }

    pub fn dot(self, other: Tuple) -> f64 {
        self.x * other.x + self.y * other.y + self.z * other.z + self.w * other.w
    }

    pub fn cross(self, other: Tuple) -> Vector {
        Vector::new(
            self.y * other.z - self.z * other.y,
            self.z * other.x - self.x * other.z,
            self.x * other.y - self.y * other.x,
        )
    }
}

impl PartialEq for Tuple {
    fn eq(&self, other: &Tuple) -> bool {
        (self.x - other.x).abs() < EPSILON
            && (self.y - other.y).abs() < EPSILON
            && (self.z - other.z).abs() < EPSILON
            && (self.w - other.w).abs() < EPSILON
    }
}

use std::ops::{Add, Div, Mul, Neg, Sub};

impl Add for Tuple {
    type Output = Self;
    fn add(self, other: Tuple) -> Tuple {
        Tuple::new(
            self.x + other.x,
            self.y + other.y,
            self.z + other.z,
            self.w + other.w,
        )
    }
}

impl Sub for Tuple {
    type Output = Self;
    fn sub(self, other: Tuple) -> Tuple {
        Tuple::new(
            self.x - other.x,
            self.y - other.y,
            self.z - other.z,
            self.w - other.w,
        )
    }
}

impl Neg for Tuple {
    type Output = Self;
    fn neg(self) -> Tuple {
        Tuple::zero() - self
    }
}

impl Mul<f64> for Tuple {
    type Output = Self;
    fn mul(self, scalar: f64) -> Tuple {
        Tuple::new(
            self.x * scalar,
            self.y * scalar,
            self.z * scalar,
            self.w * scalar,
        )
    }
}

impl Div<f64> for Tuple {
    type Output = Self;
    fn div(self, scalar: f64) -> Tuple {
        Tuple::new(
            self.x / scalar,
            self.y / scalar,
            self.z / scalar,
            self.w / scalar,
        )
    }
}

#[derive(Debug, Clone, Copy, PartialEq)]
pub struct Vector(pub Tuple);

impl Vector {
    pub fn new(x: f64, y: f64, z: f64) -> Vector {
        Vector(Tuple::new(x, y, z, 0.0))
    }

    pub fn zero() -> Vector {
        Vector::new(0.0, 0.0, 0.0)
    }

    pub fn reflect(self, normal: Vector) -> Vector {
        let inv = self;
        (inv - normal * 2.0 * inv.dot(normal)).try_into().unwrap()
    }

    pub fn normalize(self) -> Vector {
        Tuple::from(self).normalize().try_into().unwrap()
    }

    pub fn cross(self, rhs: Vector) -> Vector {
        Tuple::from(self).cross(rhs.into()).try_into().unwrap()
    }

    pub fn dot(self, rhs: Vector) -> f64 {
        Tuple::from(self).dot(rhs.into()).try_into().unwrap()
    }

    pub fn magnitude(self) -> f64 {
        Tuple::from(self).magnitude()
    }
}

macro_rules! impl_vector_tuple_ops {
    ($trait:ty, $fn:ident, $rhs:ty) => {
        impl $trait for Vector {
            type Output = Vector;
            fn $fn(self, rhs: $rhs) -> Self {
                Tuple::from(self).$fn(rhs.into()).try_into().unwrap()
            }
        }
    };
}

impl_vector_tuple_ops!(Add, add, Vector);
impl_vector_tuple_ops!(Sub, sub, Vector);
impl_vector_tuple_ops!(Mul<f64>, mul, f64);
impl_vector_tuple_ops!(Div<f64>, div, f64);

impl Neg for Vector {
    type Output = Vector;
    fn neg(self) -> Self::Output {
        Tuple::from(self).neg().try_into().unwrap()
    }
}

impl From<Vector> for Tuple {
    fn from(vector: Vector) -> Self {
        vector.0
    }
}

impl TryFrom<Tuple> for Vector {
    type Error = &'static str;

    fn try_from(tuple: Tuple) -> Result<Self, Self::Error> {
        if tuple.is_vector() {
            Ok(Vector(tuple))
        } else {
            Err("failed to convert a tuple into a vector")
        }
    }
}

#[derive(Debug, Clone, Copy, PartialEq)]
pub struct Point(pub Tuple);
impl Point {
    pub fn new(x: f64, y: f64, z: f64) -> Point {
        Point(Tuple::new(x, y, z, 1.0))
    }

    pub fn zero() -> Point {
        Point::new(0.0, 0.0, 0.0)
    }
}

impl From<Point> for Tuple {
    fn from(point: Point) -> Self {
        point.0
    }
}

impl TryFrom<Tuple> for Point {
    type Error = &'static str;

    fn try_from(tuple: Tuple) -> Result<Self, Self::Error> {
        if tuple.is_point() {
            Ok(Point(tuple))
        } else {
            Err("failed to convert a tuple into a point")
        }
    }
}

impl Sub for Point {
    type Output = Vector;
    fn sub(self, rhs: Self) -> Self::Output {
        (Tuple::from(self) - Tuple::from(rhs)).try_into().unwrap()
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn tuple_as_point() {
        let tuple = Tuple::new(4.3, -4.2, 3.1, 1.0);
        assert!(tuple.is_point());
        assert!(!tuple.is_vector());
    }
    #[test]
    fn tuple_as_vector() {
        let tuple = Tuple::new(4.3, -4.2, 3.1, 0.0);
        assert!(!tuple.is_point());
        assert!(tuple.is_vector());
    }
    #[test]
    fn tuple_point_constructor() {
        let point = Point::new(4., -4., 3.);
        let tuple = Tuple::new(4., -4., 3., 1.);
        assert_eq!(Tuple::from(point), tuple);
    }
    #[test]
    fn tuple_vector_constructor() {
        let vector = Vector::new(4., -4., 3.);
        let tuple = Tuple::new(4., -4., 3., 0.);
        assert_eq!(Tuple::from(vector), tuple);
    }
    #[test]
    fn add_two_tuples() {
        let a = Tuple::new(3., -2., 5., 1.0);
        let b = Tuple::new(-2., 3., 1., 0.);
        let expected_sum = Tuple::new(1., 1., 6., 1.);
        assert_eq!(a + b, expected_sum);
    }
    #[test]
    fn sub_two_points() {
        let a = Point::new(3., 2., 1.);
        let b = Point::new(5., 6., 7.);
        let diff = Vector::new(-2., -4., -6.);
        assert_eq!(a - b, diff);
    }
    #[test]
    fn sub_vector_from_point() {
        let p = Tuple::from(Point::new(3., 2., 1.));
        let v = Tuple::from(Vector::new(5., 6., 7.));
        let diff = Point::new(-2., -4., -6.);
        assert_eq!(Tuple::from(p) - Tuple::from(v), Tuple::from(diff));
    }
    #[test]
    fn sub_vector_from_zero() {
        let v = Vector::new(1., -2., 3.);
        let diff = Vector::new(-1., 2., -3.);
        assert_eq!(Vector::zero() - v, diff);
    }
    #[test]
    fn negate_tuple() {
        let a = Tuple::new(1., -2., 3., -4.);
        let neg_a = Tuple::new(-1., 2., -3., 4.);
        assert_eq!(-a, neg_a);
    }
    #[test]
    fn multiply_tuple_by_scalar() {
        let a = Tuple::new(1., -2., 3., -4.);
        let result = Tuple::new(3.5, -7., 10.5, -14.);
        assert_eq!(a * 3.5, result);
    }
    #[test]
    fn multiply_tuple_by_fraction() {
        let a = Tuple::new(1., -2., 3., -4.);
        let result = Tuple::new(0.5, -1., 1.5, -2.);
        assert_eq!(a * 0.5, result);
    }
    #[test]
    fn divide_tuple_by_scalar() {
        let a = Tuple::new(1., -2., 3., -4.);
        let result = Tuple::new(0.5, -1., 1.5, -2.);
        assert_eq!(a / 2., result);
    }
    #[test]
    fn magnitude_of_vector() {
        assert_eq!(Vector::new(1., 0., 0.).magnitude(), 1.);
        assert_eq!(Vector::new(0., 1., 0.).magnitude(), 1.);
        assert_eq!(Vector::new(0., 0., 1.).magnitude(), 1.);
        assert_eq!(Vector::new(1., 2., 3.).magnitude(), 14f64.sqrt());
        assert_eq!(Vector::new(-1., -2., -3.).magnitude(), 14f64.sqrt());
    }
    #[test]
    fn normalize_vector() {
        assert_eq!(Vector::new(4., 0., 0.).normalize(), Vector::new(1., 0., 0.));
        assert_eq!(
            Vector::new(1., 2., 3.).normalize(),
            Vector::new(1. / 14f64.sqrt(), 2. / 14f64.sqrt(), 3. / 14f64.sqrt())
        );
        assert_eq!(Vector::new(4., 0., 0.).normalize().magnitude(), 1.);
    }
    #[test]
    fn dot_product_two_vectors() {
        let a = Vector::new(1., 2., 3.);
        let b = Vector::new(2., 3., 4.);
        assert_eq!(a.dot(b), 20.);
    }
    #[test]
    fn cross_product_two_vectors() {
        let a = Vector::new(1., 2., 3.);
        let b = Vector::new(2., 3., 4.);
        assert_eq!(a.cross(b), Vector::new(-1., 2., -1.));
        assert_eq!(b.cross(a), Vector::new(1., -2., 1.));
    }

    #[test]
    fn reflecting_a_vector_at_45_deg() {
        let v = Vector::new(1.0, -1.0, 0.0);
        let n = Vector::new(0.0, 1.0, 0.0);
        let r = v.reflect(n);
        assert_eq!(r, Vector::new(1.0, 1.0, 0.0));
    }

    #[test]
    fn reflecting_vector_off_a_slanted_surface() {
        let v = Vector::new(0.0, -1.0, 0.0);
        let n = Vector::new(2f64.sqrt() / 2f64, 2f64.sqrt() / 2f64, 0f64);
        let r = v.reflect(n);
        assert_eq!(r, Vector::new(1.0, 0.0, 0.0));
    }
}
