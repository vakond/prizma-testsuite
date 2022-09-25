//! testcases/mod.rs

include!(concat!(env!("OUT_DIR"), "/verify_function.rs"));
include!(concat!(env!("OUT_DIR"), "/check_function.rs"));
include!(concat!(env!("OUT_DIR"), "/select_function.rs"));

/// Unified interface of all test cases.
pub trait Test {
    fn name(&self) -> &'static str;
    fn setup(&self);
    fn run(&self) -> crate::error::Result<bool>;
    fn teardown(&self);
}

mod alpha;
mod beta;
mod gamma;
