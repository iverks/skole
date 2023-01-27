use rand::distributions::Uniform;
use rand::Rng;
use std::cell::RefCell;
use std::cmp::Ordering;
use std::collections::BinaryHeap;
use std::f64::consts::PI;

const MIN_X: f64 = 0.0;
const MAX_X: f64 = 1.0;
const MIN_Y: f64 = 0.0;
const MAX_Y: f64 = 1.0;

#[derive(Copy, Clone, PartialEq, Debug)]
pub struct Particle {
    px: f64,
    py: f64,
    vx: f64,
    vy: f64,
    r: f64,
    m: f64,
    collision_count: i32,
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
    time: f64,
    particles: (usize, CollisionObject),
    collision_counts: (i32, i32),
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
        self.partial_cmp(other).unwrap()
    }
}

pub struct EventDrivenGas {
    pq: BinaryHeap<Collision>,
    particles: Vec<RefCell<Particle>>,
    xi: f64,
    cur_time: f64,
}

impl EventDrivenGas {
    pub fn new() -> Self {
        EventDrivenGas::new_uniform_v()
    }

    pub fn new_uniform_v() -> EventDrivenGas {
        let pq = BinaryHeap::new();
        let mut particles = vec![];
        let mut rng = rand::thread_rng();
        let pos_gen = Uniform::new(MIN_X, MAX_X);
        let angle_gen = Uniform::new(0.0, PI);
        let v_0 = 0.04;
        let r_0 = 0.03;
        for _ in 0..100 + 1 {
            let px = rng.sample(pos_gen);
            let py = rng.sample(pos_gen);
            let angle = rng.sample(angle_gen);
            let vx = v_0 * angle.cos();
            let vy = v_0 * angle.sin();
            let r = r_0;
            let m = 1.0;
            particles.push(RefCell::new(Particle {
                px,
                py,
                vx,
                vy,
                r,
                m,
                collision_count: 0,
            }));
        }

        let mut sim = Self {
            pq,
            particles,
            xi: 1.0,
            cur_time: 0.0,
        };

        sim.get_initial_collisions();

        return sim;
    }

    pub fn get_initial_collisions(&mut self) {
        for particle_idx in 0..self.particles.len() {
            self.add_collisions_to_pq(particle_idx);
        }
    }

    pub fn time_until_wall(&self, particle_idx: usize) -> (f64, CollisionObject) {
        let particle = self.particles[particle_idx].borrow_mut();
        let x_time_wall = {
            if particle.vx == 0.0 {
                (f64::INFINITY, CollisionObject::Never)
            } else if particle.vx > 0.0 {
                (
                    (MAX_X - particle.r - particle.px) / particle.vx,
                    CollisionObject::WallLeft,
                )
            } else {
                (
                    (MIN_X + particle.r - particle.px) / particle.vx,
                    CollisionObject::WallRight,
                )
            }
        };
        let y_time_wall = {
            if particle.vy == 0.0 {
                (f64::INFINITY, CollisionObject::Never)
            } else if particle.vy > 0.0 {
                (
                    (MAX_Y - particle.r - particle.py) / particle.vy,
                    CollisionObject::WallTop,
                )
            } else {
                (
                    (MIN_Y + particle.r - particle.py) / particle.vy,
                    CollisionObject::WallBottom,
                )
            }
        };

        let mut closest_wall = std::cmp::min_by(x_time_wall, y_time_wall, |x, y| {
            x.0.partial_cmp(&y.0).expect("impossible to sort")
        });
        closest_wall.0 += self.cur_time;
        return closest_wall;
    }

    pub fn collide(&mut self, particle_idx: usize, collision_object: CollisionObject) {
        let mut particle = self.particles[particle_idx].borrow_mut();
        particle.collision_count += 1;
        match collision_object {
            CollisionObject::WallBottom | CollisionObject::WallTop => {
                particle.vx *= self.xi;
                particle.vy *= -self.xi;
            }
            CollisionObject::WallLeft | CollisionObject::WallRight => {
                particle.vx *= -self.xi;
                particle.vy *= self.xi;
            }
            CollisionObject::Never => unreachable!("Should never collide with never"),
            CollisionObject::Particle(idx) => {
                let mut other = self.particles[idx].borrow_mut();
                let delta_vx = particle.vx - other.vx;
                let delta_vy = particle.vy - other.vy;
                let delta_x = particle.px - other.px;
                let delta_y = particle.py - other.py;
                let r_squared = (particle.r + other.r).powi(2);

                particle.vx += ((1.0 + self.xi)
                    * (other.m / (particle.m + other.m))
                    * ((delta_vx * delta_x) / r_squared))
                    * delta_x;
                particle.vy += ((1.0 + self.xi)
                    * (other.m / (particle.m + other.m))
                    * ((delta_vy * delta_y) / r_squared))
                    * delta_y;
                other.vx -= ((1.0 + self.xi)
                    * (particle.m / (particle.m + other.m))
                    * ((delta_vx * delta_x) / r_squared))
                    * delta_x;
                other.vy -= ((1.0 + self.xi)
                    * (particle.m / (particle.m + other.m))
                    * ((delta_vy * delta_y) / r_squared))
                    * delta_y;
            }
        }
    }

    fn add_collisions_to_pq(&mut self, particle_idx: usize) {
        let collision_count = self.particles[particle_idx].borrow().collision_count;
        let (wall_time, wall) = self.time_until_wall(particle_idx);
        if wall != CollisionObject::Never {
            self.pq.push(Collision {
                time: wall_time,
                particles: (particle_idx, wall),
                collision_counts: (collision_count, 0),
            });
        }
        let particle = self.particles[particle_idx].borrow();
        for (idx, other_cell) in self.particles.iter().enumerate() {
            if idx == particle_idx {
                continue;
            }
            let other = other_cell.borrow();
            let delta_vx = particle.vx - other.vx;
            let delta_vy = particle.vy - other.vy;
            let delta_x = particle.px - other.px;
            let delta_y = particle.py - other.py;
            let deltaprikk = delta_vx * delta_x + delta_vy * delta_y;

            if deltaprikk >= 0.0 {
                continue;
            }

            let d = deltaprikk.powi(2)
                - (delta_vx.powi(2) + delta_vy.powi(2))
                    * (delta_x.powi(2) * delta_y.powi(2) - (particle.r + other.r).powi(2));
            if d <= 0.0 {
                continue;
            }

            self.pq.push(Collision {
                time: -(deltaprikk + d.sqrt()) / (delta_vx.powi(2) + delta_vy.powi(2)),
                particles: (particle_idx, CollisionObject::Particle(idx)),
                collision_counts: (particle.collision_count, other.collision_count),
            });
        }
    }

    fn move_particles(&mut self, timestep: f64) {
        for particle_cell in self.particles.iter() {
            let mut particle = particle_cell.borrow_mut();
            particle.px += particle.vx * timestep;
            particle.py += particle.vy * timestep;
        }
    }

    pub fn step(&mut self) {
        // Get collision
        let collision = loop {
            let coll = self.pq.pop().expect("queue empty");
            let first_is_valid = coll.collision_counts.0
                == self.particles[coll.particles.0].borrow().collision_count;
            let second_count = match coll.particles.1 {
                CollisionObject::Particle(idx) => self.particles[idx].borrow().collision_count,
                _ => 0,
            };
            let second_is_valid = coll.collision_counts.1 == second_count;
            if first_is_valid && second_is_valid {
                break coll;
            }
        };
        // Move particles until time of collision
        self.move_particles(collision.time - self.cur_time);
        self.cur_time += collision.time;
        // Do collision speed changes
        self.collide(collision.particles.0, collision.particles.1);
        // Insert new collisions into queue
        self.add_collisions_to_pq(collision.particles.0);
        match collision.particles.1 {
            CollisionObject::Particle(idx) => self.add_collisions_to_pq(idx),
            _ => (),
        }
    }

    pub fn step_many(&mut self, num_loops: i32) {
        for _ in 0..num_loops {
            self.step();
        }
    }
}

#[cfg(test)]
mod tests {
    use std::{cell::RefCell, collections::BinaryHeap};

    use crate::ex1::edg::{EventDrivenGas, Particle};

    #[test]
    fn test_one_particle_straight_on() {
        let pq = BinaryHeap::new();
        let particles: Vec<RefCell<Particle>> = vec![
            RefCell::new(Particle {
                px: 0.5,
                py: 0.8,
                vx: 0.0,
                vy: 0.5,
                r: 0.01,
                m: 1.0,
                collision_count: 0,
            }),
            RefCell::new(Particle {
                px: 0.8,
                py: 0.5,
                vx: 0.5,
                vy: 0.0,
                r: 0.01,
                m: 1.0,
                collision_count: 0,
            }),
        ];
        let mut edg = EventDrivenGas {
            pq,
            particles,
            xi: 1.0,
            cur_time: 0.0,
        };
        edg.get_initial_collisions();
        edg.step_many(2);
        assert_eq!(edg.particles[0].borrow().vy, -0.5);
        assert_eq!(edg.particles[1].borrow().vx, -0.5);
    }

    #[test]
    fn test_one_particle_diagonal() {
        let pq = BinaryHeap::new();
        let particles: Vec<RefCell<Particle>> = vec![RefCell::new(Particle {
            px: 0.5,
            py: 0.8,
            vx: 0.5,
            vy: 0.5,
            r: 0.01,
            m: 1.0,
            collision_count: 0,
        })];
        let mut edg = EventDrivenGas {
            pq,
            particles,
            xi: 1.0,
            cur_time: 0.0,
        };
        edg.get_initial_collisions();
        edg.step();
        assert_eq!(edg.particles[0].borrow().vy, -0.5);
        assert_eq!(edg.particles[0].borrow().vx, 0.5);
    }

    #[test]
    fn test_one_particle_all_walls() {
        let initial_vx = 0.5;
        let initial_vy = 0.5;
        let pq = BinaryHeap::new();
        let particles: Vec<RefCell<Particle>> = vec![RefCell::new(Particle {
            px: 0.5,
            py: 0.8,
            vx: initial_vx,
            vy: initial_vy,
            r: 0.01,
            m: 1.0,
            collision_count: 0,
        })];
        let mut edg = EventDrivenGas {
            pq,
            particles,
            xi: 1.0,
            cur_time: 0.0,
        };
        edg.get_initial_collisions();
        edg.step_many(4);
        assert_eq!(edg.particles[0].borrow().vy, initial_vy);
        assert_eq!(edg.particles[0].borrow().vx, initial_vx);
    }

    #[test]
    fn test_one_particle_inelastic() {
        let pq = BinaryHeap::new();
        let particles: Vec<RefCell<Particle>> = vec![RefCell::new(Particle {
            px: 0.5,
            py: 0.8,
            vx: 0.0,
            vy: 0.5,
            r: 0.01,
            m: 1.0,
            collision_count: 0,
        })];
        let mut edg = EventDrivenGas {
            pq,
            particles,
            xi: 0.0,
            cur_time: 0.0,
        };
        edg.get_initial_collisions();
        edg.step();
        assert_eq!(edg.particles[0].borrow().vy, 0.0);
    }

    #[test]
    fn test_two_particles_head_on() {
        let pq = BinaryHeap::new();
        let particles: Vec<RefCell<Particle>> = vec![
            RefCell::new(Particle {
                px: 0.3,
                py: 0.5,
                vx: 0.5,
                vy: 0.0,
                r: 0.01,
                m: 1.0,
                collision_count: 0,
            }),
            RefCell::new(Particle {
                px: 0.8,
                py: 0.5,
                vx: -0.5,
                vy: 0.0,
                r: 0.01,
                m: 1.0,
                collision_count: 0,
            }),
        ];
        let mut edg = EventDrivenGas {
            pq,
            particles,
            xi: 1.0,
            cur_time: 0.0,
        };
        edg.get_initial_collisions();
        edg.step();
        assert_eq!(edg.particles[0].borrow().vx, -0.5);
        assert_eq!(edg.particles[0].borrow().vy, 0.0);
        assert_eq!(edg.particles[1].borrow().vx, 0.5);
        assert_eq!(edg.particles[1].borrow().vy, 0.0);
    }
}
