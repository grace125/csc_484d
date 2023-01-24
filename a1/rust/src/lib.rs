use rodio::Source;
use std::time::Duration;

pub struct SineWave {
    current_sample: u32,
    sample_rate: u32,
    hertz: f32,
}

impl SineWave {
    pub fn new(sample_rate: u32, hertz: f32) -> Self {
        SineWave {
            current_sample: 0,
            sample_rate,
            hertz,
        }
    }
}

impl Iterator for SineWave {
    type Item = f32;

    fn next(&mut self) -> Option<f32> {
        let t = self.current_sample as f32 / self.sample_rate as f32;
        self.current_sample += 1;
        let sample = (std::f32::consts::TAU * self.hertz * t).sin();
        Some(sample)
    }
}

impl Source for SineWave {
    fn current_frame_len(&self) -> Option<usize> {
        None
    }
    fn channels(&self) -> u16 {
        1
    }
    fn sample_rate(&self) -> u32 {
        self.sample_rate
    }
    fn total_duration(&self) -> Option<Duration> {
        None
    }
}

pub struct SquareWave {
    current_sample: u32,
    sample_rate: u32,
    hertz: f32,
}

impl SquareWave {
    pub fn new(sample_rate: u32, hertz: f32) -> Self {
        SquareWave {
            current_sample: 0,
            sample_rate,
            hertz,
        }
    }
}

impl Iterator for SquareWave {
    type Item = f32;

    fn next(&mut self) -> Option<f32> {
        let t = self.current_sample as f32 / self.sample_rate as f32;
        self.current_sample += 1;
        let sample = if (t * self.hertz) % 1.0 < 0.5 { 1.0 } else { -1.0 };
        Some(sample)
    }
}

impl Source for SquareWave {
    fn current_frame_len(&self) -> Option<usize> {
        None
    }
    fn channels(&self) -> u16 {
        1
    }
    fn sample_rate(&self) -> u32 {
        self.sample_rate
    }
    fn total_duration(&self) -> Option<Duration> {
        None
    }
}

// FIXME: rename to triangle
pub struct SawtoothWave {
    current_sample: u32,
    sample_rate: u32,
    hertz: f32,
}

impl SawtoothWave {
    pub fn new(sample_rate: u32, hertz: f32) -> Self {
        SawtoothWave {
            current_sample: 0,
            sample_rate,
            hertz,
        }
    }
}

impl Iterator for SawtoothWave {
    type Item = f32;

    fn next(&mut self) -> Option<f32> {
        let hert_seconds = self.current_sample as f32 / self.sample_rate as f32 * self.hertz;
        self.current_sample += 1;
        let sample = 
            (if hert_seconds % 1.0 < 0.5 { 1.0 } else { -1.0 })
            * 4.0 * ((hert_seconds % 0.5) - 0.25);               // Slope
        Some(sample)
    }
}

impl Source for SawtoothWave {
    fn current_frame_len(&self) -> Option<usize> {
        None
    }
    fn channels(&self) -> u16 {
        1
    }
    fn sample_rate(&self) -> u32 {
        self.sample_rate
    }
    fn total_duration(&self) -> Option<Duration> {
        None
    }
}
