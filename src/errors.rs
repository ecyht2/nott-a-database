//! The errors returned by the parsers.
use std::{error::Error, fmt::Display};

#[derive(Debug)]
/// Errors when parsing a [`StudentResult`](crate::StudentResult) from the raw data.
pub enum ParseResultError {
    /// No/Invalid student ID found in data.
    InvalidID,
    /// No/Invalid student first name found in data.
    InvalidFirstName,
    /// No/Invalid student last name found in data.
    InvalidLastName,
    /// No/Invalid student study plan found in data.
    InvalidPlan,
    /// No/Invalid year of program found in data.
    InvalidYearOfProgram,
    /// No/Invalid progression information found in data.
    InvalidProgression,
    /// No/Invalid module information found in data.
    InvalidModule,
}

impl Display for ParseResultError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let output = match self {
            ParseResultError::InvalidID => "No/Invalid student ID.",
            ParseResultError::InvalidFirstName => "No/Invalid first name.",
            ParseResultError::InvalidLastName => "No/Invalid last name.",
            ParseResultError::InvalidPlan => "No/Invalid plan.",
            ParseResultError::InvalidYearOfProgram => "No/Invalid year of program.",
            ParseResultError::InvalidProgression => "No/Invalid progression status.",
            ParseResultError::InvalidModule => "No/Invalid module.",
        };
        write!(f, "{}", output)
    }
}

#[derive(Debug)]
/// Errors when parsing the raw data.
pub enum ParseDataError {
    /// Invalid result entry in the data.
    InvalidResult(ParseResultError),
}

impl Display for ParseDataError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let output = match self {
            ParseDataError::InvalidResult(_) => "lmao",
        };
        write!(f, "{}", output)
    }
}

impl Error for ParseResultError {}

#[derive(Debug)]
/// Invalid header found for the data.
pub struct InvalidHeader(pub String);

impl Display for InvalidHeader {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "Invalid Header Found: {}", self.0)
    }
}

impl Error for InvalidHeader {}
