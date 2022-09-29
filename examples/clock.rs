use std::f64::consts::PI;

use ray_tracer::canvas::Canvas;
use ray_tracer::color::Color;
use ray_tracer::transformations;
use ray_tracer::tuple::Point;

fn main() {
    let width = 200;
    let height = 200;
    let mut canv = Canvas::new(width, height);

    for i in 0..12 {
        let p = Point::new(1.0, 0.0, 0.0);
        let a = transformations::rotation_z(i as f64 * PI / 6.0);
        let b = transformations::scaling(width as f64 / 3.0, height as f64 / 3.0, 0.0);
        let c = transformations::translation(width as f64 / 2.0, height as f64 / 2.0, 0.0);

        let t = c * b * a;

        let p2 = t * p;

        canv.write_pixel(p2.x as isize, p2.y as isize, Color::new(1.0, 1.0, 1.0));
    }

    println!("{}", canv.to_ppm());
}
