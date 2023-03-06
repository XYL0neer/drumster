use std::io::{Error, ErrorKind};

use super::model::{DrumMachine, Instrument, Track};

pub enum TriggerPattern {
    OffBeat,
    OnBeat,
    BackBeat,
    Shuffle,
}

impl TriggerPattern {
    pub fn to_triggers(self, strokes: u32, beats: u8) -> Vec<u32> {
        vec![]
    }
}

#[derive(Default)]
pub struct DrumMachineBuilder {
    pub bpm: Option<u32>,
    pub beats: Option<u8>,
    pub base: Option<u8>,
    pub resolution: Option<u32>,
    pub tracks: Vec<Track>,
}

impl DrumMachineBuilder {
    pub fn new() -> Self {
        Self {
            tracks: vec![],
            ..Default::default()
        }
    }

    pub fn set_bpm(mut self, bpm: u32) -> Self {
        self.bpm = Some(bpm);
        self
    }

    pub fn set_time_signature(mut self, beats: u8, base: u8) -> Self {
        self.beats = Some(beats);
        self.base = Some(base);
        self
    }

    pub fn set_resolution(mut self, resolution: u32) -> Self {
        self.resolution = Some(resolution);
        self
    }

    pub fn add_track(
        mut self,
        instrument: Instrument,
        pattern: TriggerPattern,
    ) -> Result<Self, Error> {
        if let None = self.beats {
            return Err(Error::new(
                ErrorKind::Other,
                "Set time signature before adding a track",
            ));
        }
        if let None = self.resolution {
            return Err(Error::new(
                ErrorKind::Other,
                "Set resolution before adding a Track",
            ));
        }
        let stroke_per_beat = self.resolution.unwrap() as f64 / self.base.unwrap() as f64;
        println!("{} strokes per beat", stroke_per_beat);
        let stroke_per_tact = stroke_per_beat as u32 * self.beats.unwrap() as u32;
        let triggers = pattern.to_triggers(stroke_per_tact, self.beats.unwrap());
        self.tracks.push(Track {
            instrument: instrument,
            volume: 1.0,
            triggers: triggers,
        });
        Ok(self)
    }

    pub fn build(self) -> DrumMachine {
        DrumMachine {
            bpm: self.bpm.unwrap(),
            beats: self.beats.unwrap(),
            base: self.base.unwrap(),
            resolution: self.resolution.unwrap(),
            tracks: self.tracks,
        }
    }
}
