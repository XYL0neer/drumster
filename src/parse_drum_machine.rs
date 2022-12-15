use std::collections::VecDeque;

#[derive(Debug)]
pub struct DrumMachine {
    beats: u8,
    base: u8,
    bpm: u32,
    resolution: u32,
    tracks: Vec<Track>,
}

#[derive(Debug)]
pub struct Track {
    instrument: Instrument,
    triggers: Vec<u32>,
}

#[derive(Debug)]
enum Instrument {
    Kick,
    Snare,
    HiHat,
}

// TODO error handling not yet implemented, only valid csv will work
pub fn parse_csv(csv_content: &String) -> DrumMachine {
    let mut lines = csv_content.lines().collect::<VecDeque<&str>>();

    let config_line = lines.pop_front().unwrap();

    let (beats, config_line) = next_element(config_line);
    let (base, config_line) = next_element(config_line);
    let (bpm, config_line) = next_element(config_line);
    let (resolution, config_line) = next_element(config_line);

    let tracks: Vec<Track> = config_line
        .split(';')
        .map(|inst| {
            let instrument = match inst {
                "Kick" => Ok(Instrument::Kick),
                "Snare" => Ok(Instrument::Snare),
                "HiHat" => Ok(Instrument::HiHat),
                _ => Err(()),
            };
            let line = lines.pop_front().unwrap();
            create_track(instrument.unwrap(), line)
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

fn next_element(line: &str) -> (&str, &str) {
    line.split_once(';').unwrap()
}

fn create_track(inst: Instrument, line: &str) -> Track {
    let line = line.split(';').map(|x| x.parse::<u32>().unwrap()).collect();
    Track {
        instrument: inst,
        triggers: line,
    }
}
