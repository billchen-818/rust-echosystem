use anyhow::Context;
use std::fs;

use thiserror::Error;

#[derive(Error, Debug)]
pub enum MyError {
    #[error("I/O error")]
    Io(#[from] std::io::Error),
    #[error("Parse error")]
    Parse(#[from] std::num::ParseIntError),
    #[error("Serization error: {0}")]
    Serde(#[from] serde_json::Error),
    #[error("Custom error: {0}")]
    Custom(String),
    // #[error("Error: {a}, {b:?}, {c:?}, {d}")]
    // BigError {
    //     a: String,
    //     b: Vec<String>,
    //     c: [u8; 64],
    //     d: u64,
    // },
    #[error("Error: {0:?}")]
    MyBigError(Box<MyBigError>),
}

#[derive(Debug)]
pub struct MyBigError {
    pub a: String,
    pub b: Vec<String>,
    pub c: [u8; 64],
    pub d: u64,
}

fn main() -> Result<(), anyhow::Error> {
    println!("Hello, world!");
    println!("size of MyError: {}", size_of::<MyError>());
    let filename = "non_existent_file.txt";
    let _fd =
        fs::File::open(filename).with_context(|| format!("Can not find file: {}", filename))?;

    fail_with_error()?;

    Ok(())
}

fn fail_with_error() -> Result<(), MyError> {
    Err(MyError::Custom("Something went wrong".to_string()))
}
