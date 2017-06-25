//! Provides methods for compressing and decompressing resource file data.
//! Compression is done using the LZ4 algorithm, which trades some compression size for high speed decompressing.
use Error;
use lz4;
use std::io::{self, Read};


/// Compress an input stream and return the results and the uncompressed size.
pub fn compress<R: Read>(mut input: R) -> Result<(u64, Vec<u8>), Error> {
    lets! {
        let Ok(mut encoder) = lz4::EncoderBuilder::new().build(Vec::new());
        let Ok(size) = io::copy(&mut input, &mut encoder);
        let (buf, Ok(_)) = encoder.finish();

        else Err(Error::CompressionError);
        lift Ok((size, buf));
    }
}

/// Decompress an input stream and return the results.
pub fn decompress<R: Read>(input: R) -> Result<Vec<u8>, Error> {
    let mut buf = Vec::new();

    lets! {
        let Ok(mut decoder) = lz4::Decoder::new(input);
        let Ok(_) = io::copy(&mut decoder, &mut buf);

        else Err(Error::DecompressionError);
        lift Ok(buf);
    }
}
