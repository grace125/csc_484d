use rodio::Source;
use std::time::Duration;
use crate::WaveTable;

/// An extension trait for functions of the form `Fn(f32) -> f32`.
/// 
/// This extension traits expects functions to be periodic between `0.0` and `1.0`,
/// and for the function to output values between `-1.0` and `1.0`.
pub trait SourceFunctionExt: Fn(f32) -> f32 + 'static + Sized {
    
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
