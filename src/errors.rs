//! The errors returned by the parsers.
use std::{error::Error, fmt::Display};

use calamine::XlsxError;

#[derive(Debug)]
/// Errors when parsing a [`StudentResult`](crate::StudentResult) from the raw data.
pub enum ParseResultError {
    /// No/Invalid student ID found in data.
    InvalidID,
    /// No/Invalid student last name found in data.
    InvalidLastName,
    /// No/Invalid student first name found in data.
    InvalidFirstName,
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
            ParseResultError::InvalidLastName => "No/Invalid last name.",
            ParseResultError::InvalidFirstName => "No/Invalid first name.",
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

/// Errors when parsing a row of award report (0B) raw data.
#[derive(Debug)]
pub enum ParseAwardRowError {
    /// No/Invalid ID row.
    InvalidId,
    /// No/Invalid student last name found in data.
    InvalidLastName,
    /// No/Invalid student first name found in data.
    InvalidFirstName,
    /// No/Invalid career number of the student.
    InvalidCareerNumber,
    /// No/Invalid academice program of the student.
    InvalidAcademicProgram,
    /// No/Invalid description of the program studied.
    InvalidProgramDescription,
    /// No/Invalid academic plan of the student.
    InvalidAcademicPlan,
    /// No/Invalid description of the plan studied.
    InvalidPlanDescription,
    /// No/Invalid intake year of the student.
    InvalidIntake,
    /// No/Invalid QAA Effective Date of the student.
    InvalidQAAEffectiveDate,
    /// No/Invalid Degree Calculation Model of the student.
    InvalidDegreeCalculationModel,
    /// No/Invalid final raw mark from the student's result.
    InvalidRawFinalMark,
    /// No/Invalid final mark from the student's result after truncating percision
    /// from the raw mark.
    InvalidTruncatedFinalMark,
    /// No/Invalid final mark from the student's result after all processing.
    InvalidFinalMark,
    /// No/Invalid borderline status of the student.
    InvalidBorderline,
    /// No/Invalid Calculation Review Rqd column of the student.
    InvalidCalculationReviewRqd,
    /// No/Invalid Degree Award column of the student.
    InvalidDegreeAward,
    /// No/Invalid Selected column of the student.
    InvalidSelected,
    /// No/Invalid Exception Data column of the student.
    InvalidExceptionData,
    /// No/Invalid recommended action taken for the student.
    InvalidRecommendation,
}

impl Display for ParseAwardRowError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            ParseAwardRowError::InvalidId => write!(f, "No/Invalid student ID column."),
            ParseAwardRowError::InvalidLastName => write!(f, "No/Invalid Surname column."),
            ParseAwardRowError::InvalidFirstName => write!(f, "No/Invalid First Name column."),
            ParseAwardRowError::InvalidCareerNumber => {
                write!(f, "No/Invalid Career Number column.")
            }
            ParseAwardRowError::InvalidAcademicProgram => {
                write!(f, "No/Invalid Academic Program column.")
            }
            ParseAwardRowError::InvalidProgramDescription => {
                write!(f, "No/Invalid Program Description column.")
            }
            ParseAwardRowError::InvalidAcademicPlan => {
                write!(f, "No/Invalid Academic Plan column.")
            }
            ParseAwardRowError::InvalidPlanDescription => {
                write!(f, "No/Invalid Plan Description column.")
            }
            ParseAwardRowError::InvalidIntake => write!(f, "No/Invalid Intake column."),
            ParseAwardRowError::InvalidQAAEffectiveDate => {
                write!(f, "No/Invalid QAA Effective Date column.")
            }
            ParseAwardRowError::InvalidDegreeCalculationModel => {
                write!(f, "No/Invalid Degree Calculation Model column.")
            }
            ParseAwardRowError::InvalidRawFinalMark => {
                write!(f, "No/Invalid Raw Final Mark column.")
            }
            ParseAwardRowError::InvalidTruncatedFinalMark => {
                write!(f, "No/Invalid Truncated Final Mark column.")
            }
            ParseAwardRowError::InvalidFinalMark => write!(f, "No/Invalid Final Mark column."),
            ParseAwardRowError::InvalidBorderline => write!(f, "No/Invalid Borderline? column."),
            ParseAwardRowError::InvalidCalculationReviewRqd => {
                write!(f, "No/Invalid Calculation Review Rqd column.")
            }
            ParseAwardRowError::InvalidDegreeAward => write!(f, "No/Invalid Degree Award column."),
            ParseAwardRowError::InvalidSelected => write!(f, "No/Invalid Selected column."),
            ParseAwardRowError::InvalidExceptionData => {
                write!(f, "No/Invalid Exception Data column.")
            }
            ParseAwardRowError::InvalidRecommendation => {
                write!(f, "No/Invalid Recommendation column.")
            }
        }
    }
}

impl Error for ParseAwardRowError {}

/// Errors when parsing award report (0B) raw data.
#[derive(Debug)]
pub enum ParseAwardError {
    /// Found an invalid row in raw data.
    InvalidRow(usize, ParseAwardRowError),
}

impl Display for ParseAwardError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            ParseAwardError::InvalidRow(row, err) => write!(f, "{err} at row {row}"),
        }
    }
}

impl Error for ParseAwardError {}

/// Errors when parsing a row of May resit report (0C) raw data.
#[derive(Debug)]
pub enum ParseMayResitRowError {
    /// No/Invalid student ID found in data.
    InvalidID,
    /// No/Invalid student last name found in data.
    InvalidLastName,
    /// No/Invalid student first name found in data.
    InvalidFirstName,
    /// No/Invalid student study plan found in data.
    InvalidPlan,
    /// No/Invalid year of program found in data.
    InvalidYearOfProgram,
    /// No/Invalid autumn credit found in data.
    InvalidAutumnCredit,
    /// The average/mean marks of the student in the Autumn Semester.
    InvalidAutumnMean,
    /// The amount of credits taken by the student in the Spring Semester.
    InvalidFullCredit,
    /// The amount of credits taken by the student in the Spring Semester.
    InvalidFullMean,
    /// The amount of credits taken by the student in the entire year.
    InvalidSpringCredit,
    /// The average/mean marks of the student in the entire year.
    InvalidSpringMean,
    /// The amount of credits taken by the student in the entire year.
    InvalidYearCredit,
    /// The average/mean marks of the student in the entire year.
    InvalidYearProgAverage,
    /// Credits (L3) <30
    InvalidCreditsL3Lt30,
    /// Credits (L3) 30-39
    InvalidCreditsL33039,
    /// No/Invalid progression information found in data.
    InvalidProgression,
    /// No/Invalid module information found in data.
    InvalidCourse,
    /// No/Invalid remarks found in data.
    InvalidRemarks,
}

impl Display for ParseMayResitRowError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let output = match self {
            Self::InvalidID => "No/Invalid Student ID column.",
            Self::InvalidLastName => "No/Invalid Last Name column.",
            Self::InvalidFirstName => "No/Invalid First Name column.",
            Self::InvalidPlan => "No/Invalid Plan column.",
            Self::InvalidYearOfProgram => "No/Invalid Year Of Program column.",
            Self::InvalidAutumnCredit => "No/Invalid Autumn Credit column.",
            Self::InvalidAutumnMean => "No/Invalid Autumn Mean column.",
            Self::InvalidFullCredit => "No/Invalid Full Credit column.",
            Self::InvalidFullMean => "No/Invalid Full Mean column.",
            Self::InvalidSpringCredit => "No/Invalid Spring Credit column.",
            Self::InvalidSpringMean => "No/Invalid Spring Mean column.",
            Self::InvalidYearCredit => "No/Invalid Year Credit column.",
            Self::InvalidYearProgAverage => "No/Invalid Year Mean column.",
            Self::InvalidCreditsL3Lt30 => "No/Invalid Credits <30 column.",
            Self::InvalidCreditsL33039 => "No/Invalid Credits 30-39 column.",
            Self::InvalidProgression => "No/Invalid Progression column.",
            Self::InvalidCourse => "No/Invalid Course column.",
            Self::InvalidRemarks => "No/Invalid Remarks column.",
        };
        write!(f, "{}", output)
    }
}

impl Error for ParseMayResitRowError {}

/// Errors when parsing August resit report (0D) raw data.
#[derive(Debug)]
pub enum ParseMayResitError {
    /// An error occured when opening the row data workbook.
    WorkbookError(XlsxError),
    /// Invalid amount of worksheets found in raw data.
    InvalidWorksheet,
    /// Unable to find headers.
    NoHeaders,
    /// Invalid headers found when parsing resit report.
    InvalidHeaders(String),
    /// Unable to find subheaders.
    NoSubheader,
    /// Found an invalid row in raw data.
    InvalidDataRow(usize, ParseMayResitRowError),
}

impl Display for ParseMayResitError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Self::WorkbookError(e) => {
                write!(f, "Error: {e} occured when opening resit report.")
            }
            Self::InvalidWorksheet => {
                write!(f, "No worksheet \"Sheet1\" found in the workbook.")
            }
            Self::NoHeaders => {
                write!(f, "No header row found when parsing Spring resit report")
            }
            Self::InvalidHeaders(s) => {
                write!(
                    f,
                    "No/Invalid headers {s} found when spring parsing resit report"
                )
            }
            Self::NoSubheader => {
                write!(f, "No subheader row found when parsing spring resit report")
            }
            Self::InvalidDataRow(row, e) => write!(f, "{e} at data {row}"),
        }
    }
}

impl Error for ParseMayResitError {}

/// Errors when parsing a row of August resit report (0D) raw data.
#[derive(Debug)]
pub enum ParseAugResitRowError {
    /// No/Invalid student ID found in data.
    InvalidID,
    /// No/Invalid student last name found in data.
    InvalidLastName,
    /// No/Invalid student first name found in data.
    InvalidFirstName,
    /// No/Invalid student study plan found in data.
    InvalidPlan,
    /// No/Invalid year of program found in data.
    InvalidYearOfProgram,
    /// No/Invalid autumn credit found in data.
    InvalidAutumnCredit,
    /// The average/mean marks of the student in the Autumn Semester.
    InvalidAutumnMean,
    /// The amount of credits taken by the student in the Spring Semester.
    InvalidFullCredit,
    /// The amount of credits taken by the student in the Spring Semester.
    InvalidFullMean,
    /// The amount of credits taken by the student in the entire year.
    InvalidSpringCredit,
    /// The average/mean marks of the student in the entire year.
    InvalidSpringMean,
    /// The amount of credits taken by the student in the entire year.
    InvalidYearCredit,
    /// The average/mean marks of the student in the entire year.
    InvalidYearProgAverage,
    /// Credits (L3) <30
    InvalidCreditsL3Lt30,
    /// Credits (L3) 30-39
    InvalidCreditsL33039,
    /// No/Invalid progression information found in data.
    InvalidProgression,
    /// No/Invalid module information found in data.
    InvalidCourse,
    /// No/Invalid remarks found in data.
    InvalidRemarks,
}

impl Display for ParseAugResitRowError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let output = match self {
            Self::InvalidID => "No/Invalid Student ID column.",
            Self::InvalidLastName => "No/Invalid Last Name column.",
            Self::InvalidFirstName => "No/Invalid First Name column.",
            Self::InvalidPlan => "No/Invalid Plan column.",
            Self::InvalidYearOfProgram => "No/Invalid Year Of Program column.",
            Self::InvalidAutumnCredit => "No/Invalid Autumn Credit column.",
            Self::InvalidAutumnMean => "No/Invalid Autumn Mean column.",
            Self::InvalidFullCredit => "No/Invalid Full Credit column.",
            Self::InvalidFullMean => "No/Invalid Full Mean column.",
            Self::InvalidSpringCredit => "No/Invalid Spring Credit column.",
            Self::InvalidSpringMean => "No/Invalid Spring Mean column.",
            Self::InvalidYearCredit => "No/Invalid Year Credit column.",
            Self::InvalidYearProgAverage => "No/Invalid Year Mean column.",
            Self::InvalidCreditsL3Lt30 => "No/Invalid Credits <30 column.",
            Self::InvalidCreditsL33039 => "No/Invalid Credits 30-39 column.",
            Self::InvalidProgression => "No/Invalid Progression column.",
            Self::InvalidCourse => "No/Invalid Course column.",
            Self::InvalidRemarks => "No/Invalid Remarks column.",
        };
        write!(f, "{}", output)
    }
}

impl Error for ParseAugResitRowError {}

/// Errors when parsing August resit report (0D) raw data.
#[derive(Debug)]
pub enum ParseAugResitError {
    /// An error occured when opening the row data workbook.
    WorkbookError(XlsxError),
    /// Invalid amount of worksheets found in raw data.
    InvalidWorksheet(usize),
    /// Invalid headers found when parsing resit report.
    InvalidHeaders,
    /// Unable to find subheaders.
    NoSubheader,
    /// Found an invalid row in raw data.
    InvalidDataRow(usize, ParseAugResitRowError),
}

impl Display for ParseAugResitError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Self::WorkbookError(e) => {
                write!(f, "Error: {e} occured when opening resit report.")
            }
            Self::InvalidWorksheet(count) => {
                write!(
                    f,
                    "Invalid amount of worksheets found in resit report, expected 1 found {count}"
                )
            }
            Self::InvalidHeaders => {
                write!(f, "No/Invalid headers found when parsing resit report")
            }
            Self::NoSubheader => {
                write!(f, "No subheader row found when parsing resit report")
            }
            Self::InvalidDataRow(row, e) => write!(f, "{e} at data {row}"),
        }
    }
}

impl Error for ParseAugResitError {}
