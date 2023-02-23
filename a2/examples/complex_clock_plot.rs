use std::f32::consts::{TAU, FRAC_PI_2};

use a2::complex::*;
use plotters::coord::types::RangedCoordf32;
use plotters::prelude::*;

// Outputs a plot of the complex numbers i, 2 + i, and i * (2+i)
fn main() -> Result<(), Box<dyn std::error::Error>> {
    let root = BitMapBackend::new("q8.png", (300, 300)).into_drawing_area();
    root.fill(&WHITE)?;

    let hour = 1.0;
    let min = 15.0;
    let sec = 20.0;

    let z_hour = Complex::from_polar(2.0, hour / 12.0 * TAU - FRAC_PI_2);
    let z_min =  Complex::from_polar(3.0, min / 60.0 * TAU - FRAC_PI_2);
    let z_sec =  Complex::from_polar(3.0, sec / 60.0 * TAU - FRAC_PI_2);

    let root = root.apply_coord_spec(Cartesian2d::<RangedCoordf32, RangedCoordf32>::new(
        -4f32..4f32,
        -4f32..4f32,
        (0..300, 0..300),
    ));

    root.draw(&Complex::default().as_labelled_point(BLACK, "Origin"))?;
    root.draw(&z_hour.as_labelled_point(BLACK, "Hour"))?;
    root.draw(&z_min.as_labelled_point(BLACK, "Minute"))?;
    root.draw(&z_sec.as_labelled_point(BLACK, "Second"))?;

    let one_sec_clockwise = Complex::from_polar(1.0, 1.0 / 60.0 * TAU - FRAC_PI_2);
    let one_sec_counter_clockwise = Complex::from_polar(1.0, -1.0 / 60.0 * TAU - FRAC_PI_2);

    println!("Complex required to move one second clockwise: {:?}", one_sec_clockwise);
    println!("Complex required to move one second counter-clockwise: {:?}", one_sec_counter_clockwise);

    Ok(())
}
