use bevy::audio::Decodable;
use rodio::Source;
use std::sync::Arc;
use std::time::Duration;

/// A table which holds a waveform, and produces a [Source].
#[derive(Clone)]
pub struct WaveTable {
    data: Arc<Vec<f32>>,
}

impl WaveTable {
    /// Creates a new wavetable from a collection of samples.
    #[must_use]
    pub fn new(data: impl IntoIterator<Item = f32>) -> WaveTable {
        WaveTable {
            data: Arc::new(data.into_iter().collect()),
        }
    }

    /// Creates a new wavetable by sampling a function of the form `Fn(f32) -> f32`
    /// `sample_num` times between `0.0` and `1.0`.
    #[must_use]
    pub fn from_function<T: SourceFunctionExt>(sample_num: u32, function: T) -> WaveTable {
        let sample_num_float = sample_num as f32;
        WaveTable {
            data: Arc::new(
                (0..sample_num)
                    .map(|i| (function)(i as f32 / sample_num_float))
                    .collect(),
            ),
        }
    }

    /// Creates a [Source] from a wave table.
    #[must_use]
    pub fn source(&self, sample_rate: u32) -> WaveTableSource {
        WaveTableSource {
            table: self.clone(),
            index: 0.0,
            increment: 1.0,
            sample_rate,
        }
    }

    /// Samples the wave table at a continuous value.
    ///
    /// `index` should never be negative.
    #[must_use]
    pub fn sample(&self, index: f32) -> f32 {
        let left_index = index as usize % self.data.len();
        let right_index = (left_index + 1) % self.data.len();
        let weight = index % 1.0;

        self.data[left_index] * weight + self.data[right_index] * (1.0 - weight)
    }

    /// Gives a reference to the underlying data of the table.
    #[must_use]
    pub fn data(&self) -> &Vec<f32> {
        &self.data
    }
}

/// A [Source] of audio created by [WaveTable].
#[derive(Clone)]
pub struct WaveTableSource {
    table: WaveTable,
    index: f32,
    increment: f32,
    sample_rate: u32,
}

impl WaveTableSource {
    pub fn set_frequency(&mut self, freq: f32) {
        self.increment = freq * self.table.data.len() as f32 / self.sample_rate as f32;
    }

    #[must_use]
    pub fn with_frequency(mut self, freq: f32) -> Self {
        self.set_frequency(freq);
        self
    }
}

impl Iterator for WaveTableSource {
    type Item = f32;

    fn next(&mut self) -> Option<f32> {
        let result = Some(self.table.sample(self.index));
        self.index = (self.index + self.increment) % self.table.data.len() as f32;
        result
    }
}

impl Source for WaveTableSource {
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

impl Decodable for WaveTableSource {
    type Decoder = Self;

    type DecoderItem = f32;

    fn decoder(&self) -> Self::Decoder {
        self.clone()
    }
}

/// An extension trait for functions of the form `Fn(f32) -> f32`.
///
/// This extension traits expects functions to be periodic between `0.0` and `1.0`,
/// and for the function to output values between `-1.0` and `1.0`.
pub trait SourceFunctionExt: Fn(f32) -> f32 + 'static + Sized {
    fn source(self, sample_rate: u32) -> FunctionSource<Self>;

    fn wavetable(self, sample_num: u32) -> WaveTable {
        WaveTable::from_function(sample_num, self)
    }
}

impl<T: Fn(f32) -> f32 + 'static + Sized> SourceFunctionExt for T {
    fn source(self, sample_rate: u32) -> FunctionSource<Self> {
        FunctionSource {
            function: self,
            index: 0.0,
            increment: 1.0,
            sample_rate,
        }
    }
}

/// A source of audio created from a function of the form `Fn(f32) -> f32`.

pub struct FunctionSource<T: Fn(f32) -> f32 + 'static> {
    function: T,
    index: f32,
    increment: f32,
    sample_rate: u32,
}

impl<T: SourceFunctionExt> FunctionSource<T> {
    pub fn set_frequency(&mut self, freq: f32) {
        self.increment = freq / self.sample_rate() as f32;
    }

    #[must_use]
    pub fn with_frequency(mut self, freq: f32) -> Self {
        self.set_frequency(freq);
        self
    }
}

impl<T: SourceFunctionExt> Iterator for FunctionSource<T> {
    type Item = f32;

    fn next(&mut self) -> Option<f32> {
        let result = Some((self.function)(self.index));
        self.index = (self.index + self.increment) % 1.0;
        result
    }
}

impl<T: SourceFunctionExt> Source for FunctionSource<T> {
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

pub mod wave {
    #[must_use]
    pub fn sin(t: f32) -> f32 {
        (std::f32::consts::TAU * t).sin()
    }

    #[must_use]
    pub fn square(t: f32) -> f32 {
        if t < 0.5 {
            1.0
        } else {
            -1.0
        }
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
}

#[inline]
#[must_use]
/// Takes the dot product of two sources, maxing out at the given duration
pub fn dot_prod_duration<S1, S2>(source1: S1, source2: S2, duration: Duration) -> f32
where
    S1: Source<Item = f32>,
    S2: Source<Item = f32>,
{
    source1
        .take_duration(duration)
        .into_iter()
        .zip(source2.into_iter())
        .map(|(sample1, sample2)| sample1 * sample2)
        .reduce(|prod1, prod2| prod1 + prod2)
        .unwrap_or(0.0)
}

#[inline]
#[must_use]
/// Takes the dot product of two sources, maxing out at the given number of samples
pub fn dot_prod_samples<S1, S2>(source1: S1, source2: S2, samples: usize) -> f32
where
    S1: Source<Item = f32>,
    S2: Source<Item = f32>,
{
    source1
        .into_iter()
        .take(samples)
        .zip(source2.into_iter())
        .map(|(sample1, sample2)| sample1 * sample2)
        .reduce(|prod1, prod2| prod1 + prod2)
        .unwrap_or(0.0)
}
