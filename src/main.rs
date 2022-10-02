use macroquad::prelude::*;
use particle::*;
use macroquad::window;
use std::f32::consts::PI;

pub mod particle;

const COUNT: usize = 200;
const CLASSES_COUNT: usize = 8;

fn window_conf() -> Conf {
    Conf {
        window_title: "Particle Life".to_owned(),
        window_width: 768,
        window_height: 488,
        window_resizable: false,
        ..Default::default()
    }
}

fn init(particles: &mut [particle::Particle], particle_classes: &mut [particle::ParticleClass]) {
    for i in 0..particle_classes.len() {
        particle_classes[i] = ParticleClass::new();
    }

    for i in 0..particles.len() {
        particles[i].position = Vec2::new(rand::gen_range(0.0, window::screen_width()), rand::gen_range(0.0, window::screen_height()));
        particles[i].velocity = Vec2::from_angle(rand::gen_range(0.0, 2.0 * PI)).rotate(Vec2::new(20.0, 0.0));
        particles[i].particle_class = particle_classes[(rand::rand() % (particle_classes.len() as u32)) as usize];
    }
}

#[macroquad::main(window_conf)]
async fn main() {
    rand::srand(macroquad::miniquad::date::now() as _);

    let mut particles = [Particle::new(Vec2::new(0.0,0.0), Vec2::new(0.0, 0.0), ParticleClass::new()); COUNT];
    let mut particle_classes = [ParticleClass::new(); CLASSES_COUNT];

    init(&mut particles, &mut particle_classes);

    loop {
        clear_background(BLACK);

        for i in 0..particles.len() {
            let particle1 = &mut particles[i].clone();

            for j in 0..particles.len() {
                if i != j {
                    let particle2 = &mut particles[j];
    
                    particle2.add_force(particle1);
                }
            }
        }

        for i in 0..particles.len() {
            let particle1 = &mut particles[i].clone();

            for j in 0..particles.len() {
                if i != j {
                    let particle2 = &mut particles[j];

                    particle2.resolve_collision(particle1);
                }
            }
        }

        for i in 0..particles.len() {
            particles[i].update();
        }

        for i in 0..particles.len() {
            particles[i].draw();
        }

        if macroquad::input::is_key_pressed(KeyCode::Space)
        {
            init(&mut particles, &mut particle_classes);
        }
        next_frame().await
    }
}