use thiserror::Error;

#[derive(Error, Debug)]
pub enum DecodeError {
    #[error("invalid magic number (expected `LSCN` or `76 83 67 78`, got `{normal}` or `{:02X?}`)", bytes)]
    InvalidMagic { normal: String, bytes: [u8; 4] },

    #[error("unexpected eof while reading {0} bytes")]
    UnexpectedEOF(usize),

    #[error("invalid component type found (got `{0}`)")]
    InvalidComponentType(u16),

    #[error("missing fields: {0:?}")]
    MissingComponentFields(Vec<String>),

    #[error("io error: {0}")]
    Io(#[from] std::io::Error),
}
