use chrono::prelude::*;
use locale::Locale;
use rusqlite::Row;
use std::ops::Deref;
use std::str;


/// Provides information about a resource.
#[derive(Clone)]
pub struct ResourceInfo {
    path: String,
    locale: Option<Locale>,
    size: u32,
    compressed_size: u32,
    date_created: Option<DateTime<UTC>>,
    date_modified: Option<DateTime<UTC>>,
}

impl ResourceInfo {
    /// Get the path of the resource.
    pub fn path(&self) -> &str {
        &self.path
    }

    /// Get the locale of the resource, if any.
    pub fn locale(&self) -> Option<&Locale> {
        self.locale.as_ref()
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
            locale: row.get::<_, Option<String>>("locale").map(Locale::from),
            size: row.get("size"),
            compressed_size: row.get("compressed_size"),
            date_created: row.get("date_created"),
            date_modified: row.get("date_modified"),
        }
    }
}

/// A decompressed resource.
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
