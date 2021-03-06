use std::fs::File;
use std::{io, str, fmt};
use std::io::Read;
use std::str::FromStr;
use std::path::Path;

use linalg::Matrix;

fn file_to_string<P: AsRef<Path>>(path: P) -> Result<String, io::Error> {
    let mut s = String::new();
    File::open(path)
        .and_then(|mut f| f.read_to_string(&mut s))
        .map(|_bytes| s)
}

pub fn matrix_from_txt<P: AsRef<Path>, T>(path: P) -> Result<Matrix<T>, String>
        where T: str::FromStr, <T as str::FromStr>::Err: fmt::Display {
    file_to_string(path)
        .or_else(|e| Err(e.to_string()))
        .and_then(|r| Matrix::from_str(r.as_str()).or_else(|e| Err(e.to_string())) )
}
