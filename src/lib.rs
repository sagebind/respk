//! ResPK (Resource PacKage) is an archive format designed for bundling game
//! resources, such as data files, images, sounds, music, and videos together
//! into fewer larger files. Bundling resources together can improve performance
//! and also makes it easier to do distribution and patching.
//!
//! The file format is simply a [SQLite] database file with a particular schema.
//! SQLite works well in this situation, as it is an open format that can be
//! easily viewed with many tools, offers high performance for random row
//! access, and allows concurrent reads. Being just a SQLite database also
//! allows you to add any custom extra data or metadata you might need into your
//! own ResPK archives while maintaining compatibility.
//!
//! All resources are compressed individually using the fantastic [LZ4]
//! compression algorithm, which offers a decent amount of compression at
//! blazing fast speed. While not as space-efficient as ZLIB or LZMA, the speed
//! of LZ4 actually outweighs the speed of reading from most storage devices,
//! which means it is actually _faster_ to read a LZ4-compressed file from disk
//! than an uncompressed file due to the reduced reads.
//!
//! [LZ4]: https://github.com/lz4/lz4
//! [SQLite]: https://sqlite.org

mod compression;
mod error;
mod package;
mod resource;

pub use crate::error::Error;
pub use crate::package::Package;
pub use crate::resource::{Resource, ResourceInfo};
