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
use crate::helper::hsv_to_rgb;

pub fn mandelbrot_animation(text: &str, duration: Duration, term_signal: &Arc<AtomicBool>) {
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

    let max_iter = 100;
    let zoom_speed = 0.2;
    let zoom_reset_threshold = 50.0;
    let mut time = 0.0;
    let center_real = -0.5;
    let center_imag = 0.0;

    while duration.as_millis() == 0 || start.elapsed() < duration {
        if term_signal.load(Ordering::Relaxed) {
            return;
        }

        // Update time and handle zoom reset
        time += 0.016; // Approximately 60fps
        let zoom = (0.05_f64 + time * zoom_speed).exp();
        if zoom > zoom_reset_threshold {
            time = 0.0;
        }
        
        for y in 1..=term.height {
            for x in 1..=term.width {
                if y == center_y && x >= text_start_x && x <= text_end_x {
                    continue;
                }

                // Map screen coordinates to complex plane using f64
                let real = (x as f64 - term.width as f64 / 2.0) * 4.0 / (term.width as f64 * zoom) + center_real;
                let imag = (y as f64 - term.height as f64 / 2.0) * 4.0 / (term.height as f64 * zoom) + center_imag;

                let mut z_real = 0.0;
                let mut z_imag = 0.0;
                let mut iter = 0;

                // Mandelbrot iteration
                while iter < max_iter && z_real * z_real + z_imag * z_imag < 4.0 {
                    let new_real = z_real * z_real - z_imag * z_imag + real;
                    let new_imag = 2.0 * z_real * z_imag + imag;
                    z_real = new_real;
                    z_imag = new_imag;
                    iter += 1;
                }

                // Color based on iteration count
                if iter == max_iter {
                    write!(
                        stdout,
                        "{}{}▓",
                        cursor::Goto(x, y),
                        color::Fg(color::Black)
                    ).unwrap();
                } else {
                    // Create smooth coloring
                    let hue = (iter as f64 / max_iter as f64 + time * 0.1) % 1.0;
                    let saturation = 0.8;
                    let value = if iter < max_iter { 1.0 } else { 0.0 };
                    
                    let (r, g, b) = hsv_to_rgb(hue, saturation, value);
                    
                    write!(
                        stdout,
                        "{}{}▓",
                        cursor::Goto(x, y),
                        color::Fg(color::Rgb(r, g, b))
                    ).unwrap();
                }
            }
        }

        // Draw text
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

pub fn mandelbrot_matrix_animation(text: &str, duration: Duration, term_signal: &Arc<AtomicBool>) {
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

    let max_iter = 100;
    let zoom_speed = 0.2;
    let mut time = 0.0;
    let center_real = -0.5;
    let center_imag = 0.0;
    let mut zooming_in = true;

    while duration.as_millis() == 0 || start.elapsed() < duration {
        if term_signal.load(Ordering::Relaxed) {
            return;
        }

        // Update time and handle zoom direction
        time += if zooming_in { 0.016 } else { -0.016 }; // Time flows forward or backward
        let zoom = (0.05_f64 + time * zoom_speed).exp();

        // Switch direction when reaching extremes
        if zooming_in && zoom > 2.5 {
            zooming_in = false;
        } else if !zooming_in && zoom < 0.2 {
            zooming_in = true;
        }

        // Rest of the code is identical to original mandelbrot_animation
        for y in 1..=term.height {
            for x in 1..=term.width {
                if y == center_y && x >= text_start_x && x <= text_end_x {
                    continue;
                }

                let real = (x as f64 - term.width as f64 / 2.0) * 4.0 / (term.width as f64 * zoom) + center_real;
                let imag = (y as f64 - term.height as f64 / 2.0) * 4.0 / (term.height as f64 * zoom) + center_imag;

                let mut z_real = 0.0;
                let mut z_imag = 0.0;
                let mut iter = 0;

                while iter < max_iter && z_real * z_real + z_imag * z_imag < 4.0 {
                    let new_real = z_real * z_real - z_imag * z_imag + real;
                    let new_imag = 2.0 * z_real * z_imag + imag;
                    z_real = new_real;
                    z_imag = new_imag;
                    iter += 1;
                }

                if iter == max_iter {
                    write!(
                        stdout,
                        "{}{}▓",
                        cursor::Goto(x, y),
                        color::Fg(color::Black)
                    ).unwrap();
                } else {
                    let hue = (iter as f64 / max_iter as f64 + time * 0.1) % 1.0;
                    let saturation = 0.8;
                    let value = if iter < max_iter { 1.0 } else { 0.0 };

                    let (r, g, b) = hsv_to_rgb(hue, saturation, value);

                    write!(
                        stdout,
                        "{}{}▓",
                        cursor::Goto(x, y),
                        color::Fg(color::Rgb(r, g, b))
                    ).unwrap();
                }
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

pub fn mandelbrot_fast_animation(text: &str, duration: Duration, term_signal: &Arc<AtomicBool>) {
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

    let max_iter = 100;
    let mut zoom: f32;
    let center_real = -0.5;
    let center_imag = 0.0;

    while duration.as_millis() == 0 || start.elapsed() < duration {
        if term_signal.load(Ordering::Relaxed) {
            return;
        }

        let time = start.elapsed().as_secs_f32();
        zoom = 1.0 + time.sin() * 0.5; // Zoom oscillates between 0.5 and 1.5

        for y in 1..=term.height {
            for x in 1..=term.width {
                if y == center_y && x >= text_start_x && x <= text_end_x {
                    continue;
                }

                // Map screen coordinates to complex plane
                let real = (x as f32 - term.width as f32 / 2.0) * 4.0 / (term.width as f32 * zoom) + center_real;
                let imag = (y as f32 - term.height as f32 / 2.0) * 4.0 / (term.height as f32 * zoom) + center_imag;

                let mut z_real = 0.0;
                let mut z_imag = 0.0;
                let mut iter = 0;

                // Mandelbrot iteration
                while iter < max_iter && z_real * z_real + z_imag * z_imag < 4.0 {
                    let new_real = z_real * z_real - z_imag * z_imag + real;
                    let new_imag = 2.0 * z_real * z_imag + imag;
                    z_real = new_real;
                    z_imag = new_imag;
                    iter += 1;
                }

                // Color based on iteration count
                if iter == max_iter {
                    write!(
                        stdout,
                        "{}{}▓",
                        cursor::Goto(x, y),
                        color::Fg(color::Black)
                    ).unwrap();
                } else {
                    // Create smooth coloring
                    let hue = (iter as f32 / max_iter as f32 + time * 0.1) % 1.0;
                    let saturation = 0.8;
                    let value = if iter < max_iter { 1.0 } else { 0.0 };

                    let (r, g, b) = hsv_to_rgb(hue, saturation, value);

                    write!(
                        stdout,
                        "{}{}▓",
                        cursor::Goto(x, y),
                        color::Fg(color::Rgb(r, g, b))
                    ).unwrap();
                }
            }
        }

        // Draw text
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