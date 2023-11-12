//! bpm: beats per minute calculator
//! by Jon Jensen
//! inspired by Jörg in Chemnitz, Sachsen, Germany, 1992
//! 2023-11-11

use std::io::{stdin, stdout, Write};
use std::time::Instant;
use termion::event::{Event, Key};
use termion::input::TermRead;
use termion::raw::IntoRawMode;

const MIN_SECONDS_BETWEEN_BEATS: f32 = 0.1;

fn main() {
    println!("Press the space bar every time you hear the beat.");
    println!("Press the escape key during the last beat to show how many beats per minute (bpm) there were and then reset.");
    println!("Press control-C to quit.\n");

    let stdin = stdin();
    let mut stdout = stdout().into_raw_mode().unwrap();

    let mut start_time = Instant::now();
    let mut last_time = start_time;
    let mut beat_count: u16 = 0;
    let mut printed: bool = false;

    for c in stdin.events() {
        let evt = c.unwrap();
        match evt {
            Event::Key(Key::Ctrl('c')) => break,
            Event::Key(Key::Esc) => {
                if printed {
                    print!("\r\n");
                }
                let elapsed_time = start_time.elapsed().as_secs_f32();
                let bpm = beat_count as f32 / elapsed_time * 60.0;
                print!(
                    "{:.1} bpm ({} beats in {:.1} seconds)\r\n",
                    bpm, beat_count, elapsed_time
                );
                beat_count = 0;
                printed = false;
            }
            Event::Key(Key::Char(' ')) => {
                let first_beat = beat_count < 1;
                if first_beat || last_time.elapsed().as_secs_f32() > MIN_SECONDS_BETWEEN_BEATS {
                    last_time = Instant::now();
                    if first_beat {
                        start_time = last_time;
                    }
                    beat_count += 1;
                    print!("👍");
                    printed = true;
                }
            }
            Event::Key(_) => {
                print!("👎");
                printed = true;
            }
            _ => {}
        }
        stdout.flush().unwrap();
    }
}
