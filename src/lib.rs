#![allow(dead_code)]
extern crate lz4;
extern crate rusqlite;
#[macro_use]
extern crate rustcane;

mod compression;
mod error;
mod package;
mod resource;

pub use error::Error;
pub use package::Package;
