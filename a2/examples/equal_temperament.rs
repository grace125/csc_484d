use std::time::Duration;

use a2::prelude::*;
use rodio::{dynamic_mixer, OutputStream, Sink, Source};

fn main() {
    let (_stream, stream_handle) = OutputStream::try_default().unwrap();
    let sink = Sink::try_new(&stream_handle).unwrap();

    let asdr = Envelope::adsr(0.8, 0.1, 0.1, 0.5, 0.2, 0.1);
    let wavetable = cool_wave.wavetable(1024);

    for tet in 3..=12 {
        println!("{:?} tet", tet);

        let step = 2.0f32.powf(1.0 / tet as f32);

        for i in (0..=tet).into_iter().chain((0..tet).rev()) {
            let (controller, mixer) = dynamic_mixer::mixer::<f32>(2, 44100);
            let source = wavetable.source(44100).with_frequency(440.0 * step.powi(i));
            let source = asdr.source_from(source).amplify(0.2);
            controller.add(source);
            let source = wavetable.source(44100).with_frequency(440.0);
            let source = asdr.source_from(source).amplify(0.2);
            controller.add(source);
            sink.append(mixer);
        }

        sink.sleep_until_end();
    }
}

fn cool_wave(t: f32) -> f32 {
    0.5 * wave::sin(t) + 0.3 * wave::sin(t + 2.0) + 0.2 * wave::sin(t + 5.0)
}
