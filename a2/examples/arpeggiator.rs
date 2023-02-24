use crossbeam_channel::select;
use midir::{MidiInput, MidiOutput, MidiOutputPort};
use rand::Rng;
use std::sync::Arc;
use std::time::Duration;
use std::{
    error::Error,
    io::{stdin, stdout, Write},
    sync::Mutex,
};

fn main() {
    let (sender, receiver) = crossbeam_channel::unbounded::<Event>();
    let (sender_play, receiver_play) = crossbeam_channel::unbounded::<()>();
    let mut input = String::new();

    let mut midi_in = MidiInput::new("Arpeggiator-in").unwrap();
    let in_ports = midi_in.ports();
    let in_port = match in_ports.len() {
        0 => panic!("no input port found"),
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
            stdout().flush().unwrap();
            let mut input = String::new();
            stdin().read_line(&mut input).unwrap();
            in_ports
                .get(input.trim().parse::<usize>().unwrap())
                .ok_or("invalid input port selected")
                .unwrap()
        }
    };

    let s = sender.clone();
    let s_play = sender_play.clone();
    let conn_in = midi_in
        .connect(
            in_port,
            "Arpeggiator input",
            move |_, message, _| match message[0] & 0b11110000 {
                0b10010000 => {
                    s.send(Event::NoteAdd(message[1]));
                    s_play.send(());
                }
                0b10000000 => {
                    s.send(Event::NoteRemove(message[1]));
                    s_play.send(());
                }
                _ => {}
            },
            (),
        )
        .unwrap();

    let midi_out = MidiOutput::new("Arpeggiator-out").unwrap();
    let out_ports = midi_out.ports();
    let out_port: &MidiOutputPort = match out_ports.len() {
        0 => panic!("no output port found"),
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
            stdout().flush().unwrap();
            let mut input = String::new();
            stdin().read_line(&mut input).unwrap();
            out_ports
                .get(input.trim().parse::<usize>().unwrap())
                .ok_or("invalid output port selected")
                .unwrap()
        }
    };

    println!("\nOpening connection");
    let mut conn_out = midi_out.connect(out_port, "Arpeggiator-output").unwrap();

    // Command Interface for changing tempo
    std::thread::spawn(move || {
        println!("\nType \"help\" for options.");

        loop {
            input.clear();
            stdin().read_line(&mut input);
            let mut iter = input.trim().split(' ');
            match iter.next() {
                Some("exit") => {
                    sender.send(Event::Exit);
                }
                Some("tempo") => {
                    let t = match iter.next() {
                        Some(t) => t,
                        None => {
                            println!("Too few arguments: expected float for tempo");
                            continue;
                        }
                    };
                    match t.parse::<f32>() {
                        Err(_) => {
                            println!("Improper argument: expected float for tempo");
                            continue;
                        }
                        Ok(tempo) => {
                            if tempo <= 0.0 {
                                println!("Improper argument: expected positive float for tempo");
                                continue;
                            }
                            sender.send(Event::TempoChange(tempo));
                        }
                    }
                }
                Some("mode") => {
                    let mode = match iter.next().map(|m| m.trim()) {
                        Some("up") => Mode::Up,
                        Some("down") => Mode::Down,
                        Some("up_down") => Mode::UpDown,
                        Some("random") => Mode::Random,
                        Some(_) => {
                            println!("Invalid argument: expected \"up\", \"down\", \"up_down\", or \"random\"");
                            continue;
                        }
                        None => {
                            println!("Too few arguments: expected \"up\", \"down\", \"up_down\", or \"random\"");
                            continue;
                        }
                    };

                    sender.send(Event::ModeChange(mode));
                }
                Some("help") => {
                    println!("Valid commands are \"help\", \"exit\", \"tempo <f32>\", and \"mode <up|down|up_down|random>\"");
                }
                _ => {}
            }
        }
    });

    let mut notes = Vec::<u8>::new();
    let mut tempo = 240.0;
    let mut mode = Mode::Up;
    let mut index = 0;
    let mut rng = rand::thread_rng();
    let mut is_up = true;
    loop {
        while let Ok(event) = receiver.try_recv() {
            match event {
                Event::Exit => break,
                Event::NoteAdd(new_note) => {
                    if notes.len() == 0 {
                        notes.push(new_note);
                    } else {
                        let i = notes.partition_point(|x| *x < new_note);

                        if i == notes.len() || notes[i] != new_note {
                            notes.insert(i, new_note);
                        }
                    }
                }
                Event::NoteRemove(old_note) => {
                    if let Ok(index) = notes.binary_search(&old_note) {
                        notes.remove(index);
                    };
                }
                Event::TempoChange(new_tempo) => tempo = new_tempo,
                Event::ModeChange(new_mode) => mode = new_mode,
            }
        }

        receiver_play.recv();

        if notes.len() == 0 {
            continue;
        }
        index = index.min(notes.len() - 1);
        match mode {
            Mode::Up => {
                index += 1;
                index %= notes.len();
            }
            Mode::Down => {
                index += notes.len() - 1;
                index %= notes.len();
            }
            Mode::Random => {
                index = rng.gen_range(0..notes.len());
            }
            Mode::UpDown => {
                if is_up && index == notes.len() - 1 {
                    is_up = false;
                } else if !is_up && index == 0 {
                    is_up = true;
                }

                if is_up {
                    index += 1;
                    index %= notes.len();
                } else {
                    index += notes.len() - 1;
                    index %= notes.len();
                }
            }
        }
        let note = notes[index];

        let _ = conn_out.send(&[0b10010000, note, 0x64]);
        std::thread::sleep(Duration::from_secs_f32(60.0 / tempo));
        let _ = conn_out.send(&[0b10000000, note, 0x64]);
        sender_play.send(());
    }

    conn_out.close();
}

enum Event {
    Exit,
    NoteAdd(u8),
    NoteRemove(u8),
    TempoChange(f32),
    ModeChange(Mode),
}

#[derive(Clone, Copy)]
enum Mode {
    Up,
    Down,
    UpDown,
    Random,
}
