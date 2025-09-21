use std::path::PathBuf;

use clap::{ArgAction, Parser, ValueEnum};
use image::ImageFormat;

#[derive(Parser, Debug)]
#[command(
    name = "milkshake",
    version,
    about = "Generate images with Google's Nano Banana model from your terminal."
)]
pub struct Cli {
    /// Text prompt to send to the Nano Banana model. If omitted, the prompt is read from STDIN.
    #[arg(value_name = "PROMPT")]
    pub prompt: Vec<String>,

    /// Model identifier to call. Defaults to gemini-2.5-flash-image-preview.
    #[arg(
        short,
        long,
        env = "MILKSHAKE_MODEL",
        default_value = "gemini-2.5-flash-image-preview"
    )]
    pub model: String,

    /// Google AI for Developers API key. Falls back to the GOOGLE_API_KEY env variable.
    #[arg(long, env = "GOOGLE_API_KEY", hide_env_values = true)]
    pub api_key: Option<String>,

    /// Output path for the generated image. Defaults to a Pictures/milkshake directory if available.
    #[arg(short, long, value_name = "PATH")]
    pub output: Option<PathBuf>,

    /// Disable copying the generated image to the clipboard.
    #[arg(long, action = ArgAction::SetTrue)]
    pub no_copy: bool,

    /// Request timeout in seconds.
    #[arg(long, default_value_t = 60)]
    pub timeout: u64,

    /// Suppress informational output (errors are still printed).
    #[arg(short, long, action = ArgAction::SetTrue)]
    pub quiet: bool,

    /// Output image encoding.
    #[arg(long, value_enum, default_value_t = OutputFormat::Png)]
    pub format: OutputFormat,
}

#[derive(Copy, Clone, Debug, ValueEnum)]
pub enum OutputFormat {
    Png,
}

impl OutputFormat {
    pub fn mime_type(&self) -> &'static str {
        match self {
            OutputFormat::Png => "image/png",
        }
    }

    pub fn extension(&self) -> &'static str {
        match self {
            OutputFormat::Png => "png",
        }
    }

    pub fn image_format(&self) -> ImageFormat {
        match self {
            OutputFormat::Png => ImageFormat::Png,
        }
    }
}
