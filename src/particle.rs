use macroquad::{prelude::*, window};

const RADIUS: f32 = 20.0;
const FRICTION: f32 = 0.5;

#[derive(Debug, Clone, Copy)]
pub struct ParticleClass {
    attract_force: f32,
    repulse_force: f32,
    max_radius: f32,
    min_radius: f32,

    color: Color,
}

impl ParticleClass {
    pub fn new() -> Self {
        
        let max_radius = rand::gen_range(0.0, 50.0);

        return Self {
            attract_force: rand::gen_range(0.0, 200.0),
            repulse_force: rand::gen_range(0.0, 200.0),
            max_radius: max_radius,
            min_radius: rand::gen_range(0.0, max_radius * 0.5),

            color: Color::new(rand::gen_range(0.0, 1.0),
                              rand::gen_range(0.0, 1.0),
                              rand::gen_range(0.0, 1.0),
                              1.0 ),
        }
    }
}

#[derive(Debug, Clone, Copy)]
pub struct Particle {
    pub position: Vec2,
    pub velocity: Vec2,
    pub particle_class: ParticleClass
}

impl Particle {
    pub fn new(position: Vec2, velocity: Vec2, particle_class: ParticleClass) -> Self {
        Self {
            position: position,
            velocity: velocity,
            particle_class: particle_class
        }
    }

    pub fn resolve_collision(&mut self, other: &mut Particle) {
        let between = other.position - self.position;
        let length2 : f32 = between.length_squared();
    
        if length2 - RADIUS * RADIUS * 4.0f32 < 0.0f32 {
            let normal = between.normalize();
    
            let collision_depth = (length2.sqrt() - RADIUS * 2.0f32).abs() * 0.5f32;

            self.position.x -= normal.x * collision_depth;
            self.position.y -= normal.y * collision_depth;
        }
    }

    pub fn add_force(&mut self, other: &mut Particle) {
        let distance2 = self.position.distance_squared(other.position);

        if distance2 <= self.particle_class.max_radius * self.particle_class.max_radius && 
           distance2 >= self.particle_class.min_radius * self.particle_class.min_radius {

            let distance = distance2.sqrt();

            let collision_dir_x = (self.position.x - other.position.x) / distance;
            let collision_dir_y = (self.position.y - other.position.y) / distance;

            self.velocity.x += collision_dir_x * self.particle_class.repulse_force;
            self.velocity.y += collision_dir_y * self.particle_class.repulse_force;

            if !(distance2 - RADIUS * RADIUS * 4.0f32 < 0.0f32) {
                self.velocity.x -= collision_dir_x * self.particle_class.attract_force;
                self.velocity.y -= collision_dir_y * self.particle_class.attract_force;

            
                // self.velocity.x += collision_dir_x * self.particle_class.attract_force;
                // self.velocity.y += collision_dir_y * self.particle_class.attract_force;
            }

            self.velocity.x *= FRICTION;
            self.velocity.y *= FRICTION;
        }
    }

    pub fn update(&mut self) {
        let delta = get_frame_time();

        self.position.x += self.velocity.x * delta;
        self.position.y += self.velocity.y * delta;

        if self.position.x < -RADIUS {
            self.position.x = window::screen_width() + RADIUS
        } else if self.position.x > window::screen_width() + RADIUS {
            self.position.x = -RADIUS
        } else if self.position.y < -RADIUS {
            self.position.y = window::screen_height() + RADIUS
        } else if self.position.y > window::screen_height() + RADIUS {
            self.position.y = -RADIUS
        }
    }

    pub fn draw(&self) {
        draw_circle(self.position.x, self.position.y, RADIUS, self.particle_class.color);
    }
}
