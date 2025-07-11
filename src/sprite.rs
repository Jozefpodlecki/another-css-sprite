use image::{DynamicImage, GenericImage, ImageBuffer, RgbaImage};
use walkdir::WalkDir;
use std::fs;

pub struct Sprite {
    pub images: Vec<(String, DynamicImage)>,
}

impl Sprite {
    pub fn new(input_dir: &str) -> Self {
        let mut images = Vec::new();
       
        for entry in WalkDir::new(input_dir)
            .into_iter()
            .filter_map(Result::ok)
            .filter(|e| e.file_type().is_file())
        {
            let path = entry.path();
            let img = image::open(&path).expect("Failed to open image");
            let name = path.file_stem().unwrap().to_string_lossy().to_string();
            images.push((name, img));
        }

        Self { images }
    }

    pub fn generate_sprite_and_css(&self, output_image: &str, output_css: &str) {
        let (sprite, css) = self.build_sprite_vertical();
        sprite.save(output_image).expect("Failed to save sprite image");
        fs::write(output_css, css).expect("Failed to write CSS");
    }

    fn build_sprite_vertical(&self) -> (RgbaImage, String) {
        let max_width: u32 = self.images.iter().map(|(_, img)| img.width()).max().unwrap_or(0);
        let total_height: u32 = self.images.iter().map(|(_, img)| img.height()).sum();

        let mut sprite: RgbaImage = ImageBuffer::new(max_width, total_height);

        let mut y_offset = 0;
        let mut css = String::new();

        for (name, img) in &self.images {
            sprite.copy_from(img, 0, y_offset).expect("Copy failed");

            css.push_str(&format!(
                ".{} {{ background-position: 0px -{}px; width: {}px; height: {}px; }}\n",
                name,
                y_offset,
                img.width(),
                img.height()
            ));

            y_offset += img.height();
        }

        (sprite, css)
    }
}
