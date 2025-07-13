mod utils;

use std::{borrow::Cow, fs::File, io::Write};

use lightningcss::{printer::PrinterOptions, properties::{background::BackgroundPosition, Property}, rules::CssRule, stylesheet::{ParserOptions, StyleSheet}, traits::ToCss, values::image::Image};

fn generate_html_preview(css_path: &str, image_path: &str, output_path: &str) -> anyhow::Result<()> {
    let css_content  = std::fs::read_to_string(css_path)?;
    let stylesheet = StyleSheet::parse(&css_content, ParserOptions::default()).unwrap();

    let mut html = String::new();
    html.push_str(r#"<!DOCTYPE html><html><head><meta charset="utf-8"><title>Preview</title>"#);
    html.push_str(&format!(r#"<link rel="stylesheet" href="{}">"#, css_path));
    html.push_str(r#"<style>"#);
    html.push_str(r#"
        body {
            background-color: black;
            margin: 0;
            padding: 1rem;
        }
        .container {
            display: flex;
            flex-wrap: wrap;
            gap: 1rem;
            align-items: flex-start;
        }
    "#);
    html.push_str(r#"</style></head><body>"#);
    html.push_str(r#"<div class="container">"#);

    for rule in &stylesheet.rules.0 {
        if let CssRule::Style(style_rule) = rule {
            if let Some(first_selector) = style_rule.selectors.0.get(0) {
                let class_name = first_selector
                    .to_css_string(PrinterOptions::default())?;
                let class_name = class_name.trim_start_matches('.');
                let node_html = &format!(r#"<div class="{}"></div>"#, class_name.trim());
                html.push_str(node_html);
            }
        }
    }

    html.push_str(r#"</div></body></html>"#);

    let mut file = File::create(output_path)?;
    file.write_all(html.as_bytes())?;

    println!("Generated HTML preview: {}", output_path);

    Ok(())
}

fn main() -> anyhow::Result<()> {
    let css_rules_path = r#"C:\repos\another-css-sprite\sprite.css"#;
    let image_path = r#"C:\repos\another-css-sprite\test.webp"#;
    let output_html = r#"C:\repos\another-css-sprite\preview.html"#;

    generate_html_preview(css_rules_path, image_path, output_html)?;

    Ok(())
}