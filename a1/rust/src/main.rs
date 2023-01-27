use rodio::{OutputStream, Sink, Source};
use a1::{SourceFunctionExt, wave};
use std::time::Duration;

// TODO: make examples for each assignment question

fn main() {
    let (_stream, stream_handle) = OutputStream::try_default().unwrap();
    let sink = Sink::try_new(&stream_handle).unwrap();

    let wavetable = wave::sawtooth.wavetable(44100);
    let source = 
        wavetable.source()
            .with_frequency(440.0)
            .take_duration(Duration::from_secs_f32(3.0));

    let source2 = wavetable.source().with_frequency(220.0);

    sink.append(source);
    sink.append(source2);

    sink.sleep_until_end();
}
