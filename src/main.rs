use std::io::{stdin, stdout, Write};
use std::sync::mpsc::{channel, Receiver, Sender};
use std::thread;

use clap::Parser;
use crossterm::{cursor, execute, queue, terminal};
use crossterm::style::Print;

use crate::drum_machine::drum_player::play_drum_machine;
use crate::drum_machine::model::DrumMachine;
use crate::drum_machine::parse_drum_machine;

mod drum_machine;

#[derive(Parser)]
#[clap(author, version, about)]
struct CLI {
    path: Option<String>,
}

fn main() {
    let cli = CLI::parse();
    let sample = cli.path.unwrap();
    println!("Play drums from file: {:?}", sample);

    let drum_machine = parse_drum_machine::parse_csv(&sample);
    let mut current_position = 0;
    render_drum_machine(&drum_machine, current_position);


    let (sender, receiver) = channel();

    let copied = drum_machine.clone();
    let player_thread = thread::spawn(move || { play_drum_machine(copied, sender) });

    loop {
        current_position = receiver.recv().unwrap();
        render_drum_machine(&drum_machine, current_position);
    }
}

fn render_drum_machine(drum_machine: &DrumMachine, current_position: u32) {
    let mut stdout = stdout();
    execute!(stdout, terminal::Clear(terminal::ClearType::All)).unwrap();
    let stroke_per_beat = drum_machine.resolution as f64 / drum_machine.base as f64;
    let stroke_per_tact = stroke_per_beat as u32 * drum_machine.beats as u32;

    for track in drum_machine.tracks.iter() {
        let instrument_title = format!("{: <8}", track.instrument.to_str());
        let mut tact = String::new();
        for curr_stroke in 0..stroke_per_tact {
            if track.triggers.contains(&curr_stroke) {
                tact += "X";
            } else {
                tact += "-";
            }
            if curr_stroke == current_position {
                tact += "⎥";
            } else {
                tact += " ";
            }
        }

        queue!(stdout, Print(format!("{} | {}|\n", instrument_title, tact))).unwrap();
    }
    stdout.flush().unwrap();
}
