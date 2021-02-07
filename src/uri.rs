use std::{
    convert::{TryFrom, TryInto},
    str::FromStr,
};
use thiserror::Error;
use uriparse::uri::{URIError, URI as ParsedUri};

#[derive(Debug, Error)]
pub enum ParseError {
    #[error("Invalid syntax: {0}")]
    InvalidSyntax(#[from] URIError),
}

#[derive(Debug)]
pub struct Uri {
    raw: String,
}

impl Uri {
    pub fn parsed(&self) -> ParsedUri {
        self.raw.as_str().try_into().expect("Failed to parse.")
    }
}

impl FromStr for Uri {
    type Err = ParseError;

    fn from_str(raw: &str) -> Result<Self, Self::Err> {
        let _ = ParsedUri::try_from(raw)?;
        Ok(Uri {
            raw: raw.to_string(),
        })
    }
}
