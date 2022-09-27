//! error.rs

use std::path::PathBuf;

#[derive(thiserror::Error, Debug)]
pub enum Error {
    #[error("cannot open file '{1}'")]
    OpenFile(#[source] std::io::Error, PathBuf),

    #[error("cannot read line '{1}'")]
    ReadLine(#[source] std::io::Error, String),

    #[error("input/output")]
    Io(#[from] std::io::Error),

    #[error("mutually exclusive options")]
    MutuallyExclusive,

    #[error("testcase '{0}' returns inconsistent name '{1}'")]
    InconsistentTestcase(&'static str, &'static str),

    #[error("index '{0}' is out of range")]
    InvalidIndex(usize),

    #[error("name '{0}' is not recognized as testcase")]
    NameNotFound(String),
}

pub type Result<T> = std::result::Result<T, Error>;
