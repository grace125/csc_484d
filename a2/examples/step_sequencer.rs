use bevy::prelude::*;
use bevy_egui::{egui, EguiContext, EguiPlugin};
use a2::bevy_midi::output::*;
use std::time::Duration;

fn main() {
    App::new()
        .add_plugins(DefaultPlugins)
        .add_plugin(EguiPlugin)
        .insert_resource(MidiOutputSettings {
            port_name: "interactive_example",
        })
        .add_plugin(MidiOutputPlugin)
        .init_resource::<StepSequencer>()
        .add_system(play)
        .add_system(step_sequencer_windows.after(play))
        .add_system(midi_port_window.after(step_sequencer_windows))
        .add_system(main_window.after(midi_port_window))
        .run()
}

fn step_sequencer_windows(
    mut egui_context: ResMut<EguiContext>,
    mut step_sequencer: ResMut<StepSequencer>,
    player: Option<ResMut<StepSequencePlayer>>,
) {

    for (i, mut layer) in step_sequencer.layers.iter_mut().enumerate() {
        egui::Window::new(format!("Step Sequencer {:?}", i+1)).show(egui_context.ctx_mut(), |ui| {
            for row in layer.data.iter_mut() {
                ui.horizontal(|ui| {
                    for (i, b) in row.iter_mut().enumerate() {
                        let visuals = ui.visuals_mut();

                        visuals.widgets.inactive.bg_fill =

                            match (&player, &b) {
                                (Some(player), _) if player.index == Some(i) => 
                                    if *b  { egui::Color32::from_rgb(0, 255, 255) }
                                    else   { egui::Color32::GREEN },
                                _ => if *b { egui::Color32::BLUE }
                                     else  { egui::Color32::BLACK }
                            };
                        if ui.button("     ").clicked() {
                            *b = !*b;
                        }
                    }
                });
            }

            ui.horizontal(|ui| {
                ui.label(format!("Channel: {:?}", layer.channel + 1));
                if ui.button("-").clicked() && layer.channel > 0 {
                    layer.channel -= 1;
                }
                if ui.button("+").clicked() && layer.channel < 15 {
                    layer.channel += 1;
                }
            });

            let mut s = format!("{:?}", layer.velocity);
            ui.label("Velocity: ");
            ui.add(egui::TextEdit::singleline(&mut s));
            if let Ok(v) = s.parse::<u8>() {
                layer.velocity = v;
            }

            let mut s = format!("{:?}", layer.lowest_value);
            ui.label("Lowest Value: ");
            ui.add(egui::TextEdit::singleline(&mut s));
            if let Ok(v) = s.parse::<u8>() {
                layer.lowest_value = v;
            }
        });
    }    
}

fn main_window(
    mut egui_context: ResMut<EguiContext>,
    mut step_sequencer: ResMut<StepSequencer>,
    mut commands: Commands,
    player: Option<ResMut<StepSequencePlayer>>,
    output: Res<MidiOutput>
) {
    egui::Window::new("").show(egui_context.ctx_mut(), |ui| {

        ui.horizontal(|ui| {
            if ui.button("Play").clicked() {
                commands.insert_resource(StepSequencePlayer {
                    timer: Timer::new(Duration::from_secs_f32(60.0/step_sequencer.tempo), TimerMode::Repeating),
                    index: None,
                });
            }
            if ui.button("Stop").clicked() {
                commands.remove_resource::<StepSequencePlayer>();
                
                // Stop all currently playing notes
                if let Some(player) = player {
                    if let Some(index) = player.index {
                        for layer in step_sequencer.layers.iter() {
                            for ev in layer.note_offs_at(index) {
                                output.send(ev);
                            }
                        }
                    }
                }
            }
        });

        let mut s = format!("{:?}", step_sequencer.tempo);
        ui.label("Tempo (bpm)");
        ui.add(egui::TextEdit::singleline(&mut s));
        if let Ok(t) = s.parse::<f32>() {
            if t > 0.001 {
                step_sequencer.tempo = t;
            }
        }

        if ui.button("Add Sequencer Layer").clicked() {
            step_sequencer.layers.push(StepSequenceLayer::default())
        }
    });
}

fn midi_port_window(
    mut egui_context: ResMut<EguiContext>,
    output: Res<MidiOutput>,
) {
    egui::Window::new("Port").show(egui_context.ctx_mut(), |ui| {
        if ui.button("Refresh Ports").clicked() {
            output.refresh_ports();
        }
        for (label, port) in output.ports() {
            if ui.button(label).clicked() {
                output.connect(port.clone());
            }
        }
    });
}

fn play(
    mut player: Option<ResMut<StepSequencePlayer>>,
    step_sequencer: Res<StepSequencer>,
    output: Res<MidiOutput>,
    time: Res<Time>,
) {
    if let Some(mut player) = player {
        match player.index {
            Some(index) => {
                if player.timer.tick(time.delta()).just_finished() {

                    let next_index = (index + 1) % 16;
                    for layer in step_sequencer.layers.iter() {
                        for ev in layer.note_offs_at(index) {
                            output.send(ev);
                        }
                        for ev in layer.note_ons_at(next_index) {
                            output.send(ev);
                        }
                    }

                    player.index = Some(next_index);
                }
            }
            None => {
                player.index = Some(0);
                for layer in step_sequencer.layers.iter() {
                    for ev in layer.note_ons_at(0) {
                        output.send(ev);
                    }
                }
            }
        }
    }
}

#[derive(Clone)]
struct StepSequenceLayer {
    data: [[bool; 16]; 16],
    channel: u8,
    lowest_value: u8,
    velocity: u8
}

impl Default for StepSequenceLayer {
    fn default() -> Self {
        StepSequenceLayer {
            data: default(),
            channel: 0,
            lowest_value: 48,
            velocity: 0x64,
        }
    }
}

impl StepSequenceLayer {
    fn note_offs_at(&self, index: usize) -> Vec<[u8; 3]> {
        let mut v = Vec::new();
        for j in 0..16 {
            if self.data[j][index] {
                v.push([
                    0b10000000 | (self.channel & 0b00001111), 
                    (((15 - j) + self.lowest_value as usize) % 256) as u8, 
                    self.velocity
                ]);
            }
        }
        v
    }
    fn note_ons_at(&self, index: usize) -> Vec<[u8; 3]> {
        let mut v = Vec::new();
        for j in 0..16 {
            if self.data[j][index] {
                v.push([
                    0b10010000 | (self.channel & 0b00001111), 
                    (((15 - j) + self.lowest_value as usize) % 256) as u8, 
                    self.velocity
                ]);
            }
        }
        v
    }
}

#[derive(Resource, Clone)]
struct StepSequencer {
    layers: Vec<StepSequenceLayer>,
    tempo: f32,
}

impl Default for StepSequencer {
    fn default() -> Self {
        StepSequencer {
            layers: Vec::new(),
            tempo: 240.0
        }
    }
}

#[derive(Resource, Clone)]
struct StepSequencePlayer {
    timer: Timer,
    index: Option<usize>,
}