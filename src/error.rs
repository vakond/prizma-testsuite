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
}

pub type Result<T> = std::result::Result<T, Error>;
