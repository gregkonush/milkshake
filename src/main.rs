mod cli;
mod clipboard;
mod google;
mod output;

use std::env;
use std::io::{self, IsTerminal, Read};
use std::time::Duration;

use anyhow::{Context, Result, anyhow};
use clap::Parser;
use image::DynamicImage;

use crate::cli::Cli;
use crate::google::{GenerateImageRequest, GoogleImageClient};

fn main() {
    if let Err(err) = run() {
        eprintln!("Error: {err}");
        std::process::exit(1);
    }
}

fn run() -> Result<()> {
    let cli = Cli::parse();

    let prompt = resolve_prompt(&cli)?;

    let api_key = cli
        .api_key
        .or_else(|| env::var("GOOGLE_API_KEY").ok())
        .ok_or_else(|| {
            anyhow!("missing Google AI API key; set GOOGLE_API_KEY or pass --api-key")
        })?;

    let client = GoogleImageClient::new(api_key, Duration::from_secs(cli.timeout))?;
    let request = GenerateImageRequest {
        prompt,
        model: cli.model.clone(),
    };

    let generated = client.generate_image(&request)?;

    if generated.mime_type != cli.format.mime_type() && !cli.quiet {
        eprintln!(
            "Warning: expected {}, but model returned {}",
            cli.format.mime_type(),
            generated.mime_type
        );
    }

    let dyn_image = decode_image(&generated.data)?;

    let output_path = output::resolve_output_path(cli.output.clone(), cli.format)?;
    dyn_image
        .save_with_format(&output_path, cli.format.image_format())
        .with_context(|| format!("failed to write image to {}", output_path.display()))?;

    let mut clipboard_status = None;
    if !cli.no_copy {
        match clipboard::copy_image(&dyn_image) {
            Ok(_) => {
                clipboard_status = Some("copied to clipboard");
            }
            Err(err) => {
                clipboard_status = Some("clipboard unavailable");
                if !cli.quiet {
                    eprintln!("Warning: failed to copy image to clipboard: {err}");
                }
            }
        }
    }

    if !cli.quiet {
        println!("Saved image to {}", output_path.display());
        if let Some(status) = clipboard_status {
            println!("Clipboard status: {status}");
        }
        if let Some(ratings) = generated.safety_ratings.as_ref() {
            for rating in ratings {
                if rating.blocked.unwrap_or(false) {
                    println!(
                        "Safety rating: {} flagged (probability {}).",
                        rating.category, rating.probability
                    );
                }
            }
        }
        if let Some(feedback) = generated.prompt_feedback.as_ref() {
            if let Some(ratings) = feedback.safety_ratings.as_ref() {
                for rating in ratings {
                    if rating.blocked.unwrap_or(false) {
                        println!(
                            "Prompt feedback: {} blocked (probability {}).",
                            rating.category, rating.probability
                        );
                    }
                }
            }
        }
    }

    Ok(())
}

fn read_prompt_from_stdin() -> Result<String> {
    let mut buffer = String::new();
    io::stdin()
        .read_to_string(&mut buffer)
        .context("failed to read prompt from STDIN")?;
    let trimmed = buffer.trim();
    if trimmed.is_empty() {
        return Err(anyhow!("prompt from STDIN was empty"));
    }
    Ok(trimmed.to_string())
}

fn resolve_prompt(cli: &Cli) -> Result<String> {
    if !cli.prompt.is_empty() {
        return Ok(cli.prompt.join(" "));
    }

    if io::stdin().is_terminal() {
        return Err(anyhow!("no prompt provided; pass text or pipe it on STDIN"));
    }

    read_prompt_from_stdin()
}

fn decode_image(data: &[u8]) -> Result<DynamicImage> {
    image::load_from_memory(data).context("Gemini API returned unsupported image data")
}
