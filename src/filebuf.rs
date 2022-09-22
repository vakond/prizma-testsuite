//! filebuf.rs

use crate::error::{Error, Result};

use std::{
    fs::File,
    io::{BufRead, BufReader},
    path::Path,
};

/// Opens a file for buffered reading.
#[allow(unused)]
pub fn open(filepath: &Path) -> Result<impl BufRead> {
    let file = File::open(&filepath).map_err(|e| Error::OpenFile(e, filepath.into()))?;
    Ok(BufReader::new(file))
}

/// Reads all bytes until a newline (the `0xA` byte) is reached,
/// and puts them to the provided buffer replacing the buffer's contents.
#[allow(unused)]
pub fn read_line(line: &mut String, reader: &mut impl BufRead) -> Result<usize> {
    line.clear();
    reader
        .read_line(line)
        .map_err(|e| Error::ReadLine(e, line.clone()))
}
