use crate::color::{Color, BLACK};
use crate::light::PointLight;
use crate::tuple::{Point, Vector};

#[derive(Debug, Clone, Copy, PartialEq)]
pub struct Material {
    pub color: Color,
    pub ambient: f64,
    pub diffuse: f64,
    pub specular: f64,
    pub shininess: f64,
}

impl Default for Material {
    fn default() -> Self {
        Material {
            color: Color::new(1.0, 1.0, 1.0),
            ambient: 0.1,
            diffuse: 0.9,
            specular: 0.9,
            shininess: 200.0,
        }
    }
}

pub fn lighting(
    material: Material,
    light: PointLight,
    point: Point,
    eyev: Vector,
    normalv: Vector,
    in_shadow: bool,
) -> Color {
    // combine surface color with light intensity
    let effective_color = material.color * light.intensity;

    // find direction of light source
    let lightv: Vector = (light.position - point).normalize();

    // compute ambient light
    let ambient = effective_color * material.ambient;
    if in_shadow {
        return ambient;
    }

    // light_dote represents cosine of angle between light and normal vector
    // negative means light is on other side of surface
    let light_dot_normal = lightv.dot(normalv);
    let (diffuse, specular) = if light_dot_normal < 0.0 {
        (BLACK, BLACK)
    } else {
        let diffuse = effective_color * material.diffuse * light_dot_normal;

        // reflect dot eye represents the cosine of the angle between the
        // reflection and the eye
        let reflectv = -lightv.reflect(normalv);
        let reflect_dot_eye = reflectv.dot(eyev);
        let specular = if reflect_dot_eye <= 0.0 {
            BLACK
        } else {
            let factor = reflect_dot_eye.powf(material.shininess);
            light.intensity * material.specular * factor
        };
        (diffuse, specular)
    };

    ambient + diffuse + specular
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn default_material() {
        let m: Material = Default::default();
        assert_eq!(m.ambient, 0.1);
        assert_eq!(m.diffuse, 0.9);
        assert_eq!(m.specular, 0.9);
        assert_eq!(m.shininess, 200.0);
    }

    fn background() -> (Material, Point) {
        (Default::default(), Point::new(0.0, 0.0, 0.0))
    }

    #[test]
    fn lighting_eye_between_light_and_surface() {
        let (m, position) = background();
        let eyev = Vector::new(0.0, 0.0, -1.0);
        let normalv = Vector::new(0.0, 0.0, -1.0);
        let light = PointLight::new(Point::new(0.0, 0.0, -10.0), Color::new(1.0, 1.0, 1.0));
        let result = lighting(m, light, position, eyev, normalv, false);
        assert_eq!(result, Color::new(1.9, 1.9, 1.9));
    }

    #[test]
    fn lighting_eye_between_light_and_surface_offset_45deg() {
        let (m, position) = background();
        let eyev = Vector::new(0.0, 2f64.sqrt() / 2.0, -2f64.sqrt() / 2.0);
        let normalv = Vector::new(0.0, 0.0, -1.0);
        let light = PointLight::new(Point::new(0.0, 0.0, -10.0), Color::new(1.0, 1.0, 1.0));
        let result = lighting(m, light, position, eyev, normalv, false);
        assert_eq!(result, Color::new(1.0, 1.0, 1.0));
    }
    #[test]
    fn lighting_with_eye_opposite_surface_light_offset_45deg() {
        let (m, position) = background();
        let eyev = Vector::new(0.0, 0.0, -1.0);
        let normalv = Vector::new(0.0, 0.0, -1.0);
        let light = PointLight::new(Point::new(0.0, 10.0, -10.0), Color::new(1.0, 1.0, 1.0));
        let result = lighting(m, light, position, eyev, normalv, false);
        assert_eq!(result, Color::new(0.7364, 0.7364, 0.7364));
    }
    #[test]
    fn lighting_with_eye_in_the_path_of_reflection() {
        let (m, position) = background();
        let eyev = Vector::new(0.0, -2f64.sqrt() / 2.0, -2f64.sqrt() / 2.0);
        let normalv = Vector::new(0.0, 0.0, -1.0);
        let light = PointLight::new(Point::new(0.0, 10.0, -10.0), Color::new(1.0, 1.0, 1.0));
        let result = lighting(m, light, position, eyev, normalv, false);
        assert_eq!(result, Color::new(1.6364, 1.6364, 1.6364));
    }
    #[test]
    fn lighting_light_behind_surface() {
        let (m, position) = background();
        let eyev = Vector::new(0.0, 0.0, -1.0);
        let normalv = Vector::new(0.0, 0.0, -1.0);
        let light = PointLight::new(Point::new(0.0, 0.0, 10.0), Color::new(1.0, 1.0, 1.0));
        let result = lighting(m, light, position, eyev, normalv, false);
        assert_eq!(result, Color::new(0.1, 0.1, 0.1));
    }
    #[test]
    fn lighting_with_surface_in_shadow() {
        let (m, position) = background();
        let eyev = Vector::new(0.0, 0.0, -1.0);
        let normalv = Vector::new(0.0, 0.0, -1.0);
        let light = PointLight::new(Point::new(0.0, 0.0, -1.0), Color::new(1.0, 1.0, 1.0));
        let in_shadow = true;
        let result = lighting(m, light, position, eyev, normalv, in_shadow);

        assert_eq!(result, Color::new(0.1, 0.1, 0.1));
    }
}
