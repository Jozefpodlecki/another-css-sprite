use std::fs::{create_dir};
use std::path::Path;

use anothercssspritelib::*;
use clap::Parser;

pub fn main() {
     let args = CliArgs::parse();

    // let input_dir = r#"C:\repos\css-sprite\images"#;
    let sprite = Sprite::new(&args.input);
    sprite.generate_sprite_and_css(&args.output, &args.css);
}