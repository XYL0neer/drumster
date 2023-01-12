use clap::Parser;
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
    render_drum_machine(&drum_machine);

    play_drum_machine(drum_machine);
}

fn render_drum_machine(drum_machine: &DrumMachine) {
    let stroke_per_beat = drum_machine.resolution as f64 / drum_machine.base as f64;
    let stroke_per_tact = stroke_per_beat as u32 * drum_machine.beats as u32;

    let offset = 8;

    let render_border = move|| println!("{}   {}", (0..offset).map(|_| " ").collect::<String>(),(0..stroke_per_tact).map(|_| "-").collect::<String>());
    render_border();

    for track in drum_machine.tracks.iter() {
        let instrument_title = format!("{: <8}", track.instrument.to_str());
        let mut tact = String::new();
        for curr_stroke in 0..stroke_per_tact {
            if track.triggers.contains(&curr_stroke) {
                tact += "X";
            } else {
                tact += "_";
            }
        }

        println!("{} | {} |", instrument_title, tact);
    }
    render_border();
}
