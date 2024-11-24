use bevy::prelude::Resource;
use clap::Parser;

#[derive(Parser, Resource)]
#[command(version, about, long_about = None)]
pub struct Cli {
    /// Display text
    #[arg(short, long, default_value = "Hello World!")]
    pub text: String,

    /// Number of frames
    #[arg(short, long, default_value_t = 500)]
    pub frames: u32,

    /// Output path
    #[arg(short, long, default_value = "generated_text.mp4")]
    pub output: String,
}
