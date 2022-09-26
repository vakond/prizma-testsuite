//! runner.rs

use crate::{
    error::{Error, Result},
    testcases,
};

pub fn execute(select: Option<String>, exclude: Option<String>) -> Result<()> {
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

    let mut tests = testcases::select(selected, excluded)?;

    use rayon::prelude::*;

    let r: Vec<bool> = tests
        .par_iter_mut()
        .map(|test| {
            let t = &test.1;
            t.setup();
            if !t.run().unwrap() {
                println!("failed");
            }
            t.teardown();
            true
        })
        .collect();
    dbg!(r);

    Ok(())
}
