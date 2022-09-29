use ray_tracer::camera::Camera;
use ray_tracer::color::Color;
use ray_tracer::light::PointLight;
use ray_tracer::material::Material;
use ray_tracer::sphere::Sphere;
use ray_tracer::transformations::*;
use ray_tracer::tuple::{Point, Vector};
use ray_tracer::world::World;
use std::f64::consts::PI;

fn main() {
    let floor = Sphere::new()
        .set_transform(scaling(10.0, 0.01, 10.0))
        .set_material(Material {
            color: Color::new(1.0, 0.9, 0.9),
            specular: 0.0,
            ..Default::default()
        });

    let left_wall = Sphere::new()
        .set_transform(
            translation(0.0, 0.0, 5.0)
                * rotation_y(-PI / 4.0)
                * rotation_x(PI / 2.0)
                * scaling(10.0, 0.01, 10.0),
        )
        .set_material(floor.material);

    let right_wall = Sphere::new()
        .set_transform(
            translation(0.0, 0.0, 5.0)
                * rotation_y(PI / 4.0)
                * rotation_x(PI / 2.0)
                * scaling(10.0, 0.01, 10.0),
        )
        .set_material(floor.material);

    let middle = Sphere::new()
        .set_transform(translation(-0.5, 1.0, 0.5))
        .set_material(Material {
            color: Color::new(0.1, 1.0, 0.5),
            diffuse: 0.7,
            specular: 0.3,
            ..Default::default()
        });
    let right = Sphere::new()
        .set_transform(translation(1.5, 0.5, -0.5) * scaling(0.5, 0.5, 0.5))
        .set_material(Material {
            color: Color::new(0.5, 1.0, 0.1),
            diffuse: 0.7,
            specular: 0.3,
            ..Default::default()
        });
    let left = Sphere::new()
        .set_transform(translation(-1.5, 0.33, -0.75) * scaling(0.33, 0.33, 0.33))
        .set_material(Material {
            color: Color::new(1.0, 0.8, 0.1),
            diffuse: 0.7,
            specular: 0.3,
            ..Default::default()
        });

    let mut world = World::new();
    world.objects = vec![floor, left_wall, right_wall, middle, left, right];
    world.lights.push(PointLight::new(
        Point::new(-10.0, 10.0, -10.0),
        Color::new(1.0, 1.0, 1.0),
    ));

    let mut camera = Camera::new(720, 480, PI / 3.0);
    camera.set_transform(view_transform(
        Point::new(0.0, 1.5, -5.0),
        Point::new(0.0, 1.0, 0.0),
        Vector::new(0.0, 1.0, 0.0),
    ));
    let canvas = camera.render(&world);
    println!("{}", canvas.to_ppm());
}
