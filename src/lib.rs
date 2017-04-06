extern crate rusqlite;

use rusqlite::*;
use rusqlite::blob::Blob;
use std::io::{self, Read, Seek, SeekFrom};
use std::path::Path;


const SCHEMA: &'static str = "
CREATE TABLE resources (
    path            TEXT PRIMARY KEY NOT NULL,
    date_created    TEXT,
    date_modified   TEXT,
    permissions     INTEGER,
    size            INTEGER NOT NULL,
    checksum        TEXT,
    data            BLOB
)
";


pub struct Package {
    db: Connection,
}

impl Package {
    pub fn new() -> Package {
        let connection = Connection::open_in_memory().unwrap();

        connection.execute(SCHEMA, &[]).expect("failed to execute schema");

        Package {
            db: connection,
        }
    }

    pub fn open<P: AsRef<Path>>(path: P) -> Package {
        Package {
            db: Connection::open(path).unwrap(),
        }
    }

    pub fn create_resource<R: Read>(&self, filename: &str, reader: R)
    {
        self.db.execute("INSERT INTO files VALUES (?, NULL, NULL, NULL, ?, ?, ?)", &[&filename]);
    }

    /// Get a resource file for reading.
    pub fn get_resource<'pk, S: AsRef<str>>(&'pk self, name: S) -> Option<Resource<'pk>> {
        self.get_resource_id(name.as_ref()).map(|id| {
            let blob = self.db.blob_open(DatabaseName::Main, "files", "contents", id, true).unwrap();

            Resource {
                blob: blob,
            }
        })
    }

    /// Get a resource for writing.
    pub fn get_resource_mut<'pk, S: AsRef<str>>(&'pk self, name: S) -> Option<Resource<'pk>> {
        self.get_resource_id(name.as_ref()).map(|id| {
            let blob = self.db.blob_open(DatabaseName::Main, "files", "contents", id, false).unwrap();

            Resource {
                blob: blob,
            }
        })
    }

    /// Get the index ID for a file.
    fn get_resource_id(&self, name: &str) -> Option<i64> {
        let mut stmt = self.db.prepare("SELECT rowid FROM files WHERE path = ?").unwrap();
        let mut rows = stmt.query(&[&name]).unwrap();

        match rows.next() {
            Some(Ok(row)) => Some(row.get(0)),
            _ => None,
        }
    }
}

pub struct Resource<'pk> {
    blob: Blob<'pk>,
}

impl<'pk> Read for Resource<'pk> {
    fn read(&mut self, buf: &mut [u8]) -> io::Result<usize> {
        self.blob.read(buf)
    }
}

impl<'pk> Seek for Resource<'pk> {
    fn seek(&mut self, pos: SeekFrom) -> io::Result<u64> {
        self.blob.seek(pos)
    }
}
