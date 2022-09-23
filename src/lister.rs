//! lister.rs

use crate::error::Result;

include!(concat!(env!("OUT_DIR"), "/testcases.txt"));

pub fn execute() -> Result<bool> {
    println!("{TESTCASES}");
    Ok(true)
}
