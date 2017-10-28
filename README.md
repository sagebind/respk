# ResPK
ResPK is a fast and open SQLite-based archive format specifically designed for storing game resources.

[![Shippable](https://img.shields.io/shippable/59f4fe469ce1cc0700911943.svg)](https://app.shippable.com/github/sagebind/respk)
[![Crates.io](https://img.shields.io/crates/v/respk.svg)](https://crates.io/crates/respk)
[![Documentation](https://docs.rs/respk/badge.svg)](https://docs.rs/respk)
![License](https://img.shields.io/badge/license-MIT-blue.svg)

## Overview
ResPK (Resource PacKage) is an archive format designed for bundling game resources, such as data files, images, sounds, music, and videos together into fewer larger files. Bundling resources together can improve performance and also makes it easier to do distribution and patching.

The file format is simply a [SQLite] database file with a particular schema. SQLite works well in this situation, as it is an open format that can be easily viewed with many tools, offers high performance for random row access, and allows concurrent reads. Being just a SQLite database also allows you to add any custom extra data or metadata you might need into your own ResPK archives while maintaining compatibility.

All resources are compressed individually using the fantastic [LZ4] compression algorithm, which offers a decent amount of compression at blazing fast speed. While not as space-efficient as ZLIB or LZMA, the speed of LZ4 actually outweighs the speed of reading from most storage devices, which means it is actually _faster_ to read a LZ4-compressed file from disk than an uncompressed file due to the reduced reads.

## Features
- **Fast random access:** Designed from the get-go to give really fast read times and random access.
- **Compression:** Provides per-file compression to reduce overall file size and also to improve speed.
- **Streaming:** Resources can be read incrementally from disk using on-the-fly decompression.
- **Lightweight:** The library contains minimal dependencies to keep it lightweight.

## Installation
Add this to your `Cargo.toml` file:

```rust
[dependencies]
respk = "0.1"
```

## License
Available under the MIT license. See the [LICENSE](LICENSE) file for more info.


[LZ4]: https://github.com/lz4/lz4
[SQLite]: https://sqlite.org
