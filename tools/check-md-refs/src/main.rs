//! Check a given file for broken Markdown link references.
//!
//! Usage:
//!
//! ```bash
//! check-md-refs ./README.md
//! ```

use pulldown_cmark::{BrokenLink, Options, Parser};

fn main() {
    let file = std::env::args().nth(1).unwrap();
    let readme = std::fs::read_to_string(&file).unwrap();

    let broken_link_callback = &mut |broken_link: BrokenLink| {
        panic!("Broken link in {}: {}", file, broken_link.reference);
    };

    let parser =
        Parser::new_with_broken_link_callback(&readme, Options::all(), Some(broken_link_callback));

    for _event in parser {}
}
