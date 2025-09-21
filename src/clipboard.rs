use std::borrow::Cow;

use anyhow::{Context, Result};
use arboard::{Clipboard, ImageData};
use image::DynamicImage;

pub fn copy_image(image: &DynamicImage) -> Result<()> {
    let rgba = image.to_rgba8();
    let (width, height) = rgba.dimensions();
    let bytes = rgba.into_raw();

    let mut clipboard = Clipboard::new().context("failed to connect to system clipboard")?;
    clipboard
        .set_image(ImageData {
            width: width as usize,
            height: height as usize,
            bytes: Cow::Owned(bytes),
        })
        .context("failed to copy image into clipboard")?;

    Ok(())
}
