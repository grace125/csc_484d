use rodio::Source;
use std::time::Duration;
use std::sync::Arc;

#[derive(Clone)]
pub struct WaveTable {
    data: Arc<Vec<f32>>,
    sample_rate: u32,
}

impl WaveTable {
    
    pub fn new(data: Vec<f32>, sample_rate: u32) -> WaveTable {
        WaveTable { 
            data: Arc::new(data), 
            sample_rate 
        }
    }

    pub fn from_function<T: SourceFunctionExt>(sample_rate: u32, function: T) -> WaveTable {
        let sample_rate_float = sample_rate as f32;
        WaveTable {
            sample_rate,
            data: Arc::new(
                (0..sample_rate)
                    .map(|i| { (function)(i as f32 / sample_rate_float) })
                    .collect()
                )
        }
    }

    pub fn source(&self) -> WaveTableSource {
        WaveTableSource {
            table: self.clone(),
            index: 0.0,
            increment: 1.0,
        }
    }
    
    /// Samples the wave table at a continuous value.
    ///
    /// `index` should never be negative.
    pub fn sample(&self, index: f32) -> f32 {
        let left_index = index as usize;
        let right_index = (left_index + 1) % self.data.len();
        let weight = index % 1.0;
        let lerp = self.data[left_index] * weight + self.data[right_index] * (1.0 - weight);
        lerp
    }

    pub fn total_duration(&self) -> Duration {
        Duration::from_secs_f32(self.data.len() as f32 / self.sample_rate as f32)
    }

    pub fn sample_rate(&self) -> u32 {
        self.sample_rate
    }

    pub fn data(&self) -> &Vec<f32> {
        &self.data
    }
}
pub struct WaveTableSource {
    table: WaveTable,
    index: f32,
    increment: f32,
}

impl WaveTableSource {
    pub fn set_frequency(&mut self, freq: f32) {
        self.increment = freq * self.table.data.len() as f32 / self.sample_rate() as f32;
    }

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
        self.table.sample_rate
    }

    fn total_duration(&self) -> Option<Duration> {
        None
    }
}

pub trait SourceFunctionExt: Fn(f32) -> f32 + 'static + Sized {
    ///  `0.0` to `1.0`
    fn source(self, sample_rate: u32) -> FunctionSource<Self>;

    fn wavetable(self, sample_rate: u32) -> WaveTable {
        WaveTable::from_function(sample_rate, self)
    }
}

impl<T: Fn(f32) -> f32 + 'static + Sized> SourceFunctionExt for T {
    fn source(self, sample_rate: u32) -> FunctionSource<Self> {
        FunctionSource {
            function: self,
            index: 0.0,
            increment: 1.0,
            sample_rate
        }
    }
}

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
