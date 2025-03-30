use ray_tracer::tuple::Tuple;

struct Projectile {
    position: Tuple,
    velocity: Tuple,
}

struct Environment {
    gravity: Tuple,
    wind: Tuple,
}

fn tick(env: &Environment, projectile: Projectile) -> Projectile {
    let position = projectile.position + projectile.velocity;
    let velocity = projectile.velocity + env.gravity + env.wind;

    Projectile { position, velocity }
}

fn main() {
    let mut projectile = Projectile {
        position: Tuple::point(0.0, 1.0, 0.0),
        velocity: Tuple::vector(1.0, 1.0, 0.0),
    };
    let environment = Environment {
        gravity: Tuple::vector(0.0, -0.1, 0.0),
        wind: Tuple::vector(-0.01, 0.0, 0.0),
    };
    projectile = tick(&environment, projectile);
    while projectile.position.y > 0.0 {
        println!("Projectile position: {:?}", projectile.position);
        projectile = tick(&environment, projectile);
    }
    println!("Final position: {:?}", projectile.position);
}
