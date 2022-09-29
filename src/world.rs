use crate::color::{Color, BLACK};
use crate::intersection::{Computations, Intersections};
use crate::light::PointLight;
use crate::material::{lighting, Material};
use crate::matrix;
use crate::matrix::Matrix;
use crate::ray::Ray;
use crate::sphere::Sphere;
use crate::transformations;
use crate::tuple::{Point, Vector};

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
}

fn view_transform(from: Point, to: Point, up: Vector) -> Matrix {
    let forward = (to - from).normalize();
    let left = forward.cross(up.normalize());
    let true_up = left.cross(forward);

    let orientation = matrix![
        [left.0.x, left.0.y, left.0.z, 0.0],
        [true_up.0.x, true_up.0.y, true_up.0.z, 0.0],
        [-forward.0.x, -forward.0.y, -forward.0.z, 0.0],
        [0.0, 0.0, 0.0, 1.0],
    ];

    orientation * transformations::translation(-from.0.x, -from.0.y, -from.0.z)
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
    use crate::{intersection::Intersection, matrix, tuple::Vector};
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
    fn transformation_matrix_for_default_orientation() {
        let from = Point::new(0.0, 0.0, 0.0);
        let to = Point::new(0.0, 0.0, -1.0);
        let up = Vector::new(0.0, 1.0, 0.0);

        let t = view_transform(from, to, up);
        assert_eq!(t, Matrix::identity(4));
    }

    #[test]
    fn transformation_matrix_looking_in_positive_z_direction() {
        let from = Point::new(0.0, 0.0, 0.0);
        let to = Point::new(0.0, 0.0, 1.0);
        let up = Vector::new(0.0, 1.0, 0.0);

        let t = view_transform(from, to, up);
        assert_eq!(t, transformations::scaling(-1.0, 1.0, -1.0));
    }

    #[test]
    fn view_transformation_moves_the_world() {
        let from = Point::new(0.0, 0.0, 8.0);
        let to = Point::new(0.0, 0.0, 0.0);
        let up = Vector::new(0.0, 1.0, 0.0);

        let t = view_transform(from, to, up);
        assert_eq!(t, transformations::translation(0.0, 0.0, -8.0));
    }

    #[test]
    fn arbitrary_view_transformation() {
        let from = Point::new(1.0, 3.0, 2.0);
        let to = Point::new(4.0, -2.0, 8.0);
        let up = Vector::new(1.0, 1.0, 0.0);

        let t = view_transform(from, to, up);
        assert_eq!(
            t,
            matrix![
                [-0.50709, 0.50709, 0.67612, -2.36643],
                [0.76772, 0.60609, 0.12122, -2.82843],
                [-0.35857, 0.59761, -0.71714, 0.00000],
                [0.00000, 0.00000, 0.00000, 1.00000],
            ]
        )
    }
}
