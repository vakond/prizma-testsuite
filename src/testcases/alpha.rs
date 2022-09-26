//! testcases/alpha.rs

use super::Test;
use crate::error::Result;
use std::{thread, time::Duration};

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
        use rand::Rng as _;
        println!("enter alpha");
        let mut rng = rand::thread_rng();
        let delay = Duration::from_millis(rng.gen_range(1..=1000));
        thread::sleep(delay);
        println!("leave alpha");
        Ok(true)
    }

    fn teardown(&self) {}
}
