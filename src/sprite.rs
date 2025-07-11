use anyhow::*;
use clap::ValueEnum;
use image::{DynamicImage, GenericImage, ImageBuffer, RgbaImage};
use log::*;
use std::fs;
use walkdir::WalkDir;

/// Layout options for the sprite sheet.
#[derive(ValueEnum, Debug, Clone)]
pub enum Layout {
    Horizontal,
    Vertical,
}

pub struct Sprite {
    pub images: Vec<(String, DynamicImage)>,
}

impl Sprite {
    pub fn new(input_dir: &str) -> Self {
        let mut images = Vec::new();

        debug!("Scanning directory: {input_dir:?}");

        for entry in WalkDir::new(input_dir)
            .into_iter()
            .filter_map(Result::ok)
            .filter(|e| e.file_type().is_file())
        {
            let path = entry.path();

            debug!("Found: {path:?}");

            let img = match image::open(&path) {
                std::result::Result::Ok(i) => i,
                Err(e) => {
                    warn!("Failed to open image {:?}: {}", path, e);
                    continue;
                }
            };

            let name = path
                .file_stem()
                .unwrap_or_default()
                .to_string_lossy()
                .to_string();
            images.push((name, img));
        }

        debug!("Total images in {input_dir}: {}", images.len());

        Self { images }
    }

    pub fn generate_sprite_and_css(
        &self,
        output_image: &str,
        output_css: &str,
        layout: Layout,
    ) -> Result<()> {
        let (sprite, css) = self.build_sprite(layout)?;
        sprite.save(output_image).context("Failed to save sprite image")?;
        fs::write(output_css, css).context("Failed to write CSS")?;
        Ok(())
    }

    fn build_sprite(&self, layout: Layout) -> Result<(RgbaImage, String)> {
        if self.images.is_empty() {
            return Err(anyhow!("No images found in input directory"));
        }

        let (sprite, css) = match layout {
            Layout::Vertical => self.build_vertical()?,
            Layout::Horizontal => self.build_horizontal()?,
        };

        Ok((sprite, css))
    }

    fn build_vertical(&self) -> Result<(RgbaImage, String)> {
        let max_width = self.images.iter().map(|(_, img)| img.width()).max().unwrap_or(0);
        let total_height = self.images.iter().map(|(_, img)| img.height()).sum();

        if max_width == 0 || total_height == 0 {
            return Err(anyhow!("Invalid sprite dimensions: {max_width}x{total_height}"));
        }

        let mut sprite: RgbaImage = ImageBuffer::new(max_width, total_height);
        let mut css = String::new();

        let mut y_offset = 0;
        for (name, img) in &self.images {
            sprite.copy_from(img, 0, y_offset).context("Copy failed")?;

            css.push_str(&format!(
                ".{} {{ background-position: 0px -{}px; width: {}px; height: {}px; }}\n",
                name,
                y_offset,
                img.width(),
                img.height()
            ));

            y_offset += img.height();
        }

        Ok((sprite, css))
    }

    fn build_horizontal(&self) -> Result<(RgbaImage, String)> {
        let total_width = self.images.iter().map(|(_, img)| img.width()).sum();
        let max_height = self.images.iter().map(|(_, img)| img.height()).max().unwrap_or(0);

        if total_width == 0 || max_height == 0 {
            return Err(anyhow!("Invalid sprite dimensions: {total_width}x{max_height}"));
        }

        let mut sprite: RgbaImage = ImageBuffer::new(total_width, max_height);
        let mut css = String::new();

        let mut x_offset = 0;
        for (name, img) in &self.images {
            sprite.copy_from(img, x_offset, 0).context("Copy failed")?;

            css.push_str(&format!(
                ".{} {{ background-position: -{}px 0px; width: {}px; height: {}px; }}\n",
                name,
                x_offset,
                img.width(),
                img.height()
            ));

            x_offset += img.width();
        }

        Ok((sprite, css))
    }
}
