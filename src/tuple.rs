#[derive(Debug, PartialEq, Eq)]
pub struct Tuple {
    x: f64,
    y: f64,
    z: f64,
    w: f64,
}

const EPSILON: f64 = 1e-5

impl Tuple {
    pub fn new(x: f64, y: f64, z: f64, w: f64) -> Tuple {
        Tuple { x, y, z, w }
    }

    pub fn is_point(&self) -> bool {
        self.w.abs_sub(1.0) < EPSILON
    }

    pub fn is_vector(&self) -> bool {
        self.w.abs() < EPSILON
    }
}

#[cfg(test)]
mod tests {
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
}
