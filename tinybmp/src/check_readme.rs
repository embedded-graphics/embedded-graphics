//! This file includes the contents of the project's README.md so the code examples in it can be run
//! as Rust doc tests

macro_rules! doc {
    ($e:expr) => {
        #[doc = $e]
        extern {}
    };
}

doc!(include_str!("../README.md"));
