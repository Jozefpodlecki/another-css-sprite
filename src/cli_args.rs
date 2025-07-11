use clap::Parser;

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
}