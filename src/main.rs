use core::time;
use std::string::String;
use std::borrow::Borrow;
use std::fmt::format;
use std::ops::Deref;
use std::{io::BufReader, thread};

use crate::parse_drum_machine::Instrument;
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

    let drum_machine = parse_drum_machine::parse_csv(&sample).unwrap();

    dbg!(&drum_machine);
    // TODO play music loop

    let mut timer: u32 = 0;
    let timings: u32 = 16;
    let mut threads = vec![];
    loop {
        println!("Hit {}", timer);
        // TODO check if timer is in any track and play that instrument
        for track in &drum_machine.tracks {
            if track.triggers.contains(&timer) {
                let sound_file = sound_file_for_instrument(&track.instrument);
                threads.push(thread::spawn(move|| {
                    let (_stream, handle) = rodio::OutputStream::try_default().unwrap();
                    let sink = Sink::try_new(&handle).unwrap();
                    play_sound(&sink, &sound_file);
                    sink.sleep_until_end();
                }));
            }
        }
        // TODO calculate when to reset timer due to end of takt
        timer += 1;
        if timer >= timings {
            timer = 0;
        }
        // TODO calculate time to sleep by bpm, base, and resolution
        thread::sleep(time::Duration::from_millis(175));
    }
}

fn play_sound(sink: &Sink, sound_file: &str) {
    let kick = std::fs::File::open(sound_file).unwrap();
    sink.append(rodio::Decoder::new(BufReader::new(kick)).unwrap());
}

fn sound_file_for_instrument(instrument: &Instrument) -> String {
    let inst_str = match instrument {
        Instrument::Kick => "Kick",
        Instrument::HiHat => "HiHat",
        Instrument::Snare => "Snare",
    };

    let sound_file = format!("sounds/{}.wav", inst_str);
    sound_file
}
