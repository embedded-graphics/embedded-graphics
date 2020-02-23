//! This file includes the contents of the project's README.md so the code examples in it can be run
//! as Rust doc tests. The tests are only included on `x86_64` because they can't be executed on
//! `no_std` targets.

macro_rules! doc {
    ($e:expr) => {
        #[doc = $e]
        extern {}
    };
}

#[cfg(target_arch = "x86_64")]
doc!(include_str!("../README.md"));
