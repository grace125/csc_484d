use rodio::{OutputStream, Sink, Source};
use a1::*;
use std::time::Duration;



fn main() {
    let (_stream, stream_handle) = OutputStream::try_default().unwrap();
    let sink = Sink::try_new(&stream_handle).unwrap();

    let note_g4 =       2.0_f32.powf(10.0/12.0);
    let note_f4 =       2.0_f32.powf(8.0/12.0);
    let note_e4 =       2.0_f32.powf(7.0/12.0);
    let note_d4 =       2.0_f32.powf(5.0/12.0);
    let note_c4 =       2.0_f32.powf(3.0/12.0);
    let note_b_flat4 =  2.0_f32.powf(1.0/12.0);
    let note_a4 =       1.0_f32;
    let note_g3 =       note_g4 * 0.5;
    let note_f3 =       note_f4 * 0.5;
    let note_e3 =       note_e4 * 0.5;
    let note_d3 =       note_d4 * 0.5;
    let note_c3 =       note_c4 * 0.5;
    
    let wavetable = wave::pulse(0.25).wavetable(44100);
    let silence = (|t: f32| rand::random::<f32>()).wavetable(1).with_sample_rate(44100);

    for (note, dur) in [
        (Some(note_c4),       1.0),
        (Some(note_a4),       1.0),
        (Some(note_e3),       1.0),
        (Some(note_c3),       1.0),
        (Some(note_d3),       1.0/3.0),
        (Some(note_e3),       1.0/3.0),
        (Some(note_f3),       1.0/3.0),
        (Some(note_d3),       2.0/3.0),
        (Some(note_f3),       1.0/3.0),
        (Some(note_c3),       2.0),
        //
        (Some(note_d3),       1.0),
        (Some(note_b_flat4),  1.0),
        (Some(note_a4),       1.0),
        (Some(note_e3),       1.0),
        (Some(note_d3),       1.0/3.0),
        (Some(note_e3),       1.0/3.0),
        (Some(note_f3),       1.0/3.0),
        (Some(note_g3),       2.0/3.0),
        (Some(note_a4),       1.0/3.0),
        (Some(note_g3),       5.0/3.0),
        (Some(note_a4),       1.0/3.0),
        (Some(note_b_flat4),  1.0/3.0),
        (Some(note_a4),       1.0/3.0),
        (Some(note_g3),       1.0/3.0),
        //
        (Some(note_c4),       2.0/3.0),
        (Some(note_a4),       1.0/3.0),
        (Some(note_g3),       1.0/3.0),
        (Some(note_f3),       4.0/3.0),
        (Some(note_g3),       1.0/3.0),
        (Some(note_a4),       2.0/3.0),
        (Some(note_f3),       1.0/3.0),
        (Some(note_d3),       2.0/3.0),
        (Some(note_f3),       1.0/3.0),
        (Some(note_d3),       1.0/3.0),
        (Some(note_c3),       4.0/3.0),
        (Some(note_c3),       1.0/3.0),
        //
        (Some(note_f3),       2.0/3.0),
        (Some(note_a4),       1.0/3.0),
        (Some(note_g3),       1.0/3.0), 
        (None,                2.0/3.0),
        (Some(note_f3),       2.0/3.0),
        (Some(note_a4),       1.0/3.0),
        (Some(note_g3),       1.0/3.0),
        (Some(note_a4),       1.0/3.0),
        (Some(note_b_flat4),  1.0/3.0),
        (Some(note_c4),       1.0/3.0),
        (Some(note_a4),       1.0/3.0),
        (Some(note_f3),       1.0/3.0),
        (Some(note_g3),       2.0/3.0),
        (Some(note_c3),       1.0/3.0),
        (Some(note_f3),       1.0),
    ] {
        let source = match note {
            Some(key) => wavetable.source().with_frequency(440.0 * key),
            None      => silence.source()
        };
        sink.append(source.take_duration(Duration::from_secs_f32(dur)));
    }

    sink.sleep_until_end();
}