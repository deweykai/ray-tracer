use crate::{color::Color, tuple::Point};

#[derive(Debug)]
pub struct Light {
    pub intensity: Color,
    pub position: Point,
}

impl Light {
    pub fn new(intensity: Color, position: Point) -> Light {
        Light {
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
        let light = Light::new(intensity, position);
        assert_eq!(light.position, position);
        assert_eq!(light.intensity, intensity);
    }
}
