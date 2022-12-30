#[derive(Debug)]
pub struct DrumMachine {
    pub bpm: u32,
    pub beats: u8,
    pub base: u8,
    pub resolution: u32,
    pub tracks: Vec<Track>,
}

#[derive(Debug)]
pub struct Track {
    pub instrument: Instrument,
    pub triggers: Vec<u32>,
}

#[derive(Debug)]
pub enum Instrument {
    Kick,
    Snare,
    HiHat,
}