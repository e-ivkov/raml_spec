use protocol::{Protocol, ProtocolParseError};
use std::{collections::HashSet, convert::TryFrom, io::Read, str::FromStr};
use thiserror::Error;
use uri::{ParseError as UriParseError, Uri};
use yaml_rust::{ScanError, YamlLoader};

pub mod uri;

const TITLE: &str = "title";
const DESCRIPTION: &str = "description";
const VERSION: &str = "version";
const BASE_URI: &str = "baseUri";
const BASE_URI_PARAMETERS: &str = "baseUriParameters";
const PROTOCOLS: &str = "protocols";
const MEDIA_TYPE: &str = "mediaType";
const DOCUMENTATION: &str = "documentation";
const SCHEMAS: &str = "schemas";
const TYPES: &str = "types";
const TRAITS: &str = "traits";
const RESOURCE_TYPES: &str = "resourceTypes";
const ANNOTATION_TYPES: &str = "annotationTypes";
const SECURITY_SCHEMES: &str = "securitySchemes";
const SECURED_BY: &str = "secured_by";
const USES: &str = "uses";

#[derive(Debug)]
pub struct RamlSpec {
    pub title: String,
    pub description: Option<String>,
    pub version: Option<String>,
    pub base_uri: Option<Uri>,
    pub protocols: Option<HashSet<Protocol>>,
}

impl RamlSpec {
    pub fn from_reader(reader: &mut impl Read) -> Result<Self, ParseError> {
        let mut raml = String::new();
        let _ = reader
            .read_to_string(&mut raml)
            .map_err(|err| err.to_string());
        let yaml_vec = YamlLoader::load_from_str(&raml)?;
        let yaml = yaml_vec.first().ok_or(ParseError::FileIsEmpty)?;
        Ok(Self {
            title: yaml[TITLE]
                .as_str()
                .ok_or(ParseError::FieldNotFound(TITLE.to_string()))?
                .to_string(),
            description: yaml[DESCRIPTION].as_str().map(String::from),
            version: yaml[VERSION].as_str().map(String::from),
            base_uri: yaml[BASE_URI].as_str().map(FromStr::from_str).transpose()?,
            protocols: yaml[PROTOCOLS]
                .as_vec()
                .map(|protocols| {
                    protocols
                        .iter()
                        .cloned()
                        .map(Protocol::try_from)
                        .collect::<Result<HashSet<_>, _>>()
                })
                .transpose()?,
        })
    }
}

#[derive(Debug, Error)]
pub enum ParseError {
    #[error("Field not found: {0}.")]
    FieldNotFound(String),
    #[error("Incorrect yaml syntax: {0}.")]
    IncorrectYamlSyntax(#[from] ScanError),
    #[error("File is empty.")]
    FileIsEmpty,
    #[error("Incorrect URI: {0}")]
    IncorrectUri(#[from] UriParseError),
    #[error("Failed to parse rotocol: {0}")]
    IncorrectProtocol(#[from] ProtocolParseError),
}

pub mod protocol {
    use std::convert::TryFrom;

    use thiserror::Error;
    use yaml_rust::Yaml;

    pub const HTTP: &str = "HTTP";
    pub const HTTPS: &str = "HTTPS";

    #[derive(Debug, Eq, PartialEq, Hash)]
    pub enum Protocol {
        Http,
        Https,
    }

    #[derive(Debug, Error)]
    pub enum ProtocolParseError {
        #[error("Unsupported protocol: {0}")]
        UnsupportedProtocol(String),
        #[error("Expected YAML string.")]
        InvalidYamlValue,
    }

    impl TryFrom<Yaml> for Protocol {
        type Error = ProtocolParseError;

        fn try_from(value: Yaml) -> Result<Self, Self::Error> {
            match value {
                Yaml::String(protocol) if protocol.as_str() == HTTP => Ok(Protocol::Http),
                Yaml::String(protocol) if protocol.as_str() == HTTPS => Ok(Protocol::Https),
                Yaml::String(protocol) => Err(ProtocolParseError::UnsupportedProtocol(protocol)),
                _ => Err(ProtocolParseError::InvalidYamlValue),
            }
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use anyhow::Result;
    use std::fs::File;

    const FILE_PATH: &str = "./src/test.raml";

    #[test]
    fn parsed_successfully() -> Result<()> {
        let mut file = File::open(FILE_PATH)?;
        let api = RamlSpec::from_reader(&mut file)?;
        assert_eq!(api.title, "Mobile Order API");
        Ok(())
    }
}
