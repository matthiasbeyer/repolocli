pub type Result<T> = std::result::Result<T, RepologyError>;

#[derive(thiserror::Error, Debug)]
pub enum RepologyError {
    #[error("Borrow mutable error")]
    BorrowMutErr(#[from] std::cell::BorrowMutError),

    #[error("UTF8 Error")]
    Utf8Error(#[from] std::string::FromUtf8Error),

    #[error("serde_json error")]
    SerdeJsonError(#[from] serde_json::Error),

    #[error("IO error")]
    IoError(#[from] std::io::Error),

    #[error("curl error")]
    CurlError(#[from] curl::Error),

    #[error("unknown error")]
    Unknown,
}
