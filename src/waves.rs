use std::{
    io::{stdout, Write},
    thread::sleep,
    time::{Duration, Instant},
};
use termion::{
    clear, color,
    cursor::{self},
};
use std::sync::atomic::{AtomicBool, Ordering};
use std::sync::Arc;

use crate::terminal::Terminal;

pub fn waves_animation(text: &str, duration: Duration, term_signal: &Arc<AtomicBool>) {
    let start = Instant::now();
    let term = Terminal::new();
    let mut stdout = stdout();
    let (center_x, center_y) = term.center_pos();
    
    // Calculate text boundaries
    let text_start_x = center_x - (text.len() as u16 / 2);
    let text_end_x = text_start_x + text.len() as u16;

    write!(
        stdout,
        "{}{}{}",
        termion::cursor::Hide,
        clear::All,
        termion::screen::ToAlternateScreen
    ).unwrap();

    while duration.as_millis() == 0 || start.elapsed() < duration  {
        if term_signal.load(Ordering::Relaxed) {
            return;  // Just return, cleanup handled in main
        }
        
        write!(stdout, "{}", cursor::Goto(1, 1)).unwrap();
        let time = start.elapsed().as_secs_f32();
        
        for y in 1..=term.height {
            for x in 1..=term.width {
                // Skip the area where the text is when on the text's row
                if y == center_y && x >= text_start_x && x <= text_end_x {
                    continue;
                }

                let wave = (x as f32 * 0.1 + time * 2.0).sin();
                let wave2 = (y as f32 * 0.1 + time * 1.5).cos();
                let combined = wave + wave2;
                
                let blue = (((combined + 2.0) / 4.0) * 255.0) as u8;
                
                write!(
                    stdout,
                    "{}{}▓",
                    cursor::Goto(x, y),
                    color::Fg(color::Rgb(0, 0, blue))
                ).unwrap();
            }
        }

        // Draw text only once per frame
        write!(
            stdout,
            "{}{}{}",
            cursor::Goto(text_start_x, center_y),
            color::Fg(color::White),
            text
        ).unwrap();
        
        stdout.flush().unwrap();
        sleep(Duration::from_millis(32));
    }
}