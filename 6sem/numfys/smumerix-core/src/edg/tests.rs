use approx::assert_relative_eq;
use nalgebra::Point2;
use nalgebra::Vector2;
use std::collections::BinaryHeap;

use crate::edg::{EventDrivenGas, Particle};

#[test]
fn test_one_particle_straight_on() {
    let pq = BinaryHeap::new();
    let particles: Vec<Particle> = vec![
        Particle {
            x: Point2::new(0.5, 0.8),
            v: Vector2::new(0.0, 0.5),
            r: 0.01,
            m: 1.0,
            collision_count: 0,
        },
        Particle {
            x: Point2::new(0.8, 0.5),
            v: Vector2::new(0.5, 0.0),
            r: 0.01,
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
    edg.step_many(2);
    assert_eq!(edg.particles[0].v.y, -0.5);
    assert_eq!(edg.particles[1].v.x, -0.5);
}

#[test]
fn test_one_particle_diagonal() {
    let pq = BinaryHeap::new();
    let particles: Vec<Particle> = vec![Particle {
        x: Point2::new(0.5, 0.8),
        v: Vector2::new(0.5, 0.5),
        r: 0.01,
        m: 1.0,
        collision_count: 0,
    }];
    let mut edg = EventDrivenGas {
        pq,
        particles,
        xi: 1.0,
        cur_time: 0.0,
    };
    edg.get_initial_collisions();
    edg.step();
    assert_eq!(edg.particles[0].v.y, -0.5);
    assert_eq!(edg.particles[0].v.x, 0.5);
}

#[test]
fn test_one_particle_all_walls() {
    let initial_vx = 0.5;
    let initial_vy = 0.5;
    let pq = BinaryHeap::new();
    let particles: Vec<Particle> = vec![Particle {
        x: Point2::new(0.5, 0.8),
        v: Vector2::new(initial_vx, initial_vy),
        r: 0.01,
        m: 1.0,
        collision_count: 0,
    }];
    let mut edg = EventDrivenGas {
        pq,
        particles,
        xi: 1.0,
        cur_time: 0.0,
    };
    edg.get_initial_collisions();
    edg.step_many(4);
    assert_eq!(edg.particles[0].v.y, initial_vy);
    assert_eq!(edg.particles[0].v.x, initial_vx);
}

#[test]
fn test_one_particle_inelastic() {
    let pq = BinaryHeap::new();
    let particles: Vec<Particle> = vec![Particle {
        x: Point2::new(0.5, 0.8),
        v: Vector2::new(0.0, 0.5),
        r: 0.01,
        m: 1.0,
        collision_count: 0,
    }];
    let mut edg = EventDrivenGas {
        pq,
        particles,
        xi: 0.0,
        cur_time: 0.0,
    };
    edg.get_initial_collisions();
    edg.step();
    assert_eq!(edg.particles[0].v.y, 0.0);
}

#[test]
fn test_two_particles_head_on_x() {
    let pq = BinaryHeap::new();
    let particles: Vec<Particle> = vec![
        Particle {
            x: Point2::new(0.2, 0.5),
            v: Vector2::new(0.2, 0.0),
            r: 0.01,
            m: 1.0,
            collision_count: 0,
        },
        Particle {
            x: Point2::new(0.8, 0.5),
            v: Vector2::new(-0.2, 0.0),
            r: 0.01,
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
    edg.step();
    assert_relative_eq!(edg.particles[0].v.x, -0.2, epsilon = 1e-10);
    assert_relative_eq!(edg.particles[1].v.x, 0.2, epsilon = 1e-10);
}

#[test]
fn test_two_particles_head_on_y() {
    let pq = BinaryHeap::new();
    let particles: Vec<Particle> = vec![
        Particle {
            x: Point2::new(0.5, 0.2),
            v: Vector2::new(0.0, 0.2),
            r: 0.01,
            m: 1.0,
            collision_count: 0,
        },
        Particle {
            x: Point2::new(0.5, 0.8),
            v: Vector2::new(0.0, -0.2),
            r: 0.01,
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
    edg.step();

    assert_relative_eq!(edg.particles[0].v.y, -0.2, epsilon = 1e-10);
    assert_relative_eq!(edg.particles[1].v.y, 0.2, epsilon = 1e-10);
}

#[test]
fn test_two_particles_right_angle_bottom_to_top() {
    let pq = BinaryHeap::new();
    let particles: Vec<Particle> = vec![
        Particle {
            x: Point2::new(0.2, 0.2),
            v: Vector2::new(0.2, 0.2),
            r: 0.01,
            m: 1.0,
            collision_count: 0,
        },
        Particle {
            x: Point2::new(0.8, 0.2),
            v: Vector2::new(-0.2, 0.2),
            r: 0.01,
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
    println!("PQ: {:?}", edg.pq);
    edg.step();

    assert_relative_eq!(edg.particles[0].v.x, -0.2, epsilon = 1e-10);
    assert_relative_eq!(edg.particles[0].v.y, 0.2, epsilon = 1e-10);
    assert_relative_eq!(edg.particles[1].v.x, 0.2, epsilon = 1e-10);
    assert_relative_eq!(edg.particles[1].v.y, 0.2, epsilon = 1e-10);
}

#[test]
fn test_two_particles_right_angle_top_to_bottom() {
    let pq = BinaryHeap::new();
    let particles: Vec<Particle> = vec![
        Particle {
            x: Point2::new(0.2, 0.8),
            v: Vector2::new(0.2, -0.2),
            r: 0.01,
            m: 1.0,
            collision_count: 0,
        },
        Particle {
            x: Point2::new(0.8, 0.8),
            v: Vector2::new(-0.2, -0.2),
            r: 0.01,
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
    println!("PQ: {:?}", edg.pq);
    edg.step();

    assert_relative_eq!(edg.particles[0].v.x, -0.2, epsilon = 1e-10);
    assert_relative_eq!(edg.particles[0].v.y, -0.2, epsilon = 1e-10);
    assert_relative_eq!(edg.particles[1].v.x, 0.2, epsilon = 1e-10);
    assert_relative_eq!(edg.particles[1].v.y, -0.2, epsilon = 1e-10);
}

#[test]
fn test_two_particles_right_angle_bl_tr() {
    let pq = BinaryHeap::new();
    let particles: Vec<Particle> = vec![
        Particle {
            x: Point2::new(0.2, 0.5),
            v: Vector2::new(0.2, 0.0),
            r: 0.01,
            m: 1.0,
            collision_count: 0,
        },
        Particle {
            x: Point2::new(0.5, 0.2),
            v: Vector2::new(0.0, 0.2),
            r: 0.01,
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
    println!("PQ: {:?}", edg.pq);
    edg.step();

    assert_relative_eq!(edg.particles[0].v.x, 0.0, epsilon = 1e-10);
    assert_relative_eq!(edg.particles[0].v.y, 0.2, epsilon = 1e-10);
    assert_relative_eq!(edg.particles[1].v.x, 0.2, epsilon = 1e-10);
    assert_relative_eq!(edg.particles[1].v.y, 0.0, epsilon = 1e-10);
}

#[test]
fn test_two_particles_head_on_zero_xi() {
    let pq = BinaryHeap::new();
    let particles: Vec<Particle> = vec![
        Particle {
            x: Point2::new(0.2, 0.5),
            v: Vector2::new(0.2, 0.0),
            r: 0.01,
            m: 1.0,
            collision_count: 0,
        },
        Particle {
            x: Point2::new(0.8, 0.5),
            v: Vector2::new(-0.2, 0.0),
            r: 0.01,
            m: 1.0,
            collision_count: 0,
        },
    ];
    let mut edg = EventDrivenGas {
        pq,
        particles,
        xi: 0.0,
        cur_time: 0.0,
    };
    edg.get_initial_collisions();
    edg.step();
    assert_relative_eq!(edg.particles[0].v.x, 0.0, epsilon = 1e-10);
    assert_relative_eq!(edg.particles[1].v.x, 0.0, epsilon = 1e-10);
}

#[test]
fn test_many_particles_constant_energy() {
    let mut edg = EventDrivenGas::new_uniform_v(100, 0.04, 0.03).unwrap();
    let init_energy = edg.get_total_energy();
    edg.step_many(1000);
    let final_energy = edg.get_total_energy();
    println!("Energy diff is {}", final_energy - init_energy);
    assert_relative_eq!(init_energy, final_energy, epsilon = 1e-3);
}

#[test]
fn test_small_big_x() {
    fn restart(y: f64) -> EventDrivenGas {
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
        EventDrivenGas {
            pq,
            particles,
            xi: 1.0,
            cur_time: 0.0,
        }
    }

    // y, angle
    let mut data = Vec::new();
    let x_axis = Vector2::new(1.0, 0.0);

    for y in -100..100 {
        let y = y as f64 * 0.001;
        let mut edg = restart(y);
        edg.get_initial_collisions();
        edg.step();
        let angle = edg.particles[1].v.angle(&x_axis);
        data.push((y, angle));
        // println!("angle {:?}", edg.pa)
    }

    println!("{data:.3?}");
    assert!(true); // False if we want list
}
