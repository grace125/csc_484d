use a2::complex::*;
use plotters::coord::types::RangedCoordf32;
use plotters::prelude::*;
use plotters::style::full_palette::PURPLE;

// Outputs a plot of the complex numbers i, 2 + i, and i * (2+i)
fn main() -> Result<(), Box<dyn std::error::Error>> {
    let root = BitMapBackend::new("q2.png", (300, 300)).into_drawing_area();
    root.fill(&WHITE)?;

    let z1 = Complex::new(0.0, 1.0);
    let z2 = Complex::new(2.0, 1.0);
    let z3 = z1 * z2;

    let root = root.apply_coord_spec(Cartesian2d::<RangedCoordf32, RangedCoordf32>::new(
        -4f32..4f32,
        -4f32..4f32,
        (0..300, 0..300),
    ));

    root.draw(&Complex::default().as_labelled_point(BLACK, "Origin"))?;
    root.draw(&z1.as_labelled_point(RED, "A: i"))?;
    root.draw(&z2.as_labelled_point(BLUE, "B: 2+i"))?;
    root.draw(&(z3).as_labelled_point(PURPLE, "A * B"))?;

    Ok(())
}
