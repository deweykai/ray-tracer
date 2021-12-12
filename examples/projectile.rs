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
            pos: self.pos + self.vel,
            vel: self.vel + env.gravity + env.wind,
        }
    }
}

fn main() {
    let mut p = Projectile::new(
        Tuple::new_point(0., 1., 0.),
        Tuple::new_vector(1., 1., 0.).normalize(),
    );
    let e = Env {
        gravity: Tuple::new_vector(0., -0.1, 0.),
        wind: Tuple::new_vector(-0.01, 0., 0.),
    };
    println!("{:#?}", e);
    println!("{:#?}", p);
    loop {
        p = p.tick(&e);
        println!("{:?}", p.pos);
        if p.pos.y < 0. {
            break;
        }
    }
}
