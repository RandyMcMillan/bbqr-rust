use std::fmt::{self, Display, Formatter};

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
/// The file type, currently only supports UnicodeText, Transaction, PSBT, Binary, and CBOR
pub enum FileType {
    Psbt,
    Transaction,
    Json,
    Cbor,
    UnicodeText,
}

impl FileType {
    pub fn from_byte(byte: u8) -> Option<FileType> {
        match byte {
            b'P' => Some(FileType::Psbt),
            b'T' => Some(FileType::Transaction),
            b'J' => Some(FileType::Json),
            b'C' => Some(FileType::Cbor),
            b'U' => Some(FileType::UnicodeText),
            _ => None,
        }
    }

    pub fn as_byte(&self) -> u8 {
        match self {
            FileType::Psbt => b'P',
            FileType::Transaction => b'T',
            FileType::Json => b'J',
            FileType::Cbor => b'C',
            FileType::UnicodeText => b'U',
        }
    }

    pub fn is_known_filetype(byte: u8) -> bool {
        Self::from_byte(byte).is_some()
    }
}

impl Display for FileType {
    fn fmt(&self, f: &mut Formatter) -> fmt::Result {
        match self {
            FileType::Psbt => write!(f, "PSBT"),
            FileType::Transaction => write!(f, "Transaction"),
            FileType::Json => write!(f, "JSON"),
            FileType::Cbor => write!(f, "CBOR"),
            FileType::UnicodeText => write!(f, "Unicode Text"),
        }
    }
}
