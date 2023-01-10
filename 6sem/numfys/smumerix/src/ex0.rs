use rand;

const DIST_SIZE: usize = 1000;

pub fn run_simulation() -> Vec<i32> {
    // xi is random number between -1 and 1
    // assume xi is greater than 0 for first step
    let mut times = [0; DIST_SIZE];
    for idx in 0..DIST_SIZE {
        let mut pos: f64 = rand::random();
        let mut t = 1;
        while pos > 0.0 && t <= 101 {
            let xi: f64 = rand::random::<f64>() * 2.0 - 1.0;
            pos += xi;
            t += 1;
        }
        if t > 101 {
            continue;
        }
        times[idx] = t;
    }

    return times.to_vec();
}

pub fn probability_distribution() -> Vec<f64> {
    let times = run_simulation();
    let mut counts = vec![0; *times.iter().max().unwrap() as usize + 1];
    for num in times {
        counts[num as usize] += 1;
    }
    let probs: Vec<f64> = counts
        .iter()
        .map(|val| *val as f64 / DIST_SIZE as f64)
        .collect();
    return probs;
}
