use rodio::Source;
use std::time::Duration;
use std::sync::Arc;
use crate::SourceFunctionExt;

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
            data: Arc::new(data.into_iter().collect())
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
                    .map(|i| { (function)(i as f32 / sample_num_float) })
                    .collect()
                )
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
pub struct WaveTableSource {
    table: WaveTable,
    index: f32,
    increment: f32,
    sample_rate: u32
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