use std::fs::File;
use std::io::Write;
use std::iter::successors;

use ray_tracer::canvas::Canvas;
use ray_tracer::color::Color;
use ray_tracer::ppm::ppm;
use ray_tracer::tuple::Tuple;

struct Projectile {
    position: Tuple,
    velocity: Tuple,
}

struct Environment {
    gravity: Tuple,
    wind: Tuple,
}

fn tick(env: &Environment, projectile: &Projectile) -> Projectile {
    let position = projectile.position + projectile.velocity;
    let velocity = projectile.velocity + env.gravity + env.wind;

    Projectile { position, velocity }
}

fn main() {
    let mut canvas = Canvas::new(900, 550, Some(Color::black()));
    let color = Color::new(1.0, 0.0, 0.0);

    let environment = Environment {
        gravity: Tuple::vector(0.0, -0.1, 0.0),
        wind: Tuple::vector(-0.01, 0.0, 0.0),
    };
    let projectiles = successors(
        Some(Projectile {
            position: Tuple::point(0.0, 1.0, 0.0),
            velocity: Tuple::vector(1.0, 1.8, 0.0).normalize() * 10.0,
        }),
        |current_projectile| Some(tick(&environment, current_projectile)),
    )
    .take_while(|projectile| projectile.position.y > 0.0)
    .collect::<Vec<_>>();

    for projectile in projectiles {
        let y = canvas.height - projectile.position.y as usize;
        canvas.set_pixel(projectile.position.x as usize, y, color);
    }
    let mut ppm_file = File::create("chapter_2.ppm").expect("creation failed");
    ppm_file
        .write(ppm(&canvas).as_bytes())
        .expect("write failed");
}
