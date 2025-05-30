use thiserror::Error;

#[derive(Debug, Error, PartialEq)]
pub enum ParsingError {
    #[error("Opening bracket at operation ? has no matching closing bracket")]
    UnclosedOpeningBracket,

    #[error("Closing bracket at operation ? has no matching opening bracket")]
    UnopenedClosingBracket,
}
