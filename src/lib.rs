#![allow(dead_code)]
extern crate lz4;
extern crate rusqlite;

mod compression;
mod error;
mod package;
mod resource;

pub use error::Error;
pub use package::Package;
