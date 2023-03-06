pub trait TriggerPattern {
    fn to_triggers(&self, beats: u8, base: u8, resolution: u32) -> Vec<u32>;
}

// OffBeat, // Hit between to beats
//  OnBeat, // Hit on every beat
// BackBeat, // Hit on 2 and 4
//Shuffle, // Hit on Beat and 2 resolution lower before the beat

pub struct OnBeat;

impl TriggerPattern for OnBeat {
    fn to_triggers(&self, beats: u8, base: u8, resolution: u32) -> Vec<u32> {
        let stroke_per_beat = resolution as f32 / base as f32;

        let mut triggers = vec![];
        for beat in 0..beats {
            triggers.push(beat as u32 * stroke_per_beat as u32);
        }
        triggers
    }
}

pub struct BackBeat;

impl TriggerPattern for BackBeat {
    fn to_triggers(&self, beats: u8, base: u8, resolution: u32) -> Vec<u32> {
        let stroke_per_beat = resolution as f32 / base as f32;

        let mut triggers = vec![];
        for beat in (1..beats).step_by(2) {
            triggers.push(beat as u32 * stroke_per_beat as u32);
        }
        triggers
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn on_beat_trigger() {
        let on_beat = OnBeat {};
        let result = on_beat.to_triggers(4, 4, 16);

        assert_eq!(result, vec![0, 4, 8, 12]);
    }

    #[test]
    fn back_beat_trigger() {
        let on_beat = BackBeat {};
        let result = on_beat.to_triggers(4, 4, 16);

        assert_eq!(result, vec![4, 12]);
    }
}
