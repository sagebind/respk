#![allow(dead_code)]
extern crate chrono;
extern crate lz4;
extern crate rusqlite;

#[macro_use]
mod macros;
mod compression;
mod error;
mod locale;
mod resource;

use locale::*;
use resource::*;
use rusqlite::{Connection, DatabaseName};
use rusqlite::blob::Blob;
use std::io::{self, Read};
use std::path::Path;

pub use error::Error;


// Database schema.
const SCHEMA: &'static str = "
    CREATE TABLE IF NOT EXISTS resources (
        path            TEXT PRIMARY KEY NOT NULL,
        locale          TEXT,
        date_created    TEXT,
        date_modified   TEXT,
        size            INTEGER NOT NULL,
        contents        BLOB
    );
";

/// Provides read and write access to resources in a ResPK package file.
pub struct Package {
    db: Connection,
}

impl Package {
    /// Open a package from a file.
    ///
    /// If the file does not exist, it will be created.
    pub fn open<P: AsRef<Path>>(path: P) -> Result<Package, Error> {
        lets! {
            let Ok(connection) = Connection::open(path);
            let Ok(_) = connection.execute(SCHEMA, &[]);
            else Err(Error::SQLError);
            lift Ok(Package {
                db: connection,
            });
        }
    }

    /// Get the number of files in the package.
    pub fn len(&self) -> u64 {
        self.db
            .query_row("SELECT COUNT(*) FROM resources",
                       &[],
                       |row| row.get::<_, i64>(0) as u64)
            .unwrap_or(0)
    }

    /// Get an iterator over the resources in the package.
    pub fn resources(&self) -> Result<Vec<ResourceInfo>, Error> {
        let mut stmt = self.db
            .prepare("
            SELECT path, locale, date_created, date_modified, size, \
                      LENGTH(contents) AS compressed_size
            FROM resources
        ")
            .unwrap();
        let mut rows = stmt.query(&[]).unwrap();
        let mut resources = Vec::new();

        while let Some(Ok(row)) = rows.next() {
            resources.push(ResourceInfo::from_row(&row));
        }

        Ok(resources)
    }

    /// Get a resource by name.
    pub fn get_info<S: AsRef<str>>(&self, path: S) -> Result<ResourceInfo, Error> {
        lets! {
            let Ok(mut stmt) = self.db.prepare("
                SELECT path, locale, date_created, date_modified, size, LENGTH(contents) AS compressed_size
                FROM resources
                WHERE path = ?
            ");
            let Ok(mut rows) = stmt.query(&[&path.as_ref()]);
            let Some(Ok(row)) = rows.next();

            else Err(Error::SQLError);
            lift Ok(ResourceInfo::from_row(&row));
        }
    }

    /// Get the contents of a resource file.
    pub fn get_contents<S: AsRef<str>>(&self, path: S) -> Result<Resource, Error> {
        lets! {
            let Ok(mut stmt) = self.db.prepare("
                SELECT path, locale, date_created, date_modified, size, LENGTH(contents) AS compressed_size, contents
                FROM resources
                WHERE path = ?
            ");
            let Ok(mut rows) = stmt.query(&[&path.as_ref()]);
            let Some(Ok(row)) = rows.next();

            // Decompress the contents.
            let Ok(buf) = {
                let compressed = row.get::<_, Vec<u8>>("contents");
                compression::decompress(io::Cursor::new(compressed))
            };

            else Err(Error::SQLError);
            lift Ok(Resource::new(ResourceInfo::from_row(&row), buf));
        }
    }

    /// Open a resource for reading.
    pub fn get_reader<S: AsRef<str>>(&self, path: S) -> Option<ResourceReader> {
        lets! {
            let Ok(mut stmt) = self.db.prepare("SELECT rowid FROM resources WHERE path = ?");
            let Ok(mut rows) = stmt.query(&[&path.as_ref()]);
            let Some(Ok(row)) = rows.next();
            let Ok(blob) = {
                let id = row.get(0);
                self.db.blob_open(DatabaseName::Main, "resources", "contents", id, true)
            };
            let Ok(decoder) = lz4::Decoder::new(blob);

            else None;
            lift Some(ResourceReader {
                inner: decoder,
            });
        }
    }

    /// Add a new resource to the package.
    pub fn add<S, R>(&self, path: S, contents: R) -> Result<(), Error>
        where S: AsRef<str>,
              R: Read
    {
        self.add_locale(path, None, contents)
    }

    /// Add a resource to the package in the given locale.
    pub fn add_locale<S, L, R>(&self, path: S, locale: L, contents: R) -> Result<(), Error>
        where S: AsRef<str>,
              L: Into<Option<Locale>>,
              R: Read
    {
        let path = path.as_ref();
        let locale_name: Option<String> = locale.into().map(|l| l.to_string());
        let (size, compressed_contents) = compression::compress(contents)?;

        // Insert the data into the database.
        lets! {
            let Ok(mut stmt) = self.db.prepare("
                INSERT INTO resources (path, locale, date_created, date_modified, size, contents)
                    VALUES (?, ?, NOW(), NOW(), ?, ?)
            ");

            let Ok(_) = stmt.insert(&[
                &path,
                &locale_name,
                &(size as i64),
                &compressed_contents,
            ]);

            else Err(error::Error::SQLError);
            lift Ok(());
        }
    }

    /// Update the contents of an existing resource.
    pub fn update<S, L, R>(&self, path: S, locale: L, contents: R) -> Result<(), Error>
        where S: AsRef<str>,
              L: Into<Option<Locale>>,
              R: Read
    {
        let (size, compressed_contents) = compression::compress(contents)?;

        lets! {
            let Ok(count) = self.db.execute("
                UPDATE resources
                SET date_modified = NOW(), size = ?, contents = ?
                WHERE path = ?", &[
                &(size as i64),
                &compressed_contents,
                &path.as_ref()
            ]);
            let 1 = count;

            else Err(error::Error::ResourceNotFound);
            lift Ok(());
        }
    }

    /// Delete a resource from the package.
    pub fn delete<S: AsRef<str>>(&self, path: S) -> Result<(), error::Error> {
        lets! {
            let Ok(count) = self.db.execute("DELETE FROM resources WHERE path = ?", &[&path.as_ref()]);
            let 1 = count;
            else Err(error::Error::ResourceNotFound);
            lift Ok(());
        }
    }
}

/// Reads the contents of a resource incrementally.
pub struct ResourceReader<'pk> {
    inner: lz4::Decoder<Blob<'pk>>,
}

impl<'pk> Read for ResourceReader<'pk> {
    fn read(&mut self, buf: &mut [u8]) -> io::Result<usize> {
        self.inner.read(buf)
    }
}
