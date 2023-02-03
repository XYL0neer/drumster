use std::io::{stdout, Stdout};
use std::sync::mpsc::{channel, Receiver};
use std::thread;
use std::time::Duration;

use clap::Parser;
use crossterm::{execute, Result, terminal};
use crossterm::event::{Event, KeyCode, KeyEvent, poll, read};
use crossterm::terminal::{SetSize, size};

use crate::drum_machine::drum_player::play_drum_machine;
use crate::drum_machine::model::DrumMachine;
use crate::drum_machine::parse_drum_machine;
use crate::ui::render_drum_machine;

mod drum_machine;
mod ui;

#[derive(Parser)]
#[clap(author, version, about)]
struct CLI {
    path: Option<String>,
}


fn main() {
    let mut stdout = stdout();
    let cli = CLI::parse();
    let sample = cli.path.unwrap();
    println!("Play drums from file: {:?}", sample);

    let drum_machine = parse_drum_machine::parse_csv(&sample);
    let current_position = 0;
    render_drum_machine(&mut stdout, &drum_machine, current_position);

    let (sender, receiver) = channel();

    let copied = drum_machine.clone();
    thread::spawn(move || play_drum_machine(copied, sender));

    render_until_finished(&mut stdout, receiver, &drum_machine).unwrap();
}

fn render_until_finished(stdout: &mut Stdout, receiver: Receiver<u32>, drum_machine: &DrumMachine) -> Result<()> {
    let (cols, rows) = size()?;
    terminal::enable_raw_mode()?;
    execute!(stdout, SetSize(cols, rows))?;
    loop {
        let received = receiver.recv().unwrap();
        render_drum_machine(stdout, drum_machine, received);
        if poll(Duration::from_millis(50))? {
            match read_char()? {
                'q' => break,
                _ => {}
            }
        }
    }

    terminal::disable_raw_mode()
}

pub fn read_char() -> Result<char> {
    loop {
        if let Ok(Event::Key(KeyEvent {
                                 code: KeyCode::Char(c),
                                 ..
                             })) = read()
        {
            return Ok(c);
        }
    }
}