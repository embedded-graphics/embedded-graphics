use embedded_graphics::pixelcolor::Rgb888;
use embedded_graphics_simulator::{OutputSettingsBuilder, SimulatorDisplay};
use regex::Regex;
use std::path::{Path, PathBuf};
use unindent::unindent;

pub type Display = SimulatorDisplay<Rgb888>;

fn doc_path() -> PathBuf {
    Path::new(env!("CARGO_MANIFEST_DIR")).join("../../doc")
}

pub fn write_png(name: &str, display: Display) {
    let output_settings = OutputSettingsBuilder::new().scale(2).build();

    let path = doc_path().join("assets").join(name).with_extension("png");

    display
        .to_image_buffer(&output_settings)
        .save(&path)
        .unwrap();
}

#[macro_export]
macro_rules! example {
    ($name:ident) => {
        write_png(
            stringify!($name),
            $name(crate::Display::new(
                embedded_graphics::geometry::Size::new_equal(64),
            ))
            .unwrap(),
        );
    };
}

pub fn generate_markdown(input: &str) {
    // Get module doc comments as the markdown file header.
    let mut output: Vec<_> = input
        .lines()
        .take_while(|line| line.starts_with("//!"))
        .map(|line| line.trim_start_matches("//!").trim().to_string())
        .collect();
    output.push(String::new());

    // Find all functions that start with `draw_`.
    let re = Regex::new("(?s)((?:///.*?\n)*)fn (draw_\\w*)[^\n]*(.*?\n)}").unwrap();

    for captures in re.captures_iter(input) {
        let comments = &captures[1];
        let name = &captures[2];
        let code = unindent(&captures[3]);

        // Add comments without `///`.
        for line in comments
            .lines()
            .map(|line| line.trim_start_matches('/').trim())
        {
            output.push(line.to_string());
        }
        output.push(String::new());

        // Add image.
        output.push(format!(
            "<img align=\"left\" alt=\"{0} example screenshot\" src=\"assets/{0}.png\">",
            name,
        ));
        output.push(String::new());

        // Remove `Ok(display)` and trailing empty lines from code.
        let mut lines: Vec<_> = code.lines().map(|line| line.to_string()).collect();
        assert_eq!(
            lines.pop().as_ref().map(|s| s.trim()),
            Some("Ok(display)"),
            "All examples must end with `Ok(display)`"
        );
        while lines
            .last()
            .map(|s| s.trim().is_empty())
            .unwrap_or_default()
        {
            lines.pop();
        }

        // Add code to the output.
        output.push("```rust".to_string());
        output.extend(lines);
        output.push("```".to_string());
        output.push(String::new());
    }

    std::fs::write(doc_path().join("drawing-examples.md"), output.join("\n")).unwrap();
}
