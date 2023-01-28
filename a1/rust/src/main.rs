use rodio::{OutputStream, Sink, Source};
use a1::{SourceFunctionExt, wave, WaveTable};
use std::time::Duration;
use serde::{Serialize, Deserialize};

// TODO: make examples for each assignment question

const SAMPLE_RATE: u32 = 44100;
const NUM_SINKS: usize = 4;

#[derive(Serialize, Deserialize, Clone, Debug)]
pub enum Wave {
    Sine,
    Triangle,
    Square,
    Sawtooth,
    Pulse
}

// A note, specified by start time, length, note value, and wave form (in that order)
#[derive(Serialize, Deserialize, Clone, Debug)]
pub struct Note(f32, f32, u8, Wave);

fn main() {

    let path = std::env::args().nth(1).expect("No file argument given");
    let file = std::fs::read_to_string(path).expect("No such file exists");
    let mut notes: Vec<Note> = ron::from_str(file.as_str()).expect("Error parsing file");

    notes.sort_by(
        |note_one, note_two| 
            note_one.0.partial_cmp(&note_two.0).unwrap().then(
                note_one.1.partial_cmp(&note_two.1).unwrap()
            )
    );

    let (_stream, stream_handle) = OutputStream::try_default().unwrap();

    let mut sinks: Vec<(Sink, f32)> = (0..NUM_SINKS).map(|_| (Sink::try_new(&stream_handle).unwrap(), 0.0)).collect();

    let wavetable_sin = wave::sin.wavetable(SAMPLE_RATE);
    let wavetable_triangle = wave::triangle.wavetable(SAMPLE_RATE);
    let wavetable_square = wave::square.wavetable(SAMPLE_RATE);
    let wavetable_sawtooth = wave::sawtooth.wavetable(SAMPLE_RATE);
    let wavetable_pulse = wave::pulse(0.25).wavetable(SAMPLE_RATE);
    let wavetable_silence = WaveTable::new([0.0], SAMPLE_RATE);

    for note in notes {
        let start_time = note.0;
        let duration = note.1;
        let note_value = note.2 as f32;
        let freq = 27.5 * 2.0_f32.powf((note_value - 21.0)/12.0);
        let table = match note.3 {
            Wave::Sine     => &wavetable_sin,
            Wave::Triangle => &wavetable_triangle,
            Wave::Square   => &wavetable_square,
            Wave::Sawtooth => &wavetable_sawtooth,
            Wave::Pulse    => &wavetable_pulse
        };
        let source = 
            table.source()
                .with_frequency(freq)
                .take_duration(Duration::from_secs_f32(duration));

        for (sink, sink_time) in sinks.iter_mut() {

            if *sink_time <= start_time {
                let pause = wavetable_silence
                    .source()
                    .take_duration(Duration::from_secs_f32(start_time - *sink_time));
                *sink_time = start_time + duration;
                sink.append(pause);
                sink.append(source);
                break
            }
        }
    }

    for (sink, _) in sinks.iter_mut() {
        sink.sleep_until_end();
    }
}
