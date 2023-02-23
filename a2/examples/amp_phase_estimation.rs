use a2::prelude::*;
use plotters::{
    prelude::{BitMapBackend, ChartBuilder, IntoDrawingArea},
    series::LineSeries,
    style::{IntoFont, BLACK, BLUE, GREEN, RED, WHITE},
};
use rand::{Rng, SeedableRng};
use rand_chacha::ChaCha8Rng;
use rodio::{OutputStream, Sink, Source};
use std::{f32::consts::TAU, time::Duration};

// Randomly generates a wave from three sin waves with different amplitudes/phases:
// - graphs the three waves and their sum,
// - estimates the initial amplitudes/phases,
// - and plays the sum wave.
// NOTE: the rng is seeded, so it won't vary by running it different times.
fn main() -> Result<(), Box<dyn std::error::Error>> {
    // The seed of the rng; change the seed argument to get a different wave.
    let mut rng = ChaCha8Rng::seed_from_u64(2);

    let amp1 = rng.gen_range(0.1..1.0f32);
    let amp2 = rng.gen_range(0.1..1.0f32);
    let amp3 = rng.gen_range(0.1..1.0f32);

    let phase1 = rng.gen_range(0.0..TAU);
    let phase2 = rng.gen_range(0.0..TAU);
    let phase3 = rng.gen_range(0.0..TAU);

    let sin1 = sin_wave_from(amp1, phase1, 1);
    let sin2 = sin_wave_from(amp2, phase2, 2);
    let sin3 = sin_wave_from(amp3, phase3, 3);

    let root = BitMapBackend::new("q5.png", (1024, 768)).into_drawing_area();
    root.fill(&WHITE)?;

    let mut chart = ChartBuilder::on(&root)
        .x_label_area_size(35)
        .y_label_area_size(40)
        .right_y_label_area_size(40)
        .margin(5)
        .caption("Sin Waves", ("sans-serif", 50.0).into_font())
        .build_cartesian_2d(0f32..2f32, -2f32..2f32)?;

    chart
        .configure_mesh()
        .disable_x_mesh()
        .disable_y_mesh()
        .x_label_formatter(&|x| format!("{:.3}", x / 100.0))
        .draw()?;

    chart
        .draw_series(LineSeries::new(
            (0..=100).map(|t| (t as f32 / 50.0, sin1(t as f32 / 50.0))),
            &RED,
        ))?
        .label("100Hz Sin");

    chart
        .draw_series(LineSeries::new(
            (0..=100).map(|t| (t as f32 / 50.0, sin2(t as f32 / 50.0))),
            &BLUE,
        ))?
        .label("200Hz Sin");

    chart
        .draw_series(LineSeries::new(
            (0..=100).map(|t| (t as f32 / 50.0, sin3(t as f32 / 50.0))),
            &GREEN,
        ))?
        .label("300Hz Sin");

    let sin_sum = move |t| sin1(t) + sin2(t) + sin3(t);

    chart
        .draw_series(LineSeries::new(
            (0..=100).map(|t| (t as f32 / 50.0, sin_sum(t as f32 / 50.0))),
            &BLACK,
        ))?
        .label("Sum");

    let sin_sum_source = sin_sum.wavetable(1024).source(44100).with_frequency(100.0);

    // Estimates the amplitude and pitch of the sin wave contribution of the given
    // frequency
    // I'm not showing how the phase can be instead be estimated by brute force,
    // because that's less cool
    let calculate_amp_phase = |freq: f32, dim| {
        let sin = (|t: f32| (t * TAU).sin()).source(44100);
        let cos = (|t: f32| (t * TAU).cos()).source(44100);
        let sin_contribution = 2.0
            * dot_prod_samples(sin_sum_source.clone(), sin.with_frequency(freq), dim)
            / dim as f32;
        let cos_contribution = 2.0
            * dot_prod_samples(sin_sum_source.clone(), cos.with_frequency(freq), dim)
            / dim as f32;

        let amplitude =
            (sin_contribution * sin_contribution + cos_contribution * cos_contribution).sqrt();
        let phase = cos_contribution.atan2(sin_contribution).rem_euclid(TAU);

        (amplitude, phase)
    };

    let (approx_amp1, approx_phase1) = calculate_amp_phase(100.0, 10000);
    let (approx_amp2, approx_phase2) = calculate_amp_phase(200.0, 10000);
    let (approx_amp3, approx_phase3) = calculate_amp_phase(300.0, 10000);

    println!(
        "Initial amplitudes:   {amp1}, {amp2}, {amp3} \n\
         Initial phases:       {phase1}, {phase2}, {phase3} \n\
         \n\
         Estimated amplitudes: {approx_amp1}, {approx_amp2}, {approx_amp3} \n\
         Estimated phases:     {approx_phase1}, {approx_phase2}, {approx_phase3}"
    );

    // Play audio
    let (_stream, stream_handle) = OutputStream::try_default().unwrap();
    let sink = Sink::try_new(&stream_handle).unwrap();

    sink.append(
        sin_sum_source
            .with_frequency(400.0)
            .take_duration(Duration::from_secs_f32(3.0)),
    );

    sink.sleep_until_end();

    Ok(())
}

const fn sin_wave_from(amp: f32, phase: f32, freq_mul: u32) -> impl Fn(f32) -> f32 + 'static {
    let freq = freq_mul as f32;
    move |t| {
        let arg = t * TAU * freq + phase;
        amp * arg.sin()
    }
}
