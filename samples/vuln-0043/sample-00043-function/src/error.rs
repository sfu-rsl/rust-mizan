#[derive(Debug)]
pub enum Error {
    #[cfg(feature = "std")]
    Io(std::io::Error),
    UnrecognizedFormat,
    Cryptography,
    TooLarge,
    TooShort,
}

#[cfg(feature = "std")]
impl From<std::io::Error> for Error {
    fn from(err: std::io::Error) -> Self {
        match err.kind() {
            std::io::ErrorKind::UnexpectedEof => Error::TooShort,
            _ => Error::Io(err),
        }
    }
}
