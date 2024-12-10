pub(crate) type CipherResult<R> = Result<R, CipherError>;

#[derive(Debug)]
pub enum CipherError {
    InvalidPin(String),
    IncorrectPan(String),
    InvalidKey(String),
    ErrorProcessingString(String),
    InternalError(String),
    InvalidContent(String),
    ParsingError(String),
}
