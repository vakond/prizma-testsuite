//! testcases/mod.rs

mod alpha;
mod beta;
mod gamma;

use crate::error::Result;

pub trait Test {
    fn run(&self) -> Result<bool>;
}

pub fn collect() -> Vec<impl Test> {
    let mut tests = Vec::new();
    if true {
        tests.push(alpha::get());
    }
    tests
}
