use rodio::{OutputStream, Sink};
use a1::*;

// TODO: make examples for each assignment question

fn main() {
    let (_stream, stream_handle) = OutputStream::try_default().unwrap();
    let sink = Sink::try_new(&stream_handle).unwrap();

    let source = wave::sawtooth.wavetable(44100).source().with_frequency(440.0);

    sink.append(source);

    sink.sleep_until_end();
}
