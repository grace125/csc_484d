use a2::prelude::*;
use rodio::{source::SineWave, OutputStream, Sink, Source};

fn main() {
    let (_stream, stream_handle) = OutputStream::try_default().unwrap();
    let sink = Sink::try_new(&stream_handle).unwrap();

    let peak_biquad = BiQuad::new(-1.99548123, 0.99940874, 1.00003607, -1.99548123, 0.99937266);
    let lowpass_biquad = BiQuad::new(
        -7.17336609e-17,
        0.17149959,
        0.29287490,
        0.58574979,
        0.29287490,
    );

    let adsr = Envelope::adsr(1.0, 0.2, 0.1, 0.6, 1.0, 0.3);
    let wavetable = wave::noise.wavetable(100_000);

    let pure_source = wavetable.source(44100).with_frequency(1.0);
    let peak_source = peak_biquad.source_from(pure_source.clone());
    let lowpass_source = lowpass_biquad.source_from(pure_source.clone());

    let mut max = 0.0;
    for (s1, s2) in adsr
        .source_from(pure_source)
        .zip(adsr.source_from(peak_source))
    {
        if (s1 - s2).abs() > max {
            max = (s1 - s2).abs();
        }
    }
    println!("{:?}", max);

    // sink.append(adsr.source_from(pure_source));
    // sink.append(adsr.source_from(peak_source));
    // sink.append(adsr.source_from(lowpass_source));

    sink.sleep_until_end();
}
