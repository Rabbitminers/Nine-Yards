pub mod compression;
pub mod upload;
pub mod extensions;

#[derive(Error, Debug)]
pub enum FileSystemError {
    #[error("Error while encoding: {0}")]
    Encoding(String),

    #[error("Error while decoding: {0}")]
    Decoding(String),

    #[error("Entry already exists")]
    AlreadyExists
}