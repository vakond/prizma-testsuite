//! beta.rs

use super::Test;
use crate::error::Result;

pub fn get() -> Box<dyn Test> {
    Box::new(Object {})
}

struct Object;

impl Test for Object {
    fn setup(&self) {}

    fn run(&self) -> Result<bool> {
        println!("alpha");
        Ok(true)
    }

    fn teardown(&self) {}
}
