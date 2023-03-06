use crate::drum_machine::trigger_strategy::TriggerPattern;
use std::io::Error;

use super::model::{DrumMachine, Instrument, Track};

#[derive(Default)]
pub struct DrumMachineBuilder {
    pub bpm: u32,
    pub beats: u8,
    pub base: u8,
    pub resolution: u32,
    pub tracks: Vec<Track>,
}

impl DrumMachineBuilder {
    pub fn new() -> Self {
        Self {
            bpm: 80,
            beats: 4,
            base: 4,
            resolution: 16,
            tracks: vec![],
        }
    }

    pub fn set_bpm(mut self, bpm: u32) -> Self {
        self.bpm = bpm;
        self
    }

    pub fn set_time_signature(mut self, beats: u8, base: u8) -> Self {
        self.beats = beats;
        self.base = base;
        self
    }

    pub fn set_resolution(mut self, resolution: u32) -> Self {
        self.resolution = resolution;
        self
    }

    pub fn add_track<R: TriggerPattern>(
        mut self,
        instrument: Instrument,
        pattern: R,
    ) -> Result<Self, Error> {
        let triggers = pattern.to_triggers(self.beats, self.base, self.resolution);
        self.tracks.push(Track {
            instrument: instrument,
            volume: 1.0,
            triggers: triggers,
        });
        Ok(self)
    }

    pub fn build(self) -> DrumMachine {
        DrumMachine {
            bpm: self.bpm,
            beats: self.beats,
            base: self.base,
            resolution: self.resolution,
            tracks: self.tracks,
        }
    }
}
