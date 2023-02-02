#[cfg(test)]
mod tests;

use anyhow::{anyhow, Result};
use nalgebra::{Point2, Vector2};
use rand::distributions::Uniform;
use rand::Rng;
use std::cmp::Ordering;
use std::collections::BinaryHeap;
use std::f64::consts::PI;

const MIN_X: f64 = 0.0;
const MAX_X: f64 = 1.0;
const MIN_Y: f64 = 0.0;
const MAX_Y: f64 = 1.0;

#[derive(Copy, Clone, PartialEq, Debug)]
pub struct Particle {
    pub x: Point2<f64>,
    pub v: Vector2<f64>,
    pub r: f64,
    pub m: f64,
    pub collision_count: i32,
}

impl Eq for Particle {}

#[derive(Clone, Copy, PartialEq, Eq, Debug)]
pub enum CollisionObject {
    Particle(usize),
    WallTop,
    WallBottom,
    WallLeft,
    WallRight,
    Never,
}

#[derive(Clone, PartialEq, Debug)]
pub struct Collision {
    pub time: f64,
    pub particles: (usize, CollisionObject),
    pub collision_counts: (i32, i32),
}

impl Eq for Collision {}

impl PartialOrd for Collision {
    fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
        // Reverse the order to reverse comparison
        // This is hopefully a min heap
        other.time.partial_cmp(&self.time)
    }
}

impl Ord for Collision {
    fn cmp(&self, other: &Self) -> Ordering {
        self.partial_cmp(other)
            .unwrap_or_else(|| panic!("Cannot compare: {} with {}", self.time, other.time))
    }
}

pub struct EventDrivenGas {
    pub pq: BinaryHeap<Collision>,
    pub particles: Vec<Particle>,
    pub xi: f64,
    pub cur_time: f64,
}

fn check_overlap(x: Point2<f64>, r: f64, particles: &Vec<Particle>) -> bool {
    for particle in particles {
        let delta_x = particle.x - x;
        if delta_x.dot(&delta_x) <= (particle.r + r).powi(2) {
            return true;
        }
    }
    false
}

impl EventDrivenGas {
    pub fn new() -> Result<Self> {
        EventDrivenGas::new_uniform_v(100, 0.04, 0.03)
    }

    pub fn new_uniform_v(num_particles: i32, speed: f64, radius: f64) -> Result<Self> {
        let pq = BinaryHeap::new();
        let mut particles = vec![];
        let mut rng = rand::thread_rng();
        let pos_gen = Uniform::new(MIN_X + radius, MAX_X - radius);
        let angle_gen = Uniform::new(0.0, PI);
        for _ in 0..num_particles {
            let mut x = Point2::new(rng.sample(pos_gen), rng.sample(pos_gen));
            let angle = rng.sample(angle_gen);
            let v = Vector2::new(speed * angle.cos(), speed * angle.sin());
            let r = radius;
            let m = 1.0;
            let mut loop_counter = 1;

            while check_overlap(x, r, &particles) {
                x = Point2::new(rng.sample(pos_gen), rng.sample(pos_gen));
                loop_counter += 1;
                if loop_counter > 10_000 {
                    return Err(anyhow!("Too large or many particles, can't fit"));
                }
            }

            particles.push(Particle {
                x,
                v,
                r,
                m,
                collision_count: 0,
            });
        }

        let mut sim = Self {
            pq,
            particles,
            xi: 1.0,
            cur_time: 0.0,
        };

        sim.get_initial_collisions();

        Ok(sim)
    }

    pub fn new_uniform_v_different_m(num_particles: i32, speed: f64, radius: f64) -> Result<Self> {
        if num_particles % 2 != 0 {
            return Err(anyhow!("num_particles must be divisible by 2"));
        }

        let pq = BinaryHeap::new();
        let mut particles = vec![];
        let mut rng = rand::thread_rng();
        let pos_gen = Uniform::new(MIN_X + radius, MAX_X - radius);
        let angle_gen = Uniform::new(0.0, PI);
        for i in 0..num_particles {
            let mut x = Point2::new(rng.sample(pos_gen), rng.sample(pos_gen));
            let angle = rng.sample(angle_gen);
            let v = Vector2::new(speed * angle.cos(), speed * angle.sin());
            let r = radius;
            let m_0 = 1.0;
            let m = if i < num_particles / 2 {
                m_0
            } else {
                4.0 * m_0
            };
            let mut loop_counter = 1;

            while check_overlap(x, r, &particles) {
                x = Point2::new(rng.sample(pos_gen), rng.sample(pos_gen));
                loop_counter += 1;
                if loop_counter > 10_000 {
                    return Err(anyhow!("Too large or many particles, can't fit"));
                }
            }

            particles.push(Particle {
                x,
                v,
                r,
                m,
                collision_count: 0,
            });
        }

        let mut sim = Self {
            pq,
            particles,
            xi: 1.0,
            cur_time: 0.0,
        };

        sim.get_initial_collisions();

        Ok(sim)
    }

    pub fn new_for_test_4(y: f64) -> EventDrivenGas {
        let pq = BinaryHeap::new();
        let particles: Vec<Particle> = vec![
            Particle {
                x: Point2::new(0.5, 0.5),
                v: Vector2::new(0.0, 0.0),
                r: 0.1,
                m: 1e6,
                collision_count: 0,
            },
            Particle {
                x: Point2::new(0.3, 0.5 + y),
                v: Vector2::new(0.2, 0.0),
                r: 0.001,
                m: 1.0,
                collision_count: 0,
            },
        ];
        let mut edg = EventDrivenGas {
            pq,
            particles,
            xi: 1.0,
            cur_time: 0.0,
        };

        edg.get_initial_collisions();
        edg
    }

    pub fn get_initial_collisions(&mut self) {
        for particle_idx in 0..self.particles.len() {
            self.add_collisions_to_pq(particle_idx);
        }
    }

    pub fn time_until_wall(&self, particle_idx: usize) -> Option<(f64, CollisionObject)> {
        let particle = self.particles[particle_idx];

        if particle.v == Vector2::new(0.0, 0.0) {
            return None;
        }

        let x_time_wall = {
            if particle.v.x == 0.0 {
                (f64::INFINITY, CollisionObject::Never)
            } else if particle.v.x > 0.0 {
                (
                    (MAX_X - particle.r - particle.x.x) / particle.v.x,
                    CollisionObject::WallLeft,
                )
            } else {
                (
                    (MIN_X + particle.r - particle.x.x) / particle.v.x,
                    CollisionObject::WallRight,
                )
            }
        };
        let y_time_wall = {
            if particle.v.y == 0.0 {
                (f64::INFINITY, CollisionObject::Never)
            } else if particle.v.y > 0.0 {
                (
                    (MAX_Y - particle.r - particle.x.y) / particle.v.y,
                    CollisionObject::WallTop,
                )
            } else {
                (
                    (MIN_Y + particle.r - particle.x.y) / particle.v.y,
                    CollisionObject::WallBottom,
                )
            }
        };

        let min = std::cmp::min_by(x_time_wall, y_time_wall, |x, y| {
            x.0.partial_cmp(&y.0)
                .unwrap_or_else(|| panic!("impossible to sort {} and {}", x.0, y.0))
        });

        if min.1 == CollisionObject::Never {
            panic!("Particle glitched out of bounds");
        }

        Some(min)
    }

    pub fn collide(&mut self, particle_idx: usize, collision_object: CollisionObject) {
        {
            let particle = &mut self.particles[particle_idx];
            particle.collision_count += 1;
        }
        match collision_object {
            CollisionObject::WallBottom | CollisionObject::WallTop => {
                let particle = &mut self.particles[particle_idx];
                particle
                    .v
                    .component_mul_assign(&Vector2::new(self.xi, -self.xi));
            }
            CollisionObject::WallLeft | CollisionObject::WallRight => {
                let particle = &mut self.particles[particle_idx];
                particle
                    .v
                    .component_mul_assign(&Vector2::new(-self.xi, self.xi));
            }
            CollisionObject::Never => unreachable!("Should never collide with never"),
            CollisionObject::Particle(idx) => {
                let particle = &self.particles[particle_idx];
                let other = &self.particles[idx];
                let delta_v = other.v - particle.v;
                let delta_x = other.x - particle.x;
                let r_squared = (particle.r + other.r).powi(2);

                let new_particle_v = particle.v
                    + ((1.0 + self.xi)
                        * (other.m / (particle.m + other.m))
                        * ((delta_v.dot(&delta_x)) / r_squared))
                        * delta_x;

                let new_other_v = other.v
                    - ((1.0 + self.xi)
                        * (particle.m / (particle.m + other.m))
                        * ((delta_v.dot(&delta_x)) / r_squared))
                        * delta_x;
                let mut particle = &mut self.particles[particle_idx];
                particle.v = new_particle_v;
                let mut other = &mut self.particles[idx];
                other.v = new_other_v;
                other.collision_count += 1;
            }
        }
    }

    fn add_collisions_to_pq(&mut self, particle_idx: usize) {
        let collision_count = self.particles[particle_idx].collision_count;

        if let Some((time, wall)) = self.time_until_wall(particle_idx) {
            self.pq.push(Collision {
                time: self.cur_time + time,
                particles: (particle_idx, wall),
                collision_counts: (collision_count, 0),
            });
        }

        let particle = self.particles[particle_idx];
        for (idx, other_cell) in self.particles.iter().enumerate() {
            if idx == particle_idx {
                continue;
            }
            let other = other_cell;
            let delta_v = particle.v - other.v;
            let delta_x = particle.x - other.x;
            let deltaprikk = delta_v.dot(&delta_x);

            if deltaprikk >= 0.0 {
                continue;
            }

            let d = deltaprikk.powi(2)
                - delta_v.dot(&delta_v) * (delta_x.dot(&delta_x) - (particle.r + other.r).powi(2));
            if d <= 0.0 {
                continue;
            }

            let timestep = -(deltaprikk + d.sqrt()) / (delta_v.dot(&delta_v));

            self.pq.push(Collision {
                time: self.cur_time + timestep,
                particles: (particle_idx, CollisionObject::Particle(idx)),
                collision_counts: (particle.collision_count, other.collision_count),
            });
        }
    }

    fn move_particles(&mut self, timestep: f64) {
        for particle in self.particles.iter_mut() {
            let new_px = particle.x + particle.v * timestep;
            particle.x = new_px;
        }
    }

    pub fn step(&mut self) {
        // Get collision
        let collision = loop {
            let coll = self.pq.pop().expect("queue empty");
            let first_is_valid =
                coll.collision_counts.0 == self.particles[coll.particles.0].collision_count;
            let second_count = match coll.particles.1 {
                CollisionObject::Particle(idx) => self.particles[idx].collision_count,
                _ => 0,
            };
            let second_is_valid = coll.collision_counts.1 == second_count;
            if first_is_valid && second_is_valid {
                break coll;
            }
        };
        // println!("particles: {:?}", self.particles);
        // println!("counts: {:?}", collision.collision_counts);
        // println!("coll {:.2}, cur {:.2}", collision.time, self.cur_time);
        assert!(collision.time > self.cur_time);
        // Move particles until time of collision
        self.move_particles(collision.time - self.cur_time);
        self.cur_time = collision.time;
        // Do collision speed changes
        self.collide(collision.particles.0, collision.particles.1);
        // Insert new collisions into queue
        self.add_collisions_to_pq(collision.particles.0);

        match collision.particles.1 {
            CollisionObject::Particle(idx) => self.add_collisions_to_pq(idx),
            _ => (),
        };
    }

    pub fn step_many(&mut self, num_loops: i32) {
        for _ in 0..num_loops {
            self.step();
        }
    }

    pub fn get_total_energy(&self) -> f64 {
        self.particles
            .iter()
            .map(|prt| (prt.m, prt.v))
            .map(|(m, v)| m / 2.0 * v.dot(&v))
            .sum()
    }

    pub fn get_speeds(&self) -> Vec<f64> {
        self.particles.iter().map(|p| p.v.magnitude()).collect()
    }
}

pub fn get_moved_particles(particles: &Vec<Particle>, timestep: f64) -> Vec<Particle> {
    let mut particles_clone = particles.clone();
    for particle in particles_clone.iter_mut() {
        let new_px = particle.x + particle.v * timestep;
        particle.x = new_px;
    }
    particles_clone
}
