use std::{io::BufReader, thread, time::Duration};

use clap::Parser;
use rodio::Sink;

pub mod parse_drum_machine;

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

    let play_machine = parse_drum_machine::parse_csv(&sample);

    dbg!(&play_machine);

    // concept for playing a sound
    // let mut threads = vec![];
    //
    // for x in sample.split(";") {
    //     println!("{}", x);
    //     if x == "kick" {
    //         instrument = x;
    //     } else {
    //         threads.push(thread::spawn(|| {
    //             let (_stream, handle) = rodio::OutputStream::try_default().unwrap();
    //
    //             let sink = rodio::Sink::try_new(&handle).unwrap();
    //             play_sound(&sink);
    //             sink.sleep_until_end();
    //         }));
    //         let duration = x.parse::<u64>().unwrap();
    //         thread::sleep(Duration::new(duration, 0));
    //     }
    // }
    // threads.into_iter().for_each(|t| t.join().unwrap());
}

fn play_sound(sink: &Sink) {
    let kick = std::fs::File::open("sounds/Bass-Drum-1.wav").unwrap();
    sink.append(rodio::Decoder::new(BufReader::new(kick)).unwrap());
}
