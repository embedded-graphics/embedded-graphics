use regex::Regex;
use std::path::Path;
use unindent::unindent;

pub type Display = png_target::PngTarget<embedded_graphics::pixelcolor::Rgb888>;

#[macro_export]
macro_rules! example {
    ($name:ident) => {
        let display =
            png_target::PngTarget::new(embedded_graphics::geometry::Size::new_equal(64), 2);

        let display = $name(display).unwrap();

        let path = std::path::Path::new(env!("CARGO_MANIFEST_DIR"))
            .join("../../doc")
            .join("assets")
            .join(stringify!($name))
            .with_extension("png");
        display.save(&path).unwrap();
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

    std::fs::write(
        Path::new(env!("CARGO_MANIFEST_DIR")).join("../../doc/drawing-examples.md"),
        output.join("\n"),
    )
    .unwrap();
}
