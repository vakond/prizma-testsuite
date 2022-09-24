//! config.rs

pub const FAILURE: i32 = 1;

include!(concat!(env!("OUT_DIR"), "/constant_list_of_testcases.rs"));

pub fn enumerate_testcases() -> Vec<(usize, String)> {
    TESTCASES
        .lines()
        .map(|s| {
            let mut s = s.split_whitespace();
            let i = s.next().unwrap().parse::<usize>().unwrap();
            let name = s.next().unwrap().to_string();
            (i, name)
        })
        .collect()
}
