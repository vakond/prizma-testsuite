//! testcases/alpha.rs

use super::Test;
use crate::error::Result;

pub fn get() -> Box<dyn Test + Send> {
    Box::new(Object {})
}

struct Object;

impl Test for Object {
    fn name(&self) -> &'static str {
        "alpha"
    }

    fn setup(&self) {}

    fn run(&self) -> Result<bool> {
        println!("alpha");
        Ok(true)
    }

    fn teardown(&self) {}
}
