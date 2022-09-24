//! testcases/mod.rs

/// Unified interface of all test cases.
pub trait Test {
    fn setup(&self);
    fn run(&self) -> crate::error::Result<bool>;
    fn teardown(&self);
}

mod alpha;
mod beta;
mod gamma;

pub fn select() -> Vec<Box<dyn Test>> {
    let mut tests = Vec::new();
    if true {
        tests.push(alpha::get());
    }
    if true {
        tests.push(beta::get());
    }
    if true {
        tests.push(gamma::get());
    }
    tests
}

//include!(concat!(env!("OUT_DIR"), "/select_function.rs"));
