use plotters::prelude::*;

pub fn plot_vec(x_vals: Vec<f32>, y_vals: Vec<f32>) {
    let getmax = |vals: &Vec<f32>| vals.iter().cloned().reduce(f32::max).unwrap();
    let getmin = |vals: &Vec<f32>| vals.iter().cloned().reduce(f32::min).unwrap();
    let add_margin = |max: f32, min: f32| {
        let margin = (max - min) * 5_f32 / 100_f32;
        (max + margin, min - margin)
    };
    let x_max = getmax(&x_vals);
    let x_min = getmin(&x_vals);
    let (x_lim_max, x_lim_min) = add_margin(x_max, x_min);
    let y_max = getmax(&y_vals);
    let y_min = getmin(&y_vals);
    let (y_lim_max, y_lim_min) = add_margin(y_max, y_min);

    let root = BitMapBackend::new("./results/log.png", (640, 480)).into_drawing_area();
    root.fill(&WHITE).unwrap();
    let mut chart = ChartBuilder::on(&root)
        .caption("total surface energy", ("sans-serif", 50).into_font())
        .margin(5)
        .x_label_area_size(30)
        .y_label_area_size(50)
        .build_cartesian_2d(
            (x_lim_min..x_lim_max).log_scale(),
            (y_lim_min..y_lim_max).log_scale(),
        )
        .unwrap();

    chart
        .configure_mesh()
        .x_desc("Side length (m)")
        .y_desc("Total energy (J)")
        .draw()
        .unwrap();

    chart
        .draw_series(LineSeries::new(
            (0..x_vals.len()).map(|i| (*x_vals.get(i).unwrap(), *y_vals.get(i).unwrap())),
            &RED,
        ))
        .unwrap()
        .label("Total energy")
        .legend(|(x, y)| PathElement::new(vec![(x, y), (x + 20, y)], &RED));

    chart
        .configure_series_labels()
        .background_style(&WHITE.mix(0.8))
        .border_style(&BLACK)
        .draw()
        .unwrap();

    match root.present() {
        Ok(()) => println!("Printed successfully"),
        Err(err) => println!("Failed printing with {err}"),
    };
}
