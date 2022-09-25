//! runner.rs

use crate::{
    error::{Error, Result},
    testcases,
};

pub fn execute(select: Option<String>, exclude: Option<String>) -> Result<bool> {
    use std::collections::HashSet;

    if select.is_some() && exclude.is_some() {
        return Err(Error::MutuallyExclusive);
    }

    let mut selected = HashSet::new();
    if let Some(select) = select {
        selected = select.split(',').map(|s| s.trim().to_string()).collect();
    }
    let mut excluded = HashSet::new();
    if let Some(exclude) = exclude {
        excluded = exclude.split(',').map(|s| s.trim().to_string()).collect();
    }

    let tests = testcases::select(selected, excluded)?;

    let mut ok = true;

    for (_, test) in tests {
        if !test.run().unwrap() {
            ok = false;
        }
    }

    Ok(ok)
}
