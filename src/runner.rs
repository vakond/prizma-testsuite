//! runner.rs

use crate::{
    error::{Error, Result},
    testcases,
};
use std::collections::HashSet;

pub fn execute(select: String, exclude: String) -> Result<bool> {
    if !select.is_empty() && !exclude.is_empty() {
        return Err(Error::MutuallyExclusive);
    }

    let selected: HashSet<String> = select
        .split(',')
        .map(|s| s.trim().to_string())
        .filter(|s| !s.is_empty())
        .collect();
    let excluded: HashSet<String> = exclude
        .split(',')
        .map(|s| s.trim().to_string())
        .filter(|s| !s.is_empty())
        .collect();

    let tests = testcases::select(selected, excluded);

    let mut ok = true;

    for test in tests {
        if !test.run().unwrap() {
            ok = false;
        }
    }

    Ok(ok)
}
