use std::io::{Error};

use super::model::{DrumMachine, Instrument, Track};

pub trait TriggerPattern {
    fn to_triggers(&self, beats: u8, base: u8, resolution: u32) -> Vec<u32>;
}

// OffBeat, // Hit between to beats
  //  OnBeat, // Hit on every beat
   // BackBeat, // Hit on 2 and 4
    //Shuffle, // Hit on Beat and 2 resolution lower before the beat

struct OnBeat;
impl TriggerPattern for OnBeat {
    fn to_triggers(&self, beats: u8, base: u8, resolution: u32) -> Vec<u32> {
        let stroke_per_beat = resolution as f64 / base as f64;
        println!("{} strokes per beat", stroke_per_beat);
        let stroke_per_tact = stroke_per_beat as u32 * beats as u32;
        vec![]
    }
}

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
            instrument,
            triggers,
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
