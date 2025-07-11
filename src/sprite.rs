use anyhow::*;
use clap::ValueEnum;
use image::{DynamicImage, GenericImage, ImageBuffer, RgbaImage};
use log::*;
use rect_packer::{Config, Packer};
use std::fs;
use walkdir::WalkDir;

/// Layout options for the sprite sheet.
#[derive(ValueEnum, Debug, Clone)]
pub enum Layout {
    Horizontal,
    Vertical,
    Packed
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
            Layout::Packed => self.build_packed()?
        };

        Ok((sprite, css))
    }

    fn build_vertical(&self) -> Result<(RgbaImage, String)> {
        
        let (max_width, total_height) = self.images.iter().fold((0, 0), |(w_max, h_sum), (_, img)| {
            (w_max.max(img.width()), h_sum + img.height())
        });

        if max_width == 0 || total_height == 0 {
            return Err(anyhow!("Invalid sprite dimensions: {max_width}x{total_height}"));
        }

        debug!("Sprite dimension {}:{}", max_width, total_height);

        let mut sprite: RgbaImage = ImageBuffer::new(max_width, total_height);
        let mut css = String::new();

        let mut y_offset = 0;
        for (name, img) in &self.images {
            sprite.copy_from(img, 0, y_offset).context("Copy failed")?;
            let css_rule = &format!(
                ".{} {{ background-position: 0px -{}px; width: {}px; height: {}px; }}\n",
                name,
                y_offset,
                img.width(),
                img.height()
            );
            css.push_str(css_rule);

            y_offset += img.height();
        }

        Ok((sprite, css))
    }

    fn estimate_bin_size(&self) -> (u32, u32) {
        let (total_area, max_width) = self.images.iter().fold(
            (0u32, 0u32),
            |(area_acc, w_max), (_, img)| {
                (area_acc + img.width() * img.height(), w_max.max(img.width()))
            },
        );

        let mut bin_width = 256;
        while bin_width < max_width || bin_width * bin_width < total_area {
            bin_width *= 2;
        }

        let mut bin_height = total_area / bin_width;
        if total_area % bin_width != 0 {
            bin_height += 1;
        }

        (bin_width, bin_height)
    }

    pub fn build_packed(&self) -> Result<(RgbaImage, String)> {
        
        let (width, height) = self.estimate_bin_size();
        // let (total_area, max_width, max_height) = self.images.iter().fold(
        //     (0u32, 0u32, 0u32),
        //     |(area_acc, w_max, h_max), (_, img)| {
        //         let w = img.width();
        //         let h = img.height();
        //         (
        //             area_acc + w * h,
        //             w_max.max(w),
        //             h_max.max(h),
        //         )
        //     },
        // );

        // let mut atlas_width = (total_area as f64).sqrt().ceil() as i32;
        // if atlas_width < max_width as i32 {
        //     atlas_width = max_width as i32;
        // }
        // let estimated_height = (total_area as f64 / atlas_width as f64).ceil() as i32;

        info!("Sprite dimension {}:{}", width, height);

        let config = Config {
            height: height as i32,
            width: width as i32,
            border_padding: 0,
            rectangle_padding: 0
        };
        let mut packer = Packer::new(config);
        let mut placements = Vec::new();

        for (name, img) in &self.images {
            let w = img.width() as i32;
            let h = img.height() as i32;

            let rect = packer.pack(w, h, false)
                .ok_or_else(|| anyhow!("Failed to pack image '{}'", name))?;

            placements.push((name, rect, img));
        }

        let mut sprite = ImageBuffer::new(width as u32, height as u32);
        let mut css = String::new();

        for (name, rect, img) in placements {
            sprite.copy_from(img, rect.x as u32, rect.y as u32)?;
            let css_rule = &format!(
                ".{} {{ background-position: -{}px -{}px; width: {}px; height: {}px; }}\n",
                name,
                rect.x,
                rect.y,
                img.width(),
                img.height()
            );
            css.push_str(css_rule);
        }

        Ok((sprite, css))
    }

    fn build_horizontal(&self) -> Result<(RgbaImage, String)> {
        let (total_width, max_height) = self.images.iter().fold((0, 0), |(w_sum, h_max), (_, img)| {
            (w_sum + img.width(), h_max.max(img.height()))
        });

        if total_width == 0 || max_height == 0 {
            return Err(anyhow!("Invalid sprite dimensions: {total_width}x{max_height}"));
        }

        debug!("Sprite dimension {}:{}", total_width, max_height);

        let mut sprite: RgbaImage = ImageBuffer::new(total_width, max_height);
        let mut css = String::new();

        let mut x_offset = 0;
        for (name, img) in &self.images {
            sprite.copy_from(img, x_offset, 0).context("Copy failed")?;
            let css_rule = &format!(
                ".{} {{ background-position: -{}px 0px; width: {}px; height: {}px; }}\n",
                name,
                x_offset,
                img.width(),
                img.height()
            );
            css.push_str(css_rule);

            x_offset += img.width();
        }

        Ok((sprite, css))
    }
}
