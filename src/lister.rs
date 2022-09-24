//! lister.rs

use crate::error::Result;

include!(concat!(env!("OUT_DIR"), "/constant_list_of_testcases.rs"));

pub fn execute() -> Result<bool> {
    println!("{TESTCASES}");
    Ok(true)
}
