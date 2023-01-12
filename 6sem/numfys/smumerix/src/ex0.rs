use rand;

pub fn start_point_sim(num_loops: usize) -> Vec<usize> {
    // xi is random number between -1 and 1
    // assume xi is greater than 0 for first step
    let mut sims = vec![0usize; num_loops];
    for idx in 0..num_loops {
        let mut pos: f64 = rand::random();
        let mut t = 1;
        while pos > 0.0 && t <= 101 {
            let xi: f64 = rand::random::<f64>() * 2.0 - 1.0;
            pos += xi;
            t += 1;
        }
        if t > 101 {
            // Very roughly estimate of time to return home because it does not matter for final result
            sims[idx] = t + (pos * 2.5) as usize;
        } else {
            sims[idx] = t;
        }
    }

    return sims;
}

pub fn probability_distribution(sim_result: &[usize]) -> Vec<f64> {
    let mut counts = vec![0; *sim_result.iter().max().unwrap() as usize + 1];
    for num in sim_result {
        counts[*num as usize] += 1;
    }
    let probs: Vec<f64> = counts
        .iter()
        .map(|val| *val as f64 / sim_result.len() as f64)
        .collect();
    return probs;
}

pub fn level_crossing_prob_sim(point: f64, num_loops: usize) -> Vec<usize> {
    let mut sims = vec![0; num_loops];
    for idx in 0..num_loops {
        let mut pos: f64 = 0.0;
        let mut t = 0;
        while pos < point && t <= 1001 {
            let xi: f64 = rand::random::<f64>() * 2.0 - 1.0;
            pos += xi;
            t += 1;
        }
        sims[idx] = t;

        if t > 1000 {
            // Very roughly estimate of time to return home because it does not matter for final result
            sims[idx] = t + ((pos - point).abs() * 3.5) as usize;
        } else {
            sims[idx] = t;
        }
    }
    return sims;
}
