use regex::Regex;
use std::io::prelude::*;
use std::path::Path;
use unindent::unindent;

pub type Display = png_target::PngTarget<embedded_graphics::pixelcolor::Rgb888>;

pub struct ExampleWriter {
    header_lines: Vec<String>,
    examples: Vec<Example>,
}

impl ExampleWriter {
    pub fn new(input: &str) -> Self {
        // Get module doc comments as the markdown file header.
        let header_lines: Vec<_> = input
            .lines()
            .take_while(|line| line.starts_with("//!"))
            .map(|line| line.trim_start_matches("//!").trim().to_string())
            .collect();

        // Find all functions that start with `draw_`.
        let re = Regex::new("(?s)((?:///.*?\n)*)fn (draw_\\w*)[^\n]*(.*?\n)}").unwrap();

        let mut examples = Vec::new();

        for captures in re.captures_iter(input) {
            let comments = &captures[1];
            let name = &captures[2];
            let code = unindent(&captures[3]);

            // Add comments without `///`.
            let docs = comments
                .lines()
                .map(|line| line.trim_start_matches('/').trim().to_string())
                .collect();

            // Remove `Ok(display)` and trailing empty lines from code.
            let mut code: Vec<_> = code.lines().map(|line| line.to_string()).collect();
            assert_eq!(
                code.pop().as_ref().map(|s| s.trim()),
                Some("Ok(display)"),
                "All examples must end with `Ok(display)`"
            );
            while code.last().map(|s| s.trim().is_empty()).unwrap_or_default() {
                code.pop();
            }

            examples.push(Example {
                name: name.to_string(),
                docs,
                code,
                image: String::new(),
            });
        }

        Self {
            header_lines,
            examples,
        }
    }

    pub fn set_image<F>(&mut self, name: &str, f: F)
    where
        F: Fn(Display) -> Result<Display, std::convert::Infallible>,
    {
        let display = Display::new(embedded_graphics::geometry::Size::new_equal(64), 2);
        let display = f(display).unwrap();

        let example = self.examples.iter_mut().find(|e| e.name == name).unwrap();
        example.image = display.to_base64();
    }

    pub fn write<P: AsRef<Path>>(&self, path: P) -> std::io::Result<()> {
        let mut file = std::fs::File::create(path)?;

        for line in &self.header_lines {
            writeln!(file, "//! {}", line)?;
        }
        writeln!(file, "//!")?;

        for example in &self.examples {
            let mut doc_lines = example.docs.iter();
            writeln!(file, "//! {}", doc_lines.next().unwrap())?;
            writeln!(file, "//!")?;
            writeln!(
                file,
                r#"//! <img src="data:image/png;base64,{}" style="float: right; padding-left: 1rem; padding-bottom: 1rem;">"#,
                example.image
            )?;
            for line in doc_lines {
                writeln!(file, "//! {}", line)?;
            }
            writeln!(file, "//!")?;
            writeln!(file, r#"//! <div style="clear: both;"></div>"#)?;
            writeln!(file, "//!")?;

            writeln!(file, "//! ```ignore")?;
            for line in &example.code {
                writeln!(file, "//! {}", line)?;
            }
            writeln!(file, "//! ```")?;
        }

        Ok(())
    }
}

pub struct Example {
    name: String,
    docs: Vec<String>,
    code: Vec<String>,
    image: String,
}

#[macro_export]
macro_rules! example {
    ($writer:expr, $name:ident) => {
        $writer.set_image(stringify!($name), $name)
    };
}
