//! This file includes the contents of the project's README.md so the code examples in it can be run
//! as Rust doc tests

macro_rules! doc {
    ($e:expr) => {
        #[doc = $e]
        extern {}
    };
}

doc!(include_str!("../README.md"));

// The examples in README.md inside the embedded-graphics crate depend on the
// simulator. These examples are tested here because it's currently impossible
// to add a dev-dependency to the simulator without breaking no_std builds.
//
// This test can be moved back into the embedded-graphics crate when
// https://doc.rust-lang.org/nightly/cargo/reference/unstable.html#features
// gets stabilized.
//
// NOTE: Commented out as including this line prevents a publish due to the e-g readme not being
// present in the crate path.
// doc!(include_str!("../../embedded-graphics/README.md"));
