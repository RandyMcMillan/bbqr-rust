//! QR code joining

use crate::{
    encoding::Encoding,
    file_type::FileType,
    header::{Header, HeaderParseError},
};

#[derive(Debug, thiserror::Error)]
pub enum JoinError {
    #[error("No data found")]
    Empty,

    #[error("Conflicting/variable file type/encodings/sizes")]
    ConflictingHeaders,

    #[error(transparent)]
    HeaderParseError(#[from] HeaderParseError),
}

fn decode_data(parts: &[String], encoding: char) -> Vec<u8> {
    // Implement the decode_data function based on the encoding
    // This function should decode the data from the given parts based on the encoding
    // and return the decoded data as a vector of bytes
    // You'll need to replace this placeholder implementation with the actual decoding logic
    vec![]
}
// Take scanned data, put into order, decode, return type code and raw data bytes

/// Verify that all the headers have the same variable filetype, encodings and sizes
fn verify_header(parts: &[String]) -> Result<Header, JoinError> {
    if parts.is_empty() {
        return Err(JoinError::Empty);
    }

    // find first non-empty line
    let first_header = parts
        .iter()
        .find(|line| !line.is_empty())
        .ok_or(JoinError::Empty)?;

    let header = Header::try_from_str(first_header)?;

    // verify that all the headers are the same
    for part in parts.iter().skip(1) {
        if part.len() < 6 {
            return Err(JoinError::ConflictingHeaders);
        }

        if part[0..6] != first_header[0..6] {
            return Err(JoinError::ConflictingHeaders);
        }
    }

    Ok(header)
}

