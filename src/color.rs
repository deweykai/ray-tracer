#[derive(Debug, Copy, Clone)]
pub struct Color {
    pub red: f64,
    pub green: f64,
    pub blue: f64,
}

impl Color {
    pub fn new(red: f64, green: f64, blue: f64) -> Color {
        Color { red, green, blue }
    }

    pub fn to_string(&self) -> String {
        fn to255(f: f64) -> u32 {
            (f * 256.).clamp(0., 255.) as u32
        }
        format!(
            "{} {} {}",
            to255(self.red),
            to255(self.green),
            to255(self.blue)
        )
    }
}

const EPSILON: f64 = 1e-5;

impl PartialEq for Color {
    fn eq(&self, other: &Color) -> bool {
        (self.red - other.red).abs() < EPSILON
            && (self.green - other.green).abs() < EPSILON
            && (self.blue - other.blue).abs() < EPSILON
    }
}

use std::ops::{Add, Mul, Sub};
impl Add for Color {
    type Output = Self;
    fn add(self, other: Color) -> Color {
        Color::new(
            self.red + other.red,
            self.green + other.green,
            self.blue + other.blue,
        )
    }
}

impl Sub for Color {
    type Output = Self;
    fn sub(self, other: Color) -> Color {
        Color::new(
            self.red - other.red,
            self.green - other.green,
            self.blue - other.blue,
        )
    }
}

impl Mul<f64> for Color {
    type Output = Self;
    fn mul(self, scalar: f64) -> Color {
        Color::new(self.red * scalar, self.green * scalar, self.blue * scalar)
    }
}

impl Mul<Color> for Color {
    type Output = Self;
    fn mul(self, other: Color) -> Color {
        Color::new(
            self.red * other.red,
            self.green * other.green,
            self.blue * other.blue,
        )
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn add_color() {
        let c1 = Color::new(0.9, 0.6, 0.75);
        let c2 = Color::new(0.7, 0.1, 0.25);
        assert_eq!(c1 + c2, Color::new(1.6, 0.7, 1.0));
    }
    #[test]
    fn sub_color() {
        let c1 = Color::new(0.9, 0.6, 0.75);
        let c2 = Color::new(0.7, 0.1, 0.25);
        assert_eq!(c1 - c2, Color::new(0.2, 0.5, 0.5));
    }
    #[test]
    fn mul_color_by_scalar() {
        let c = Color::new(0.2, 0.3, 0.4);
        assert_eq!(c * 2., Color::new(0.4, 0.6, 0.8));
    }
    #[test]
    fn mul_color_by_color() {
        let c1 = Color::new(1.0, 0.2, 0.4);
        let c2 = Color::new(0.9, 1.0, 0.1);
        assert_eq!(c1 * c2, Color::new(0.9, 0.2, 0.04));
    }
}
