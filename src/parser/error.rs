use thiserror::Error;

#[derive(Debug, Error, PartialEq)]
pub enum ParsingError {
    #[error("Opening bracket at operation {0} has no matching closing bracket")]
    UnclosedOpeningBracket(usize),

    #[error("Closing bracket at operation {0} has no matching opening bracket")]
    UnopenedClosingBracket(usize),
}
