use rodio::{OutputStream, Sink, Source};
use a1::*;
use std::time::Duration;

// Plays a simple wave.
//
// wave::sin can be swapped out for any wave in wave.rs,
// or for an inline function Fn(f32) -> f32 which 
//   - returns values between -1 and 1;
//   - is periodic between 0 and 1.
//   - ex. (|t: f32| (t * t * 2.0) - 1.0).wavetable(44100)...


fn main() {
    let (_stream, stream_handle) = OutputStream::try_default().unwrap();
    let sink = Sink::try_new(&stream_handle).unwrap();

    sink.append(
        wave::sin.wavetable(44100).source()
            .with_frequency(440.0)
            .take_duration(Duration::from_secs_f32(3.0))
    );

    sink.sleep_until_end();
}