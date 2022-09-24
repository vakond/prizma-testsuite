//! runner.rs

use crate::{error::Result, testcases};

pub fn execute() -> Result<bool> {
    let mut ok = true;

    let tests = testcases::select();

    for test in tests {
        if !test.run().unwrap() {
            ok = false;
        }
    }

    Ok(ok)
}
