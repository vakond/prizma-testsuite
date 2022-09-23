//! alpha.rs

use super::Test;
use crate::error::Result;

pub fn get() -> impl Test {
    Alpha
}

struct Alpha;

impl Test for Alpha {
    fn run(&self) -> Result<bool> {
        Ok(true)
    }
}
