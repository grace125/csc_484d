// TODO: implement and benchmark
#[must_use]
pub fn sin(t: f32) -> f32 {
    (std::f32::consts::TAU * t).sin()
}

#[must_use]
pub fn square(t: f32) -> f32 {
    if t < 0.5 { 1.0 } else { -1.0 }
}

#[must_use]
pub fn triangle(t: f32) -> f32 {
    (if t < 0.5 { 4.0 } else { -4.0 }) * ((t % 0.5) - 0.25)
}

#[must_use]
pub fn sawtooth(t: f32) -> f32 {
    (t - 0.5) * 2.0
}

#[must_use]
pub fn noise(_t: f32) -> f32 {
    (rand::random::<f32>() - 0.5) * 2.0
}

/// Gives a pulse wave function, given some "width" from `0.0` to `1.0` where the wave is 
/// git considered on.
#[must_use]
pub const fn pulse(width: f32) -> impl Fn(f32) -> f32 {
    move |t| if t < width { 1.0 } else { -1.0 }
}

