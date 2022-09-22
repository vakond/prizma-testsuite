//! runner.rs

use crate::{error::Result, testcases};

pub fn execute() -> Result<()> {
    testcases::collect();
    Ok(())
}
