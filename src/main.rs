use clap::Parser;
use rand::Rng;
use std::{
    io::{stdout, Write},
    thread::sleep,
    time::{Duration, Instant},
};
use termion::{
    clear, color,
    cursor::{self},
    raw::IntoRawMode,
    terminal_size,
    event::Key,
    input::TermRead,
};
use std::sync::atomic::{AtomicBool, Ordering};
use std::sync::Arc;

#[derive(Parser, Debug)]
#[command(author, version, about, long_about = None)]
struct Args {
    #[arg(long, value_enum)]
    style: AnimationStyle,
    #[arg(short, long)]
    text: String,
    #[arg(short, long, default_value = "5000", help = "Animation duration in milliseconds (infinite if 0)")]
    duration: u64,
}

#[derive(clap::ValueEnum, Clone, Debug)]
enum AnimationStyle {
    Rainbow,
    Explosion,
    Waves,
}

struct Terminal {
    width: u16,
    height: u16,
}

impl Terminal {
    fn new() -> Self {
        let (w, h) = terminal_size().unwrap();
        Self {
            width: w,
            height: h,
        }
    }

    fn center_pos(&self) -> (u16, u16) {
        (self.width / 2, self.height / 2)
    }
}

#[derive(Clone)]
struct Cell {
    char: char,
    color: color::Rgb,
}

fn rainbow_animation(text: &str, duration: Duration, term_signal: &Arc<AtomicBool>) {
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

fn explosion_animation(text: &str, duration: Duration, term_signal: &Arc<AtomicBool>) {
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

fn waves_animation(text: &str, duration: Duration, term_signal: &Arc<AtomicBool>) {
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
    }

    // Cleanup remains the same
    print!("{}{}{}", 
        termion::screen::ToMainScreen,
        termion::cursor::Show,
        clear::All
    );
} 