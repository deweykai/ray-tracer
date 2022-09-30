use crate::color::{Color, BLACK};
use crate::intersection::{Computations, Intersections};
use crate::light::PointLight;
use crate::material::{lighting, Material};
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

    pub fn intersect(&self, ray: Ray) -> Intersections {
        let mut intersections = Intersections::new();
        for object in &self.objects {
            intersections.concat(object.intersect(ray));
        }
        intersections
    }

    pub fn shade_hit(&self, comp: Computations) -> Color {
        let mut c = Color::new(0.0, 0.0, 0.0);
        for light in &self.lights {
            c = c + lighting(
                comp.object.material,
                *light,
                comp.point,
                comp.eyev,
                comp.normal,
                false,
            )
        }
        c
    }

    pub fn color_at(&self, ray: Ray) -> Color {
        let inters = self.intersect(ray);
        if let Some(hit) = inters.hit() {
            let comps = hit.prepare_computations(ray);
            self.shade_hit(comps)
        } else {
            BLACK
        }
    }

    pub fn is_shadowed(&self, point: Point) -> bool {
        for light in &self.lights {
            let v = light.position - point;
            let distance = v.magnitude();
            let direction = v.normalize();

            let r = Ray::new(point, direction);
            let intersections = self.intersect(r);

            if let Some(hit) = intersections.hit() {
                if hit.t >= distance {
                    return false;
                }
            } else {
                return false;
            }
        }

        true
    }
}

pub fn default_world() -> World {
    let mut w = World::new();
    w.objects.push(Sphere::new().set_material(Material {
        color: Color::new(0.8, 1.0, 0.6),
        diffuse: 0.7,
        specular: 0.2,
        ..Default::default()
    }));
    w.objects
        .push(Sphere::new().set_transform(transformations::scaling(0.5, 0.5, 0.5)));
    w.lights.push(PointLight::new(
        Point::new(-10.0, 10.0, -10.0),
        Color::new(1.0, 1.0, 1.0),
    ));

    w
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::{intersection::Intersection, tuple::Vector};
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

    #[test]
    fn shading_an_intersection() {
        let w = default_world();
        let r = Ray::new(Point::new(0.0, 0.0, -5.0), Vector::new(0.0, 0.0, 1.0));
        let shape = &w.objects[0];
        let i = Intersection::new(4.0, shape);
        let comps = i.prepare_computations(r);
        let c = w.shade_hit(comps);
        assert_eq!(c, Color::new(0.38066, 0.47583, 0.2855));
    }
    #[test]
    fn shading_an_intersection_from_inside() {
        let mut w = default_world();
        w.lights = vec![PointLight::new(
            Point::new(0.0, 0.25, 0.0),
            Color::new(1.0, 1.0, 1.0),
        )];
        let r = Ray::new(Point::new(0.0, 0.0, 0.0), Vector::new(0.0, 0.0, 1.0));
        let shape = &w.objects[1];
        let i = Intersection::new(0.5, shape);
        let comps = i.prepare_computations(r);
        let c = w.shade_hit(comps);
        assert_eq!(c, Color::new(0.90498, 0.90498, 0.90498));
    }
    #[test]
    fn color_when_ray_misses() {
        let w = default_world();
        let r = Ray::new(Point::new(0.0, 0.0, -5.0), Vector::new(0.0, 1.0, 0.0));
        let c = w.color_at(r);
        assert_eq!(c, Color::new(0.0, 0.0, 0.0));
    }
    #[test]
    fn color_when_ray_hits() {
        let w = default_world();
        let r = Ray::new(Point::new(0.0, 0.0, -5.0), Vector::new(0.0, 0.0, 1.0));
        let c = w.color_at(r);
        assert_eq!(c, Color::new(0.38066, 0.47583, 0.2855));
    }
    #[test]
    fn color_with_intersection_behind_ray() {
        let mut w = default_world();
        w.objects[0].material.ambient = 1.0;
        w.objects[1].material.ambient = 1.0;

        let r = Ray::new(Point::new(0.0, 0.0, 0.75), Vector::new(0.0, 0.0, -1.0));
        let c = w.color_at(r);
        assert_eq!(c, w.objects[1].material.color);
    }
    #[test]
    fn no_shadow_when_no_object_collinear_with_point() {
        let w = default_world();
        let p = Point::new(0.0, 10.0, 0.0);
        assert_eq!(w.is_shadowed(p), false);
    }
    #[test]
    fn shadow_when_object_between_light_and_point() {
        let w = default_world();
        let p = Point::new(10.0, -10.0, 10.0);
        assert_eq!(w.is_shadowed(p), true);
    }

    #[test]
    fn no_shadow_when_object_behind_point() {
        let w = default_world();
        let p = Point::new(-20.0, 20.0, -20.0);
        assert_eq!(w.is_shadowed(p), false);
    }
    #[test]
    fn no_shadow_when_object_behind_light() {
        let w = default_world();
        let p = Point::new(-2.0, 2.0, -2.0);
        assert_eq!(w.is_shadowed(p), false);
    }
}
