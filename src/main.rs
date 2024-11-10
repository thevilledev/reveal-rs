use std::{
    io::stdout,
    time::Duration,
};
use termion::{
    clear,
    raw::IntoRawMode,
    event::Key,
    input::TermRead,
};
use std::sync::atomic::{AtomicBool, Ordering};
use std::sync::Arc;

use clap::Parser;

use reveal::cli::{Args, AnimationStyle};
use reveal::rainbow::rainbow_animation;
use reveal::explosion::explosion_animation;
use reveal::waves::waves_animation;
use reveal::waves::waves_gradient_animation;
use reveal::mandelbrot::mandelbrot_animation;
use reveal::mandelbrot::mandelbrot_matrix;
fn main() {
    let args = Args::parse();
    let duration = Duration::from_millis(args.duration);
    
    let term = Arc::new(AtomicBool::new(false));
    let term_clone = Arc::clone(&term);

    let _raw = stdout().into_raw_mode().unwrap();
    let stdin = std::io::stdin();

    // Spawn input handling thread
    std::thread::spawn(move || {
        for c in stdin.keys() {
            if let Ok(Key::Ctrl('c')) = c {
                term_clone.store(true, Ordering::SeqCst);
                break;
            }
        }
    });

    print!("{}{}", termion::cursor::Hide, clear::All);
    
    match args.style {
        AnimationStyle::Rainbow => rainbow_animation(&args.text, duration, &term),
        AnimationStyle::Explosion => explosion_animation(&args.text, duration, &term),
        AnimationStyle::Waves => waves_animation(&args.text, duration, &term),
        AnimationStyle::WavesGradient => waves_gradient_animation(&args.text, duration, &term),
        AnimationStyle::Mandelbrot => mandelbrot_animation(&args.text, duration, &term),
        AnimationStyle::MandelbrotMatrix => mandelbrot_matrix(&args.text, duration, &term),
    }

    // Cleanup remains the same
    print!("{}{}{}", 
        termion::screen::ToMainScreen,
        termion::cursor::Show,
        clear::All
    );
} 