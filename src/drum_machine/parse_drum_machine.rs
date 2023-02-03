use crate::drum_machine::model::{DrumMachine, Instrument, Track};
use std::collections::vec_deque::VecDeque;

pub fn parse_csv(file_name: &str) -> DrumMachine {
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
    let (resolution, config_line) =
        next_element(config_line, None).expect("Elements after resolution missing: instruments");

    let tracks: Vec<Track> = config_line
        .split(';')
        .map(|inst| {
            let instrument = match inst {
                "Kick" => Ok(Instrument::Kick),
                "Snare" => Ok(Instrument::Snare),
                "HiHat" => Ok(Instrument::HiHat),
                _ => Err(()),
            };
            let line = lines.pop_front().unwrap_or_default();
            create_track(instrument.unwrap(), line)
        })
        .collect();

    if !lines.is_empty() {
        eprintln!(
            "There are {} more programming lines than instruments!",
            lines.len()
        );
    }

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

fn create_track(instrument: Instrument, line: &str) -> Track {
    let mut triggers: Vec<u32> = Vec::new();
    line.split(';').for_each(|x| {
        if let Ok(x) = x.parse() {
            triggers.push(x);
        }
    });
    Track {
        instrument,
        triggers,
    }
}
