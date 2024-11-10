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

use crate::terminal::{Terminal, Cell};

pub fn rainbow_animation(text: &str, duration: Duration, term_signal: &Arc<AtomicBool>) {
    let start = Instant::now();
    let term = Terminal::new();
    let mut stdout = stdout();
    let colors = [
        color::Rgb(255, 0, 0),   // Red
        color::Rgb(255, 127, 0), // Orange
        color::Rgb(255, 255, 0), // Yellow
        color::Rgb(0, 255, 0),   // Green
        color::Rgb(0, 0, 255),   // Blue
        color::Rgb(75, 0, 130),  // Indigo
        color::Rgb(148, 0, 211), // Violet
    ];

    // Create 2D buffer
    let mut buffer = vec![
        vec![Cell { char: ' ', color: color::Rgb(0, 0, 0) }; term.width as usize];
        term.height as usize
    ];
    let mut prev_buffer = buffer.clone();

    // Setup terminal
    write!(
        stdout,
        "{}{}{}",
        termion::cursor::Hide,
        clear::All,
        termion::screen::ToAlternateScreen
    ).unwrap();

    while duration.as_millis() == 0 || start.elapsed() < duration {
        if term_signal.load(Ordering::Relaxed) {
            return;  // Just return, cleanup handled in main
        }
        
        let offset = (start.elapsed().as_millis() / 100) as usize;
        
        // Update buffer
        for y in 0..term.height as usize {
            for x in 0..term.width as usize {
                let color_idx = (x + offset) % colors.len();
                buffer[y][x] = Cell {
                    char: '*',
                    color: colors[color_idx],
                };
            }
        }

        // Add text in the center
        let (center_x, center_y) = term.center_pos();
        let start_x = (center_x - (text.len() as u16 / 2)) as usize;
        let y = center_y as usize - 1;
        for (i, c) in text.chars().enumerate() {
            if start_x + i < term.width as usize {
                buffer[y][start_x + i] = Cell {
                    char: c,
                    color: color::Rgb(255, 255, 255),
                };
            }
        }

        // Only write cells that changed
        for y in 0..term.height as usize {
            for x in 0..term.width as usize {
                if buffer[y][x].char != prev_buffer[y][x].char 
                   || buffer[y][x].color != prev_buffer[y][x].color {
                    write!(
                        stdout,
                        "{}{}{}",
                        cursor::Goto(x as u16 + 1, y as u16 + 1),
                        color::Fg(buffer[y][x].color),
                        buffer[y][x].char
                    ).unwrap();
                }
            }
        }
        
        stdout.flush().unwrap();
        prev_buffer = buffer.clone();
        
        sleep(Duration::from_millis(16));
    }
}