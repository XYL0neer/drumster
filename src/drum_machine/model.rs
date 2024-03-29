#[derive(Debug, Clone)]
pub struct DrumMachine {
    pub bpm: u32,
    pub beats: u8,
    pub base: u8,
    pub resolution: u32,
    pub tracks: Vec<Track>,
}

#[derive(Debug, Clone)]
pub struct Track {
    pub instrument: Instrument,
    pub volume: f32,
    pub triggers: Vec<u32>,
}

#[derive(Debug, Clone)]
pub enum Instrument {
    Kick,
    Snare,
    HiHat,
}

impl Instrument {
    pub fn to_str(&self) -> &'static str {
        match self {
            Instrument::Kick => "Kick",
            Instrument::Snare => "Snare",
            Instrument::HiHat => "HiHat",
        }
    }
}
