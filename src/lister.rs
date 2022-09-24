//! lister.rs

use crate::{config, error::Result};

pub fn execute() -> Result<bool> {
    let cases = config::enumerate_testcases();
    for (i, name) in cases {
        println!("{i}\t{name}");
    }
    Ok(true)
}
