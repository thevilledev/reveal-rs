use clap::Parser;

#[derive(Parser, Debug)]
#[command(author, version, about, long_about = None)]
pub struct Args {
    #[arg(long, value_enum)]
    pub style: AnimationStyle,
    #[arg(short, long, default_value = "")]
    pub text: String,
    #[arg(short, long, default_value = "5000", help = "Animation duration in milliseconds (infinite if 0)")]
    pub duration: u64,
}

#[derive(clap::ValueEnum, Clone, Debug)]
pub enum AnimationStyle {
    Rainbow,
    Explosion,
    Waves,
    WavesGradient,
    Mandelbrot,
    MandelbrotMatrix,
    MandelbrotFast,
}
