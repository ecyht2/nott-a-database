//! Parser for raw data from exam results.

pub mod database;
pub mod errors;
pub mod spreadsheet_ml;

mod award;
mod marks;
mod resit;

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

/// A struct describing an ARGB colour in the workbook.
#[derive(Clone, Debug, Default, Deserialize, PartialEq)]
pub struct ColourValue {
    /// The alpha (transparency) channel value of the colour.
    pub alpha: u8,
    /// The red channel value of the colour.
    pub red: u8,
    /// The green channel value of the colour.
    pub green: u8,
    /// The blue channel value of the colour.
    pub blue: u8,
}

/// Container struct for a module information.
#[derive(Clone, Debug, Default, Deserialize, PartialEq)]
pub struct Mark {
    /// The module code of the module taken by the student.
    pub code: String,
    /// The number of credits of the module taken by the student.
    pub credit: i64,
    /// The current status of the moduel (Pass, Soft-Fail, Hard-Fail,
    /// Component-Fail).
    pub status: ModuleStatus,
    /// The fill of the cell.
    pub fill: Option<ColourValue>,
    /// The first result of the user taken from the student.
    pub mark: f64,
    /// The second result of the user taken from the student.
    pub retake1: Option<f64>,
    /// The third result of the user taken from the student.
    pub retake2: Option<f64>,
}

/// The status of the module taken by the student.
///
/// The [`ModuleStatus`] colour code is as folows:
///
/// Orange (255, 255, 235, 156) => Component Fail (CF)
///
/// Green (255, 198, 235, 156) or (255, 198, 239, 206) => Soft Fail
/// (SF)
///
/// Red (255, 255, 199, 206) => Hard Fail (HF)
#[derive(Clone, Debug, Deserialize, PartialEq)]
pub enum ModuleStatus {
    /// The student passes the module (No Fill).
    Pass,
    /// The student soft-failed the module (Green).
    SoftFail,
    /// The student hard-failed the module (Red).
    HardFail,
    /// The student component-failed the module (Orange).
    ComponentFail,
}

impl Default for ModuleStatus {
    fn default() -> Self {
        Self::Pass
    }
}

/// Struct represting a result of a student in the raw data.
#[derive(Debug, Default, Deserialize)]
pub struct StudentResult {
    /// The entry number in the sheet.
    pub no: Option<i64>,
    /// The information about the student.
    pub student_info: StudentInfo,
    /// The year of studies of the student.
    pub year_of_program: String,
    /// The amount of credits taken by the student in the Autumn Semester.
    pub autumn_credit: Option<f64>,
    /// The average/mean marks of the student in the Autumn Semester.
    pub autumn_mean: Option<f64>,
    /// The amount of credits taken by the student in the entire year.
    pub full_credit: Option<f64>,
    /// The average/mean marks of the student in the entire year.
    pub full_mean: Option<f64>,
    /// The amount of credits taken by the student in the Spring Semester.
    pub spring_credit: Option<f64>,
    /// The amount of credits taken by the student in the Spring Semester.
    pub spring_mean: Option<f64>,
    /// The amount of credits taken by the student in the entire year.
    pub year_credit: Option<f64>,
    /// The average/mean marks of the student in the entire year.
    pub year_prog_average: Option<f64>,
    /// The number of credits that has mark <30
    pub credits_l3_lt30: Option<f64>,
    /// The number of credits that has mark between 30-39.
    pub credits_l3_30_39: Option<f64>,
    /// The number of credits that has mark <40
    pub credits_l4_lt40: Option<f64>,
    /// The number of credits that has mark between 40-49.
    pub credits_l4_40_49: Option<f64>,
    /// The progression status of the student, e.g. requires retake.
    pub progression: String,
    /// All the marks of the modules taken by the student.
    pub modules: Vec<Mark>,
    /// Remarks regardding the students result.
    pub remarks: Option<String>,
}

impl StudentResult {
    /// Create a new [`StudentResult`] object.
    pub fn new() -> Self {
        Self::default()
    }
}
