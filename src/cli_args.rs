use clap::Parser;

use crate::sprite::Layout;

#[derive(Parser)]
#[command(author, version, about)]
pub struct CliArgs {
    /// Input directory containing images
    #[arg(short, long)]
    pub input: String,

    /// Output sprite image file (e.g., sprite.png)
    #[arg(short, long, default_value_t = String::from("sprite.png"))]
    pub output: String,

    /// Output CSS file (e.g., sprite.css)
    #[arg(short, long, default_value_t = String::from("sprite.css"))]
    pub css: String,

    /// Layout direction: horizontal or vertical
    #[arg(short, long, value_enum, default_value_t = Layout::Vertical)]
    pub layout: Layout,

    /// Minify the generated CSS output
    #[arg(long, default_value_t = true)]
    pub minify_css: bool,

    /// Verbosity (-v, -vv, -vvv)
    #[arg(short, long, action = clap::ArgAction::Count)]
    pub verbose: u8,
}