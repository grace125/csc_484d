// TODO: implement and benchmark
pub fn sin(t: f32) -> f32 {
    (std::f32::consts::TAU * t).sin()
}

pub fn square(t: f32) -> f32 {
    if t < 0.5 { 1.0 } else { -1.0 }
}

pub fn triangle(t: f32) -> f32 {
    (if t < 0.5 { 4.0 } else { -4.0 }) * ((t % 0.5) - 0.25)
}

pub fn sawtooth(t: f32) -> f32 {
    (t - 0.5) * 2.0
}

pub fn noise(_t: f32) -> f32 {
    (rand::random::<f32>() - 0.5) * 2.0
}

// TODO: implement pulse wave
