use compression;
use Error;
use lz4;
use resource::*;
use rusqlite::{Connection, DatabaseName};
use rusqlite::blob::Blob;
use std::io::{self, Read};
use std::path::Path;


// Schema for the SQLite database.
const SCHEMA: &'static str = "
    CREATE TABLE resources (
        path            TEXT NOT NULL,
        size            INTEGER NOT NULL,
        contents        BLOB,

        PRIMARY KEY (path)
    );

    PRAGMA application_id = 31133;
    PRAGMA resources.user_version = 1;
";

/// Provides read and write access to resources in a respk package file.
pub struct Package {
    db: Connection,
}

impl Package {
    /// Open a package from a file.
    ///
    /// If the file does not exist, it will be created.
    pub fn open<P: AsRef<Path>>(path: P) -> Result<Package, Error> {
        let file_exists = path.as_ref().exists();

        // Open the database.
        let connection = Connection::open(path)?;

        // If the database did not already exist, create the tables.
        if !file_exists {
            connection.execute(SCHEMA, &[])?;
        }

        Ok(Package {
            db: connection,
        })
    }

    /// Create a new in-memory package. Used for testing.
    pub(crate) fn temporary() -> Result<Package, Error> {
        let connection = Connection::open_in_memory()?;
        connection.execute(SCHEMA, &[])?;

        Ok(Package {
            db: connection,
        })
    }

    /// Get the number of files in the package.
    pub fn len(&self) -> u64 {
        self.db.query_row("
            SELECT COUNT(*) FROM resources
        ", &[], |row| row.get::<_, i64>(0) as u64).unwrap_or(0)
    }

    /// Get a list of resources in the package.
    pub fn resources(&self) -> Result<Vec<ResourceInfo>, Error> {
        let mut stmt = self.db.prepare("
            SELECT path, size, LENGTH(contents) AS compressed_size
            FROM resources
        ")?;
        let mut rows = stmt.query(&[])?;
        let mut resources = Vec::new();

        while let Some(row) = rows.next() {
            resources.push(ResourceInfo::from_row(&row?));
        }

        Ok(resources)
    }

    /// Get a resource by name.
    pub fn get_info<S: AsRef<str>>(&self, path: S) -> Result<ResourceInfo, Error> {
        let mut stmt = self.db.prepare("
            SELECT path, size, LENGTH(contents) AS compressed_size
            FROM resources
            WHERE path = ?
        ")?;
        let mut rows = stmt.query(&[&path.as_ref()])?;

        match rows.next() {
            Some(row) => Ok(ResourceInfo::from_row(&row?)),
            None => Err(Error::ResourceNotFound),
        }
    }

    /// Read the contents of a resource file.
    pub fn read<S: AsRef<str>>(&self, path: S) -> Result<Resource, Error> {
        let mut stmt = self.db.prepare("
            SELECT path, size, LENGTH(contents) AS compressed_size, contents
            FROM resources
            WHERE path = ?
        ")?;
        let mut rows = stmt.query(&[&path.as_ref()])?;

        let row = match rows.next() {
            Some(row) => row?,
            None => return Err(Error::ResourceNotFound),
        };

        // Decompress the contents.
        let compressed_contents = row.get::<_, Vec<u8>>("contents");
        let contents = compression::decompress(io::Cursor::new(compressed_contents))?;

        Ok(Resource::new(ResourceInfo::from_row(&row), contents))
    }

    /// Open a resource for reading.
    ///
    /// This method allows you to stream a resource from disk and process the resource incrementally instead of waiting
    /// for the entire resource file to be read into memory. If you are able to use a streamed resource, this method is
    /// usually preferred over `read()`.
    pub fn stream<S: AsRef<str>>(&self, path: S) -> Result<ResourceReader, Error> {
        let mut stmt = self.db.prepare("SELECT rowid FROM resources WHERE path = ?")?;
        let mut rows = stmt.query(&[&path.as_ref()])?;

        let row = match rows.next() {
            Some(row) => row?,
            None => return Err(Error::ResourceNotFound),
        };

        let rowid = row.get(0);
        let blob = self.db.blob_open(DatabaseName::Main, "resources", "contents", rowid, true)?;
        let decoder = lz4::Decoder::new(blob).map_err(|_| Error::DecompressionError)?;

        Ok(ResourceReader {
            inner: decoder,
        })
    }

    /// Write the given data stream to a resource path.
    pub fn write<S, R>(&self, path: S, contents: R) -> Result<(), Error>
        where S: AsRef<str>,
              R: Read
    {
        let path = path.as_ref();

        // Compress the contents before we get too excited.
        let (size, compressed_contents) = compression::compress(contents)?;

        // Insert a new row for the resource. If a row exists already under the given path, it will be replaced instead.
        self.db.execute("
            INSERT OR REPLACE INTO resources (path, size, contents)
            VALUES (?, ?, ?)
        ", &[&path, &(size as i64), &compressed_contents])?;

        Ok(())
    }

    /// Delete a resource from the package.
    pub fn delete<S>(&self, path: S) -> Result<(), Error>
        where S: AsRef<str>
    {
        let path = path.as_ref();

        let count = self.db.execute("
            DELETE FROM resources
            WHERE path = ?
        ", &[&path])?;

        if count >= 1 {
            Ok(())
        } else {
            Err(Error::ResourceNotFound)
        }
    }
}


/// Reads the contents of a resource incrementally.
///
/// If the resource is compressed, it will be decompressed on-the-fly.
pub struct ResourceReader<'pk> {
    inner: lz4::Decoder<Blob<'pk>>,
}

impl<'pk> Read for ResourceReader<'pk> {
    fn read(&mut self, buf: &mut [u8]) -> io::Result<usize> {
        self.inner.read(buf)
    }
}
