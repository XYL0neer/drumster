use crate::drum_machine::model::{DrumMachine, Instrument, Track};
use std::collections::vec_deque::VecDeque;

pub fn parse_csv(file_name: String) -> DrumMachine {
    let csv_content =
        std::fs::read_to_string(file_name).expect("Should have been able to read the file");

    let mut lines = csv_content.trim().lines().collect::<VecDeque<&str>>();

    let config_line = lines.pop_front().expect("File has no first line");

    let (bpm, config_line) = next_element(config_line, None)
        .expect("Elements after bpm missing: beats, base, resolution, instruments");
    let (beats, config_line) = next_element(config_line, None)
        .expect("Elements after beats missing: base, resolution, instruments");
    let (base, config_line) = next_element(config_line, None)
        .expect("Elements after base missing: resolution, instruments");
    let resolution = config_line;

    let tracks = lines
        .iter()
        .map(|line| {
            let track_line: Vec<&str> = line.split(";").collect();
            Track {
                instrument: match track_line[0] {
                    "Kick" => Instrument::Kick,
                    "Snare" => Instrument::Snare,
                    "HiHat" => Instrument::HiHat,
                    _ => panic!("Oh nooo"),
                },
                triggers: track_line[1]
                    .split(",")
                    .map(|x| x.parse::<u32>().unwrap())
                    .collect(),
                volume: track_line.get(2).unwrap().parse().unwrap_or(1.0),
            }
        })
        .collect();

    let drum_machine = DrumMachine {
        beats: beats.parse().unwrap(),
        base: base.parse().unwrap(),
        bpm: bpm.parse().unwrap(),
        resolution: resolution.parse().unwrap(),
        tracks,
    };

    drum_machine
}

/// Returns the string slices before and after the first occurrence of the delimiter
/// * `line` string slice to split
/// * `delimiter` optional delimiter to split at. The default delimiter is ";"
fn next_element(line: &str, delimiter: Option<char>) -> Result<(&str, &str), ()> {
    match line.split_once(delimiter.unwrap_or(';')) {
        None => Err(()),
        Some((left, right)) => Ok((left, right)),
    }
}
