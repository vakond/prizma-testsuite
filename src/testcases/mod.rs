//! testcases/mod.rs

/// Unified interface of all test cases.
pub trait Test {
    fn name(&self) -> String;
    fn setup(&self);
    fn run(&self) -> crate::error::Result<bool>;
    fn teardown(&self);
}

//include!(concat!(env!("OUT_DIR"), "/select_function.rs"));

use std::collections::HashSet;
pub fn select(selected: HashSet<String>, excluded: HashSet<String>) -> Vec<(usize, Box<dyn Test>)> {
    let s = !selected.is_empty();
    let x = !excluded.is_empty();
    assert!(!(s && x), "mutually exclusive options");

    let mut tests = Vec::new();

    if (!s && !x)
        || (s && (selected.contains("1") || selected.contains("alpha")))
        || (x && !(excluded.contains("1") || excluded.contains("alpha")))
    {
        tests.push((1, alpha::get()));
    }

    if (!s && !x)
        || (s && (selected.contains("2") || selected.contains("beta")))
        || (x && !(excluded.contains("2") || excluded.contains("beta")))
    {
        tests.push((2, beta::get()));
    }

    if (!s && !x)
        || (s && (selected.contains("3") || selected.contains("gamma")))
        || (x && !(excluded.contains("3") || excluded.contains("gamma")))
    {
        tests.push((3, gamma::get()));
    }

    tests
}

mod alpha;
mod beta;
mod gamma;
