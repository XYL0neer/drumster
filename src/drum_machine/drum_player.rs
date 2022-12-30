use crate::drum_machine::model::{DrumMachine, Instrument};
use rodio::Sink;
use std::io::BufReader;
use std::thread;
use std::time::Duration;

pub fn play_drum_machine(drum_machine: DrumMachine) {
    let mut strokes: u32 = 0;
    let (strokes_per_tact, stroke_duration) = calculate_duration_per_hit(&drum_machine);

    println!(
        "There are {} strokes in a tact and each stroke has a duration of {} milliseconds",
        strokes_per_tact,
        stroke_duration.as_millis()
    );

    let mut threads = vec![];
    loop {
        println!("Hit {}", strokes);
        for track in &drum_machine.tracks {
            if track.triggers.contains(&strokes) {
                let sound_file = sound_file_for_instrument(&track.instrument);
                threads.push(thread::spawn(move || {
                    let (_stream, handle) = rodio::OutputStream::try_default().unwrap();
                    let sink = Sink::try_new(&handle).unwrap();
                    play_sound(&sink, &sound_file);
                    sink.sleep_until_end();
                }));
            }
        }
        strokes += 1;
        if strokes >= strokes_per_tact {
            strokes = 0;
        }
        thread::sleep(stroke_duration);
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

fn calculate_duration_per_hit(drum_machine: &DrumMachine) -> (u32, Duration) {
    let stroke_per_beat = drum_machine.resolution as f64 / drum_machine.base as f64;
    println!("{} strokes per beat", stroke_per_beat);
    let stroke_per_tact = stroke_per_beat as u32 * drum_machine.beats as u32;
    println!("{} strokes per tact", stroke_per_tact);

    let bpm_freq = 60.0 / drum_machine.bpm as f64;
    let stroke_duration = bpm_freq as f64 / stroke_per_beat;
    let stroke_duration_millis = stroke_duration * 1_000.0;
    let stroke_duration_millis = stroke_duration_millis.round() as u64;
    let stroke_duration = Duration::from_millis(stroke_duration_millis);

    (stroke_per_tact, stroke_duration)
}
