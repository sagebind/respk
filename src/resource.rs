use rusqlite::Row;
use std::ops::Deref;
use std::str;


/// Provides information about a resource.
#[derive(Clone, Debug)]
pub struct ResourceInfo {
    path: String,
    size: u32,
    compressed_size: u32,
}

impl ResourceInfo {
    /// Get the path of the resource.
    pub fn path(&self) -> &str {
        &self.path
    }

    /// Get the full size of the resource contents.
    pub fn size(&self) -> u32 {
        self.size
    }

    /// Get the compressed size of the resource.
    pub fn compressed_size(&self) -> u32 {
        self.compressed_size
    }

    pub(crate) fn from_row(row: &Row) -> ResourceInfo {
        ResourceInfo {
            path: row.get("path"),
            size: row.get("size"),
            compressed_size: row.get("compressed_size"),
        }
    }
}


/// A decompressed resource.
#[derive(Debug)]
pub struct Resource {
    info: ResourceInfo,
    contents: Vec<u8>,
}

impl Resource {
    pub(crate) fn new(info: ResourceInfo, contents: Vec<u8>) -> Resource {
        Resource {
            info: info,
            contents: contents,
        }
    }

    /// Get the contents of the resource.
    pub fn contents(&self) -> &[u8] {
        &self.contents
    }

    /// Get the contents of the resource as a string.
    pub fn contents_str(&self) -> Result<&str, str::Utf8Error> {
        str::from_utf8(&self.contents)
    }
}

impl Deref for Resource {
    type Target = ResourceInfo;

    fn deref(&self) -> &ResourceInfo {
        &self.info
    }
}
