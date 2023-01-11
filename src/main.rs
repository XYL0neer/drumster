use clap::Parser;

use crate::drum_machine::drum_player::play_drum_machine;
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
    dbg!(&drum_machine);

    play_drum_machine(drum_machine);
}
