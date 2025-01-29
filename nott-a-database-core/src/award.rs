//! Parser implementation of award report (0B) raw data.
use std::{path::Path, str::FromStr};

use calamine::{open_workbook, Data, DataType, Reader, Xlsx};

use crate::{
    errors::{ParseAwardError, ParseAwardRowError},
    StudentInfo,
};

/// The header columns in award report (0B) raw data.
#[derive(Debug)]
pub enum AwardHeader {
    /// The row number of the data.
    No,
    /// The student ID of the entry.
    Id,
    /// The last name of the student.
    LastName,
    /// The first name of the student.
    FirstName,
    /// The career number of the student.
    CareerNumber,
    /// The academice program of the student.
    AcademicProgram,
    /// The description of the program studied.
    ProgramDescription,
    /// The academic plan of the student.
    AcademicPlan,
    /// The description of the plan studied.
    PlanDescription,
    /// The intake year of the student.
    Intake,
    /// The QAA Effective Data of the student.
    QAAEffectiveDate,
    /// The Degree Calculation Model of the student.
    DegreeCalculationModel,
    /// The final raw mark from the student's result.
    RawFinalMark,
    /// The final mark from the student's result after truncating percision
    /// from the raw mark.
    TruncatedFinalMark,
    /// The final mark from the student's result after all processing.
    FinalMark,
    /// The borderline status of the student.
    Borderline,
    /// The Calculation Review Rqd column of the student.
    CalculationReviewRqd,
    /// The Degree Award column of the student.
    DegreeAward,
    /// The Selected column of the student.
    Selected,
    /// The Exception Data column of the student.
    ExceptionData,
    /// An empty column used for spacing.
    Empty,
    /// The recommended action taken for the student.
    Recommendation,
}

impl FromStr for AwardHeader {
    type Err = ParseAwardError;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        Ok(match s {
            "No" => Self::No,
            "Student ID" => Self::Id,
            "Surname" => Self::LastName,
            "First Name" => Self::FirstName,
            "Career Number" => Self::CareerNumber,
            "Academic Program" => Self::AcademicProgram,
            "Program Description" => Self::ProgramDescription,
            "Academic Plan" => Self::AcademicPlan,
            "Plan Description" => Self::PlanDescription,
            "Intake" => Self::Intake,
            "QAA Effective Date" => Self::QAAEffectiveDate,
            "Degree Calculation Model" => Self::DegreeCalculationModel,
            "Raw Final Mark" => Self::RawFinalMark,
            "Truncated Final Mark" => Self::TruncatedFinalMark,
            "Final Mark" => Self::FinalMark,
            "Borderline?" => Self::Borderline,
            "Calculation Review Rqd" => Self::CalculationReviewRqd,
            "Degree Award" => Self::DegreeAward,
            "Selected" => Self::Selected,
            "Exception Data" => Self::ExceptionData,
            "" => Self::Empty,
            "Recommendation" => Self::Recommendation,
            _ => {
                return Err(ParseAwardError::InvalidHeader(
                    "Invalid award header".into(),
                ))
            }
        })
    }
}

impl StudentInfo {
    /// Creates [`StudentInfo`] from a row of award report (0B) raw data.
    pub fn from_award_row(
        data: &[Data],
        headers: &[AwardHeader],
    ) -> Result<Self, ParseAwardRowError> {
        let mut output = Self::new();

        for (header, data) in headers.iter().zip(data) {
            match header {
                AwardHeader::No => continue,
                AwardHeader::Id => {
                    output.id = data.as_i64().ok_or(ParseAwardRowError::InvalidId)?
                }
                AwardHeader::LastName => {
                    output.last_name = data
                        .as_string()
                        .ok_or(ParseAwardRowError::InvalidLastName)?
                }
                AwardHeader::FirstName => {
                    output.first_name = data
                        .as_string()
                        .ok_or(ParseAwardRowError::InvalidFirstName)?
                }
                AwardHeader::CareerNumber => {
                    output.carrer_number = Some(
                        data.as_i64()
                            .ok_or(ParseAwardRowError::InvalidCareerNumber)?,
                    )
                }
                AwardHeader::AcademicProgram => {
                    output.academic_program = Some(
                        data.as_string()
                            .ok_or(ParseAwardRowError::InvalidAcademicProgram)?,
                    )
                }
                AwardHeader::ProgramDescription => {
                    output.program_description = Some(
                        data.as_string()
                            .ok_or(ParseAwardRowError::InvalidProgramDescription)?,
                    )
                }
                AwardHeader::AcademicPlan => {
                    output.plan = data
                        .as_string()
                        .ok_or(ParseAwardRowError::InvalidAcademicPlan)?
                }
                AwardHeader::PlanDescription => {
                    output.plan_description = Some(
                        data.as_string()
                            .ok_or(ParseAwardRowError::InvalidPlanDescription)?,
                    )
                }
                AwardHeader::Intake => {
                    output.intake = Some(data.as_string().ok_or(ParseAwardRowError::InvalidIntake)?)
                }
                AwardHeader::QAAEffectiveDate => {
                    output.qaa_effective_date = Some(
                        data.as_datetime()
                            .ok_or(ParseAwardRowError::InvalidQAAEffectiveDate)?,
                    )
                }
                AwardHeader::DegreeCalculationModel => {
                    output.calculation_model = Some(
                        data.as_string()
                            .ok_or(ParseAwardRowError::InvalidDegreeCalculationModel)?,
                    )
                }
                AwardHeader::RawFinalMark => {
                    output.raw_mark = Some(
                        data.as_f64()
                            .ok_or(ParseAwardRowError::InvalidRawFinalMark)?,
                    )
                }
                AwardHeader::TruncatedFinalMark => {
                    output.truncated_mark = Some(
                        data.as_f64()
                            .ok_or(ParseAwardRowError::InvalidTruncatedFinalMark)?,
                    )
                }
                AwardHeader::FinalMark => {
                    output.final_mark =
                        Some(data.as_i64().ok_or(ParseAwardRowError::InvalidFinalMark)?)
                }
                AwardHeader::Borderline => {
                    output.borderline = Some(
                        data.as_string()
                            .ok_or(ParseAwardRowError::InvalidBorderline)?,
                    )
                }
                AwardHeader::CalculationReviewRqd => {
                    let data = data
                        .as_string()
                        .ok_or(ParseAwardRowError::InvalidCalculationReviewRqd)?;
                    output.calculation = match data.as_str() {
                        "Y" => Some(true),
                        "N" => Some(false),
                        _ => return Err(ParseAwardRowError::InvalidSelected),
                    };
                }
                AwardHeader::DegreeAward => {
                    if DataType::is_empty(data) {
                        continue;
                    }

                    output.degree_award = match data.as_string() {
                        Some(e) => Some(e),
                        None => {
                            let e = data
                                .as_time()
                                .ok_or(ParseAwardRowError::InvalidDegreeAward)?;
                            Some(e.format("%H:%M").to_string())
                        }
                    };
                }
                AwardHeader::Selected => {
                    let data = data
                        .as_string()
                        .ok_or(ParseAwardRowError::InvalidSelected)?;
                    output.selected = match data.as_str() {
                        "Y" => Some(true),
                        "N" => Some(false),
                        _ => return Err(ParseAwardRowError::InvalidSelected),
                    }
                }
                AwardHeader::ExceptionData => {
                    if DataType::is_empty(data) {
                        continue;
                    }

                    output.exception_data = Some(
                        data.as_string()
                            .ok_or(ParseAwardRowError::InvalidExceptionData)?,
                    );
                }
                AwardHeader::Empty => continue,
                AwardHeader::Recommendation => {
                    output.recommendation = Some(
                        data.as_string()
                            .ok_or(ParseAwardRowError::InvalidRecommendation)?,
                    )
                }
            }
        }

        Ok(output)
    }

    /// Creates [`StudentInfo`] from award report (0B) raw data.
    pub fn from_award<P: AsRef<Path>>(file: P) -> Result<Vec<Self>, ParseAwardError> {
        let mut excel: Xlsx<_> = open_workbook(&file).map_err(ParseAwardError::WorkbookError)?;

        let award = excel
            .worksheet_range("Award Report")
            .map_err(ParseAwardError::InvalidWorksheet)?;

        let headers: Vec<AwardHeader> = award
            .headers()
            .ok_or(ParseAwardError::NoHeaders)?
            .iter()
            .map(String::as_str)
            .map(AwardHeader::from_str)
            .collect::<Result<_, ParseAwardError>>()?;

        let mut data = vec![];
        for (row_no, row) in award.rows().enumerate().skip(1) {
            let row_data = Self::from_award_row(row, &headers)
                .map_err(|err| ParseAwardError::InvalidRow(row_no, err))?;
            data.push(row_data);
        }

        Ok(data)
    }
}
