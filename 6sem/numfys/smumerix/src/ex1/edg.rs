use std::cmp::Ordering;
use std::collections::BinaryHeap;

const MIN_X: f64 = 0.;
const MAX_X: f64 = 1.;
const MIN_Y: f64 = 0.;
const MAX_Y: f64 = 1.;

#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Particle {
    px: usize,
    py: usize,
    vx: usize,
    vy: usize,
}

#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Collision {
    time: usize,
    particles: (Particle, Particle),
}

impl Ord for Collision {
    fn cmp(&self, other: &Self) -> Ordering {
        // Notice that the we flip the ordering on costs.
        // In case of a tie we compare positions - this step is necessary
        // to make implementations of `PartialEq` and `Ord` consistent.
        other.time.cmp(&self.time)
    }
}

impl PartialOrd for Collision {
    fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
        Some(self.cmp(other))
    }
}

pub struct EventDrivenGas {
    pq: BinaryHeap<Collision>,
    particles: Vec<Particle>,
}

impl EventDrivenGas {
    pub fn new() -> Self {
        let pq = BinaryHeap::new();
        let particles = vec![];
        Self { pq, particles }
    }
}

#[cfg(test)]
mod tests {
    #[test]
    fn it_works() {
        assert_eq!(1, 1);
    }
}
