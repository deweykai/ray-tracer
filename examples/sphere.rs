use ray_tracer::canvas::Canvas;
use ray_tracer::color::Color;
use ray_tracer::ray::Ray;
use ray_tracer::sphere::Sphere;
use ray_tracer::transformations;
use ray_tracer::tuple::{Point, Vector};

fn main() {
    let canvas_pixels = 500;
    let ray_origin = Point::new(0.0, 0.0, -5.0);
    let wall_z = 10.0;
    let wall_size = 7.0;
    let pixel_size = wall_size / canvas_pixels as f64;
    let half = wall_size / 2.0;

    let mut canv = Canvas::new(canvas_pixels, canvas_pixels);
    let s = Sphere::new().set_transform(transformations::scaling(1.0, 0.5, 1.0));
    let color = Color::new(1.0, 0.0, 0.0);

    for y in 0..canvas_pixels {
        // top = +half, bottom = -half
        let world_y = half - pixel_size * y as f64;
        for x in 0..canvas_pixels {
            // left = -half, right = +half
            let world_x = -half + pixel_size * x as f64;

            let position = Point::new(world_x, world_y, wall_z);
            let r = Ray::new(
                ray_origin,
                (position.as_tuple() - ray_origin.as_tuple())
                    .normalize()
                    .try_into()
                    .unwrap(),
            );

            let xs = s.intersect(r);
            if let Some(_) = xs.hit() {
                canv = canv.write_pixel(x, y, color);
            }
        }
    }

    println!("{}", canv.to_ppm());
}
