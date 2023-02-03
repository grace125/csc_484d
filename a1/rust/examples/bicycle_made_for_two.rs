use rodio::{OutputStream, Sink, Source};
use a1::*;
use std::time::Duration;



fn main() {
    let (_stream, stream_handle) = OutputStream::try_default().unwrap();
    let sink = Sink::try_new(&stream_handle).unwrap();

    let wavetable = wave::pulse(0.25).wavetable(44100);
    let silence = (|_: f32| rand::random::<f32>()).wavetable(1).with_sample_rate(44100);


    let play = |note: Option<u8>, duration| {
        let source = match note {
            Some(note_value) => {
                let freq = 27.5 * 2.0_f32.powf((note_value as f32 - 21.0)/12.0);
                wavetable.source().with_frequency(freq)
            },
            None      => silence.source()
        };
        sink.append(source.take_duration(Duration::from_secs_f32(duration)));
    };
    
    play(Some(72), 1.0);
    play(Some(69), 1.0);
    play(Some(64), 1.0);
    play(Some(60), 1.0);
    play(Some(62), 1.0/3.0);
    play(Some(64), 1.0/3.0);
    play(Some(65), 1.0/3.0);
    play(Some(62), 2.0/3.0);
    play(Some(65), 1.0/3.0);
    play(Some(60), 2.0);
    //
    play(Some(62), 1.0);
    play(Some(70), 1.0);
    play(Some(69), 1.0);
    play(Some(64), 1.0);
    play(Some(62), 1.0/3.0);
    play(Some(64), 1.0/3.0);
    play(Some(65), 1.0/3.0);
    play(Some(67), 2.0/3.0);
    play(Some(69), 1.0/3.0);
    play(Some(67), 5.0/3.0);
    play(Some(69), 1.0/3.0);
    play(Some(70), 1.0/3.0);
    play(Some(69), 1.0/3.0);
    play(Some(67), 1.0/3.0);
    //
    play(Some(72), 2.0/3.0);
    play(Some(69), 1.0/3.0);
    play(Some(67), 1.0/3.0);
    play(Some(65), 4.0/3.0);
    play(Some(67), 1.0/3.0);
    play(Some(69), 2.0/3.0);
    play(Some(65), 1.0/3.0);
    play(Some(62), 2.0/3.0);
    play(Some(65), 1.0/3.0);
    play(Some(62), 1.0/3.0);
    play(Some(60), 4.0/3.0);
    play(Some(60), 1.0/3.0);
    //
    play(Some(65), 2.0/3.0);
    play(Some(69), 1.0/3.0);
    play(Some(67), 1.0/3.0) ;
    play(None,     2.0/3.0);
    play(Some(65), 2.0/3.0);
    play(Some(69), 1.0/3.0);
    play(Some(67), 1.0/3.0);
    play(Some(69), 1.0/3.0);
    play(Some(70), 1.0/3.0);
    play(Some(72), 1.0/3.0);
    play(Some(69), 1.0/3.0);
    play(Some(65), 1.0/3.0);
    play(Some(67), 2.0/3.0);
    play(Some(60), 1.0/3.0);
    play(Some(65), 1.0);

    sink.sleep_until_end();
}