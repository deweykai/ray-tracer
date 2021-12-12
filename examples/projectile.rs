use ray_tracer::canvas::Canvas;
use ray_tracer::color::Color;
use ray_tracer::tuple::Tuple;

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
        Tuple::new_point(0., 1., 0.),
        Tuple::new_vector(1., 1.8, 0.).normalize() * 11.25,
    );
    let e = Env {
        gravity: Tuple::new_vector(0., -0.1, 0.),
        wind: Tuple::new_vector(-0.01, 0., 0.),
    };
    loop {
        p = p.tick(&e);
        let height = c.height;
        c = c.write_pixel(
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
