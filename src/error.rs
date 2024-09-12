use miette::{Diagnostic, SourceSpan};
use thiserror::Error;

#[derive(Diagnostic, Debug, Error)]
#[error("Unexpected EOF")]
pub struct Eof;

#[derive(Diagnostic, Debug, Error)]
#[error("Unexpected token '{token}' in input")]
pub struct SingleTokenError {
    // The `Source` that miette will use.
    #[source_code]
    pub(crate) src: String,

    pub token: char,

    #[label = "this input character"]
    pub(crate) err_span: SourceSpan,
}

#[derive(Diagnostic, Debug, Error)]
#[error("Unterminated string")]
pub struct UnterminatedStringError {
    // The `Source` that miette will use.
    #[source_code]
    pub(crate) src: String,

    #[label = "here"]
    pub(crate) err_span: SourceSpan,
}