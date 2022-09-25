//! testcases/mod.rs

/// Unified interface of all test cases.
pub trait Test {
    fn name(&self) -> &'static str;
    fn setup(&self);
    fn run(&self) -> crate::error::Result<bool>;
    fn teardown(&self);
}

include!(concat!(env!("OUT_DIR"), "/select_function.rs"));

mod alpha;
mod beta;
mod gamma;
