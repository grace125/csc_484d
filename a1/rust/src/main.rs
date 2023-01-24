use rodio::{OutputStream, Sink, Source};
use std::time::Duration;
use a1::*;

fn main() {
    let (_stream, stream_handle) = OutputStream::try_default().unwrap();
    let sink = Sink::try_new(&stream_handle).unwrap();

    let source = SawtoothWave::new(44100, 440.0).take_duration(Duration::from_secs(5));
    sink.append(source);

    sink.sleep_until_end();
}
