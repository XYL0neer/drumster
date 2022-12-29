use crate::drum_player::play_drum_machine;
use crate::parse_drum_machine::DrumMachine;
use clap::Parser;

mod drum_player;
mod parse_drum_machine;

#[derive(Parser)]
#[clap(author, version, about)]
struct CLI {
    path: Option<String>,
}

fn main() {
    let cli = CLI::parse();

    let sample = cli.path.unwrap();

    println!("name: {:?}", sample);

    let sample = std::fs::read_to_string(sample).expect("Should have been able to read the file");

    let drum_machine = parse_drum_machine::parse_csv(&sample).unwrap();

    dbg!(&drum_machine);

    play_drum_machine(drum_machine);
}
