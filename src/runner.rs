//! runner.rs

use crate::{error::Result, testcases, testcases::Test};

pub fn execute() -> Result<bool> {
    let mut ok = true;
    let tests = testcases::collect();
    for test in tests {
        if !test.run().unwrap() {
            ok = false;
        }
    }
    Ok(ok)
}
