mod plot;

fn mass_to_volume(mass: f32, density: f32) -> f32 {
    return mass / density;
}

fn sidelength_to_total_energy(sidelength: f32, volume: f32, surface_energy_density: f32) -> f32 {
    let vol_of_cube = sidelength.powi(3);
    let num_cubes = volume / vol_of_cube;
    let total_area = sidelength.powi(2) * 6_f32 * num_cubes;
    return total_area * surface_energy_density;
}

fn p1a() {
    let mass = 1_f32; // 1 g NaCl
    let density = 2.17; // g / cm^3
    let surface_energy_density = 2e-5; // J / cm^2

    let volume = mass_to_volume(mass, density);
    let mut energies = Vec::new();
    let mut sidelengths = Vec::new();

    for sidelength_times_100 in (1..=100).rev() {
        let sidelength = sidelength_times_100 as f32 * 0.01_f32;

        sidelengths.push(sidelength);

        energies.push(sidelength_to_total_energy(
            sidelength,
            volume,
            surface_energy_density,
        ));
    }

    for i in 0..energies.len().into() {
        let sidelength = sidelengths.get(i).unwrap();
        let energy = energies.get(i).unwrap();
        println!("{sidelength} cm: {energy}");
    }

    plot::plot_vec(sidelengths, energies);
}

fn main() {
    p1a();
}
