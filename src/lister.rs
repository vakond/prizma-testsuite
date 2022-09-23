//! lister.rs

use crate::error::Result;

include!(concat!(env!("OUT_DIR"), "/testcases.txt"));

pub fn execute() -> Result<()> {
    println!("{TESTCASES}");
    Ok(())
}
