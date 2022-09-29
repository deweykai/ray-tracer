use crate::{color::Color, tuple::Point};

#[derive(Debug, Copy, Clone, PartialEq)]
pub struct PointLight {
    pub intensity: Color,
    pub position: Point,
}

impl PointLight {
    pub fn new(position: Point, intensity: Color) -> PointLight {
        PointLight {
            intensity,
            position,
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn point_light_with_features() {
        let intensity = Color::new(0.5, 0.5, 0.5);
        let position = Point::new(0.0, 0.0, 0.0);
        let light = PointLight::new(position, intensity);
        assert_eq!(light.position, position);
        assert_eq!(light.intensity, intensity);
    }
}
