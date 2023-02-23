use a2::prelude::*;
use bevy::utils::HashSet;
use midir::{Ignore, MidiInput, MidiOutput, MidiOutputPort};
use std::{
    error::Error,
    io::{stdin, stdout, Write},
    sync::Mutex,
};

fn main() -> Result<(), Box<dyn Error>> {
    let data = ArpeggiatorData::default();

    let mut midi_in = MidiInput::new("Arpeggiator-in")?;
    // midi_in.ignore(Ignore::None);

    let in_ports = midi_in.ports();
    let in_port = match in_ports.len() {
        0 => return Err("no input port found".into()),
        1 => {
            println!(
                "Choosing the only available input port: {}",
                midi_in.port_name(&in_ports[0]).unwrap()
            );
            &in_ports[0]
        }
        _ => {
            println!("\nAvailable input ports:");
            for (i, p) in in_ports.iter().enumerate() {
                println!("{}: {}", i, midi_in.port_name(p).unwrap());
            }
            print!("Please select input port: ");
            stdout().flush()?;
            let mut input = String::new();
            stdin().read_line(&mut input)?;
            in_ports
                .get(input.trim().parse::<usize>()?)
                .ok_or("invalid input port selected")?
        }
    };

    let in_port_name = midi_in.port_name(in_port)?;

    let mut midi_out = MidiOutput::new("Arpeggiator-out")?;

    let out_ports = midi_out.ports();

    let out_port: &MidiOutputPort = match out_ports.len() {
        0 => return Err("no output port found".into()),
        1 => {
            println!(
                "Choosing the only available output port: {}",
                midi_out.port_name(&out_ports[0]).unwrap()
            );
            &out_ports[0]
        }
        _ => {
            println!("\nAvailable output ports:");
            for (i, p) in out_ports.iter().enumerate() {
                println!("{}: {}", i, midi_out.port_name(p).unwrap());
            }
            print!("Please select output port: ");
            stdout().flush()?;
            let mut input = String::new();
            stdin().read_line(&mut input)?;
            out_ports
                .get(input.trim().parse::<usize>()?)
                .ok_or("invalid output port selected")?
        }
    };

    let mut conn_out = midi_out.connect(out_port, "midir-test")?;

    // _conn_in needs to be a named parameter, because it needs to be kept alive until the end of the scope
    // let _conn_in = midi_in.connect(
    //     in_port,
    //     "midir-read-input",
    //     move |stamp, message, _| {
    //         println!("{}: {:?} (len = {})", stamp, message, message.len());
    //     },
    //     (),
    // )?;

    // input.clear();
    // stdin().read_line(&mut input); // wait for next enter key press

    // println!("Closing connection");

    // println!(
    //     "Connection open, reading input from '{}' (press enter to exit) ...",
    //     in_port_name
    // );

    // std::thread::scope(|s| {
    //     // Midi input thread
    //     let out = s.spawn(|| {

    //     });
    //     s.spawn(|| {

    //     });
    // });

    Ok(())
}

#[derive(Default)]
struct ArpeggiatorData {
    notes: Mutex<Vec<u8>>,
    mode: Mutex<Mode>,
    sleep_time: Mutex<f32>,
}

enum Mode {
    Up,
    Down,
    UpDown,
    Random,
}

impl Default for Mode {
    fn default() -> Self {
        Mode::Up
    }
}

// fn main() {
//     App::new()
//         .add_plugins(DefaultPlugins)
//         .insert_resource(SineTable(wave::sin.wavetable(1024)))
//         .run();
// }

// #[derive(Resource, Deref, DerefMut)]
// struct SineTable(pub WaveTable);

// fn startup(table: Res<SineTable>, ) {
//     table.source(44100).with_frequency(440.0);
// }

// fn on_input(input: Res<Input<KeyCode>>) {

// }
