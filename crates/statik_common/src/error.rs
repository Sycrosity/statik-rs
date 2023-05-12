use thiserror::Error;

#[derive(Debug, Error)]
pub enum StatikError {
    // #[error("json ser/de error")]
    // Json(#[from] serde_json::Error),
    #[error("encoding (write) error")]
    Encode(#[from] crate::error::EncodeError),

    #[error("decoding (read) error")]
    Decode(#[from] crate::error::DecodeError),
}

#[derive(Debug, Error)]
pub enum DecodeError {
    #[error("decode (read) IO error")]
    IO(#[from] std::io::Error),

    #[error("VarInt exceeds maximum capacity of 5 bytes (2147483647/-2147483648)")]
    VarIntTooLarge,
}

#[derive(Debug, Error)]
pub enum EncodeError {
    #[error("encode (write) IO error")]
    IO(#[from] std::io::Error),
}
