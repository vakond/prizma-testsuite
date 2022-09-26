//! runner.rs

use crate::{
    error::{Error, Result},
    testcases::{self, Test},
};

pub fn execute(select: Option<String>, exclude: Option<String>, sequential: bool) -> Result<()> {
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

    let results = if sequential {
        sequential_run(tests)
    } else {
        parallel_run(tests)
    };

    dbg!(results);

    Ok(())
}

fn sequential_run(mut tests: Vec<(usize, Box<dyn Test + Send>)>) -> Vec<bool> {
    tests
        .iter_mut()
        .map(|test| {
            let t = &test.1;
            t.setup();
            if !t.run().unwrap() {
                println!("failed");
                return false;
            }
            t.teardown();
            println!("ok");
            true
        })
        .collect()
}

fn parallel_run(mut tests: Vec<(usize, Box<dyn Test + Send>)>) -> Vec<bool> {
    use rayon::prelude::*;

    tests
        .par_iter_mut()
        .map(|test| {
            let t = &test.1;
            t.setup();
            if !t.run().unwrap() {
                println!("failed");
                return false;
            }
            t.teardown();
            println!("ok");
            true
        })
        .collect()
}
