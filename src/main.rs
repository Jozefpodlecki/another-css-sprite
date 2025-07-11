use std::fs::{create_dir};
use anyhow::*;

use anothercssspritelib::utils::rename_files_with_prefix;
use anothercssspritelib::*;
use clap::Parser;
use simple_logger::SimpleLogger;

pub fn main() -> Result<()> {
    let args = CliArgs::parse();
    let log_level = match args.verbose {
        0 => log::LevelFilter::Warn,
        1 => log::LevelFilter::Info,
        2 => log::LevelFilter::Debug,
        _ => log::LevelFilter::Trace,
    };

    SimpleLogger::new().env()
        .with_level(log_level)
        .init().unwrap();
   

    let sprite = Sprite::new(&args.input);
    sprite.generate_sprite_and_css(&args.output, &args.css, args.layout)?;

    Ok(())
}