use crate::color::Color;
use crate::intersection::Intersections;
use crate::light::PointLight;
use crate::material::Material;
use crate::ray::Ray;
use crate::sphere::Sphere;
use crate::transformations;
use crate::tuple::Point;

pub struct World {
    pub objects: Vec<Sphere>,
    pub lights: Vec<PointLight>,
}

impl World {
    pub fn new() -> World {
        World {
            objects: vec![],
            lights: vec![],
        }
    }

    pub fn add_object(&mut self, sphere: Sphere) {
        self.objects.push(sphere);
    }

    pub fn add_light(&mut self, light: PointLight) {
        self.lights.push(light);
    }

    pub fn intersect(&self, ray: Ray) -> Intersections {
        let mut intersections = Intersections::new();
        for object in &self.objects {
            intersections.concat(object.intersect(ray));
        }
        intersections
    }
}

pub fn default_world() -> World {
    let mut w = World::new();
    w.add_object(Sphere::new().set_material(Material {
        color: Color::new(0.8, 1.0, 0.6),
        diffuse: 0.7,
        specular: 0.2,
        ..Default::default()
    }));
    w.add_object(Sphere::new().set_transform(transformations::scaling(0.5, 0.5, 0.5)));
    w.add_light(PointLight::new(
        Point::new(-10.0, 10.0, -10.0),
        Color::new(1.0, 1.0, 1.0),
    ));

    w
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::tuple::Vector;
    #[test]
    fn creating_world() {
        let w = World::new();
        assert_eq!(w.objects.len(), 0);
        assert_eq!(w.lights.len(), 0);
    }

    #[test]
    fn the_default_world() {
        let w = default_world();
        assert_eq!(w.lights.len(), 1);
        assert_eq!(w.objects.len(), 2);
    }

    #[test]
    fn intersect_world_with_ray() {
        let w = default_world();
        let r = Ray::new(Point::new(0.0, 0.0, -5.0), Vector::new(0.0, 0.0, 1.0));
        let xs = w.intersect(r);
        assert_eq!(xs.0.len(), 4);
        assert_eq!(xs.0[0].t, 4.0);
        assert_eq!(xs.0[1].t, 4.5);
        assert_eq!(xs.0[2].t, 5.5);
        assert_eq!(xs.0[3].t, 6.0);
    }
}
