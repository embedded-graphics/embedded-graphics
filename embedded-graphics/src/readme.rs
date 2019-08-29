macro_rules! doc {
    ($e:expr) => {
        #[doc = $e]
        extern {}
    };
}

doc!(include_str!("../../README.md"));
