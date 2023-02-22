use std::io::{Stdout, Write};

use crossterm::style::Print;
use crossterm::{execute, queue, terminal};

use crate::drum_machine::model::DrumMachine;

pub fn render_drum_machine(stdout: &mut Stdout, drum_machine: &DrumMachine, current_position: u32) {
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
        queue!(stdout, crossterm::cursor::MoveToColumn(1)).unwrap();
    }
    stdout.flush().unwrap();
}
