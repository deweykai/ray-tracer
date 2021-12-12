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

    pub fn new_point(x: f64, y: f64, z: f64) -> Tuple {
        Tuple::new(x, y, z, 1.0)
    }

    pub fn new_vector(x: f64, y: f64, z: f64) -> Tuple {
        Tuple::new(x, y, z, 0.0)
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

    pub fn cross(self, other: Tuple) -> Tuple {
        Tuple::new_vector(
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
        let point = Tuple::new_point(4., -4., 3.);
        let tuple = Tuple::new(4., -4., 3., 1.);
        assert_eq!(point, tuple);
    }
    #[test]
    fn tuple_vector_constructor() {
        let vector = Tuple::new_vector(4., -4., 3.);
        let tuple = Tuple::new(4., -4., 3., 0.);
        assert_eq!(vector, tuple);
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
        let a = Tuple::new_point(3., 2., 1.);
        let b = Tuple::new_point(5., 6., 7.);
        let diff = Tuple::new_vector(-2., -4., -6.);
        assert_eq!(a - b, diff);
    }
    #[test]
    fn sub_vector_from_point() {
        let p = Tuple::new_point(3., 2., 1.);
        let v = Tuple::new_vector(5., 6., 7.);
        let diff = Tuple::new_point(-2., -4., -6.);
        assert_eq!(p - v, diff);
    }
    #[test]
    fn sub_vector_from_zero() {
        let v = Tuple::new_vector(1., -2., 3.);
        let diff = Tuple::new_vector(-1., 2., -3.);
        assert_eq!(Tuple::zero() - v, diff);
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
        assert_eq!(Tuple::new_vector(1., 0., 0.).magnitude(), 1.);
        assert_eq!(Tuple::new_vector(0., 1., 0.).magnitude(), 1.);
        assert_eq!(Tuple::new_vector(0., 0., 1.).magnitude(), 1.);
        assert_eq!(Tuple::new_vector(1., 2., 3.).magnitude(), 14f64.sqrt());
        assert_eq!(Tuple::new_vector(-1., -2., -3.).magnitude(), 14f64.sqrt());
    }
    #[test]
    fn normalize_vector() {
        assert_eq!(
            Tuple::new_vector(4., 0., 0.).normalize(),
            Tuple::new_vector(1., 0., 0.)
        );
        assert_eq!(
            Tuple::new_vector(1., 2., 3.).normalize(),
            Tuple::new_vector(1. / 14f64.sqrt(), 2. / 14f64.sqrt(), 3. / 14f64.sqrt())
        );
        assert_eq!(Tuple::new_vector(4., 0., 0.).normalize().magnitude(), 1.);
    }
    #[test]
    fn dot_product_two_vectors() {
        let a = Tuple::new_vector(1., 2., 3.);
        let b = Tuple::new_vector(2., 3., 4.);
        assert_eq!(a.dot(b), 20.);
    }
    #[test]
    fn cross_product_two_vectors() {
        let a = Tuple::new_vector(1., 2., 3.);
        let b = Tuple::new_vector(2., 3., 4.);
        assert_eq!(a.cross(b), Tuple::new_vector(-1., 2., -1.));
        assert_eq!(b.cross(a), Tuple::new_vector(1., -2., 1.));
    }
}
