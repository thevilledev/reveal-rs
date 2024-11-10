use rand::Rng;
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

pub fn explosion_animation(text: &str, duration: Duration, term_signal: &Arc<AtomicBool>) {
    let start = Instant::now();
    let term = Terminal::new();
    let mut stdout = stdout();
    let mut rng = rand::thread_rng();

    // check if duration is 0 or if we've been running for longer than the duration
    while duration.as_millis() == 0 || start.elapsed() < duration {
        if term_signal.load(Ordering::Relaxed) {
            return;  // Just return, cleanup handled in main
        }
        
        write!(stdout, "{}", clear::All).unwrap();
        let time = start.elapsed().as_secs_f32();
        let radius = (time * 10.0) as u16;
        
        let (center_x, center_y) = term.center_pos();
        
        for angle in (0..360).step_by(5) {
            let x = center_x as f32 + (angle as f32).to_radians().cos() * radius as f32;
            let y = center_y as f32 + (angle as f32).to_radians().sin() * radius as f32;
            
            if x >= 0.0 && x < term.width as f32 && y >= 0.0 && y < term.height as f32 {
                let color = color::Rgb(
                    rng.gen_range(200..=255),
                    rng.gen_range(0..=100),
                    0,
                );
                write!(
                    stdout,
                    "{}{}*",
                    cursor::Goto(x as u16 + 1, y as u16 + 1),
                    color::Fg(color)
                )
                .unwrap();
            }
        }

        // Draw text in the center
        write!(
            stdout,
            "{}{}{}",
            cursor::Goto(center_x - (text.len() as u16 / 2), center_y),
            color::Fg(color::White),
            text
        )
        .unwrap();
        
        stdout.flush().unwrap();
        sleep(Duration::from_millis(50));
    }
}