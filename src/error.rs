use std::error::Error;
use std::fmt::{write, Display};

/// Errors describing what was wrong with a
/// query while it was being trasnformed into
/// a request
#[derive(Debug)]
pub enum QueryError {
    Error,
}

impl Display for QueryError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            QueryError::Error => {
                writeln!(f, "Query Error")
            }
        }
    }
}

impl Error for QueryError {}

/// Errors describing what was wrong with the
/// response obtained after a request
#[derive(Debug)]
pub enum ParseError {
    ParsableTextNotFound,
    InvalidResponseText,
    JsonUnparsable(String),
}

impl Display for ParseError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            ParseError::ParsableTextNotFound => {
                writeln!(f, "The response text doesn't contain text to be parsed")
            }
            ParseError::InvalidResponseText => {
                writeln!(f, "The response text is malformed and cannot be parsed")
            }
            ParseError::JsonUnparsable(cause) => {
                write(f, format_args!("Error in json parsing, cause: {}", cause))
            }
        }
    }
}

impl Error for ParseError {}

/// Errors describing what went wrong in an
/// interaction with a `PluginManager`
#[derive(Debug)]
pub enum PluginError {
    DuplicatedPlugin(String),
    UnregisteredPlugin(String),
    NoDefaultPlugin,
    DownloadError(String),
    StreamError(String),
}

impl Display for PluginError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            PluginError::DuplicatedPlugin(name) => {
                writeln!(f, "Duplicated plugin: `{}`", name)
            }
            PluginError::UnregisteredPlugin(name) => {
                writeln!(f, "Unregistered plugin: `{}`", name)
            }
            PluginError::NoDefaultPlugin => {
                writeln!(f, "No plugin has been set as default yet")
            }
            PluginError::DownloadError(msg) => {
                writeln!(f, "An error occured while downloading")?;
                writeln!(f, "Here's the cause: {}", msg)
            }
            PluginError::StreamError(msg) => {
                writeln!(f, "An error occured while streaming")?;
                writeln!(f, "Here's the cause: {}", msg)
            }
        }
    }
}

impl Error for PluginError {}
