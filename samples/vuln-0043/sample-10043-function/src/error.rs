#[derive(Debug)]
pub enum Error {
    #[cfg(feature = "std")]
    Io(std::io::Error),
    UnrecognizedFormat,
    Cryptography,
    TooLarge,
    TooShort,
}
