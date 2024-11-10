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

pub fn waves_gradient_animation(text: &str, duration: Duration, term_signal: &Arc<AtomicBool>) {
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

    while duration.as_millis() == 0 || start.elapsed() < duration {
        if term_signal.load(Ordering::Relaxed) {
            return;
        }
        
        write!(stdout, "{}", cursor::Goto(1, 1)).unwrap();
        let time = start.elapsed().as_secs_f32();
        
        for y in 1..=term.height {
            for x in 1..=term.width {
                if y == center_y && x >= text_start_x && x <= text_end_x {
                    continue;
                }

                let wave = (x as f32 * 0.1 + time * 2.0).sin();
                let wave2 = (y as f32 * 0.1 + time * 1.5).cos();
                let combined = wave + wave2;
                
                // Create rainbow effect
                let hue = (time * 0.2 + (x as f32 * 0.02) + (y as f32 * 0.02)) % 1.0;
                let saturation = 0.8;
                let value = ((combined + 2.0) / 4.0) * 0.8 + 0.2; // Keep some minimum brightness
                
                let (r, g, b) = hsv_to_rgb(hue, saturation, value);
                
                write!(
                    stdout,
                    "{}{}▓",
                    cursor::Goto(x, y),
                    color::Fg(color::Rgb(r, g, b))
                ).unwrap();
            }
        }

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

// Helper function to convert HSV to RGB
fn hsv_to_rgb(h: f32, s: f32, v: f32) -> (u8, u8, u8) {
    let h = h * 6.0;
    let i = h.floor();
    let f = h - i;
    let p = v * (1.0 - s);
    let q = v * (1.0 - s * f);
    let t = v * (1.0 - s * (1.0 - f));

    let (r, g, b) = match i as i32 % 6 {
        0 => (v, t, p),
        1 => (q, v, p),
        2 => (p, v, t),
        3 => (p, q, v),
        4 => (t, p, v),
        _ => (v, p, q),
    };

    ((r * 255.0) as u8, (g * 255.0) as u8, (b * 255.0) as u8)
}