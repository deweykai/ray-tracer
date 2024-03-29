use ray_tracer::canvas::Canvas;
use ray_tracer::color::Color;
use ray_tracer::tuple::{Point, Tuple, Vector};

#[derive(Debug)]
struct Projectile {
    vel: Tuple,
    pos: Tuple,
}

#[derive(Debug)]
struct Env {
    gravity: Tuple,
    wind: Tuple,
}

impl Projectile {
    fn new(pos: Tuple, vel: Tuple) -> Projectile {
        Projectile { pos, vel }
    }

    fn tick(self, env: &Env) -> Projectile {
        Projectile {
            pos: self.pos + self.vel * 0.1,
            vel: self.vel + (env.gravity + env.wind) * 0.1,
        }
    }
}

fn main() {
    let mut c = Canvas::new(900, 500);
    let mut p = Projectile::new(
        Point::new(0., 1., 0.).into(),
        (Vector::new(1., 1.8, 0.) * 11.25).into(),
    );
    let e = Env {
        gravity: Vector::new(0., -0.1, 0.).into(),
        wind: Vector::new(-0.01, 0., 0.).into(),
    };
    loop {
        p = p.tick(&e);
        let height = c.height;
        c.write_pixel(
            p.pos.x as isize,
            height - p.pos.y as isize,
            Color::new(0.7, 0.0, 0.0),
        );
        if p.pos.y < 0. {
            break;
        }
    }
    println!("{}", c.to_ppm());
}
