//! Parser for raw data from exam results.
pub use marks::StudentResult;

pub mod award;
pub mod database;
pub mod errors;
pub mod spreadsheet_ml;

mod marks;

use chrono::NaiveDateTime;
use serde::Deserialize;

/// Information about a student.
#[derive(Debug, Default, Deserialize)]
pub struct StudentInfo {
    /// The student ID of the student.
    pub id: i64,
    /// The last name of the student.
    pub last_name: String,
    /// The first name of the student.
    pub first_name: String,
    /// The career number of the student.
    pub carrer_number: Option<i64>,
    /// The academic program taken by the student.
    pub academic_program: Option<String>,
    /// The description of the plan studied.
    pub program_description: Option<String>,
    /// The course plan the student is studying.
    pub plan: String,
    /// The course plan the student is studying.
    pub plan_description: Option<String>,
    /// The intake year of the student.
    pub intake: Option<String>,
    /// The QAA Effective Date of the student.
    pub qaa_effective_date: Option<NaiveDateTime>,
    /// The Degree Calculation Model of the student.
    pub calculation_model: Option<String>,
    /// The final raw mark from the student's result.
    pub raw_mark: Option<f64>,
    /// The final mark from the student's result after truncating percision
    /// from the raw mark.
    pub truncated_mark: Option<f64>,
    /// The final mark from the student's result after all processing.
    pub final_mark: Option<i64>,
    /// The borderline status of the student.
    pub borderline: Option<String>,
    /// The Calculation Review Rqd column of the student.
    pub calculation: Option<bool>,
    /// The Degree Award column of the student.
    pub degree_award: Option<String>,
    /// The Selected column of the student.
    pub selected: Option<bool>,
    /// The Exception Data column of the student.
    pub exception_data: Option<String>,
    /// The recommended action taken for the student.
    pub recommendation: Option<String>,
}

impl StudentInfo {
    pub fn new() -> Self {
        Self::default()
    }
}
