//! Parser for August resit report (0D) raw data.
use std::{collections::VecDeque, path::Path, str::FromStr};

use calamine::{open_workbook, Data, DataType, Reader, Xlsx};

use crate::{
    errors::{ParseAugResitError, ParseAugResitRowError},
    Mark, StudentResult,
};

/// Headers for August resit report (0D) raw data.
#[derive(Debug)]
pub enum AugResitHeader {
    /// The entry number in the sheet.
    No,
    /// The student ID of the student.
    Id,
    /// The last name of the student.
    LastName,
    /// The last name of the student.
    FirstName,
    /// The first name of the student.
    Plan,
    /// The progression status of the student, e.g. requires retake.
    YearOfProgram,
    /// The amount of credits taken by the student in the Autumn Semester.
    AutumnCredit,
    /// The average/mean marks of the student in the Autumn Semester.
    AutumnMean,
    /// The amount of credits taken by the student in the Spring Semester.
    FullCredit,
    /// The amount of credits taken by the student in the Spring Semester.
    FullMean,
    /// The amount of credits taken by the student in the entire year.
    SpringCredit,
    /// The average/mean marks of the student in the entire year.
    SpringMean,
    /// An empy column.
    Empty,
    /// The result headers.
    ResultHeader,
    /// The amount of credits taken by the student in the entire year.
    YearCredit,
    /// The average/mean marks of the student in the entire year.
    YearProgAverage,
    /// Credits (L3) <30
    CreditsL3Lt30,
    /// Credits (L3) 30-39
    CreditsL33039,
    /// The progression status of the student, e.g. requires retake.
    Progression,
    /// All the marks of the modules taken by the student.
    Course,
    /// Remarks regardding the students result.
    Remarks,
}

impl AugResitHeader {
    /// Gets the headers of a August resit report (0D) raw data worksheet headers
    /// and subheaders.
    pub fn from_sheet_headers(
        headers: &[String],
        sub_headers: &[String],
    ) -> Result<Vec<AugResitHeader>, ParseAugResitError> {
        let mut output = vec![];

        enum State {
            Copy,
            Course,
            None,
        }

        let mut state = State::None;
        let mut previous = String::new();
        for (header, sub_header) in headers.iter().zip(sub_headers) {
            let header = header.replace("\r\n", " ");
            let sub_header = sub_header.replace("\r\n", " ");

            let header = if header.is_empty() {
                match state {
                    State::Copy => {
                        state = State::None;
                        previous
                    }
                    State::Course => previous,
                    State::None => String::from("Empty"),
                }
            } else {
                if header == "Course" {
                    state = State::Course;
                } else {
                    state = State::Copy;
                }
                header
            };

            let combined = if sub_header.is_empty() {
                header.to_owned()
            } else {
                format!("{} {}", header, sub_header)
            };

            previous = header;
            output.push(combined.parse()?);
        }

        Ok(output)
    }
}

impl FromStr for AugResitHeader {
    type Err = ParseAugResitError;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        Ok(match s {
            "No" => Self::No,
            "ID" => Self::Id,
            "Last Name" => Self::LastName,
            "First Name" => Self::FirstName,
            "Plan" => Self::Plan,
            "Year Of Program" => Self::YearOfProgram,
            "Autumn Credit" => Self::AutumnCredit,
            "Autumn Mean" => Self::AutumnMean,
            "Full Credit" => Self::FullCredit,
            "Full Mean" => Self::FullMean,
            "Spring Credit" => Self::SpringCredit,
            "Spring Mean" => Self::SpringMean,
            "Empty Credit" => Self::Empty,
            "Empty Mean" => Self::ResultHeader,
            "Year Credit" => Self::YearCredit,
            "Year Prog Average" => Self::YearProgAverage,
            "Credits <30" => Self::CreditsL3Lt30,
            "Credits 30-39" => Self::CreditsL33039,
            "Progression" => Self::Progression,
            "Course" => Self::Course,
            "Remarks" => Self::Remarks,
            _ => return Err(ParseAugResitError::InvalidHeaders),
        })
    }
}

impl StudentResult {
    /// Parse [`StudentResult`] from a row of August resit report (0D) raw data.
    fn from_resit_aug_row(
        headers: &[AugResitHeader],
        data: &[Data],
    ) -> Result<StudentResult, ParseAugResitRowError> {
        let mut output = Self::new();

        // Filtering out weird character "_x000D_"
        for (header, value) in headers.iter().zip(data) {
            let value = if value.is_string() {
                &Data::String(
                    value
                        .as_string()
                        .expect("The value should be a string after checking")
                        .split("\r\n")
                        .filter(|s| *s != "_x000D_")
                        .collect::<Vec<&str>>()
                        .join("\r\n")
                        .to_string(),
                )
            } else {
                value
            };

            match header {
                AugResitHeader::No => {
                    output.no = Some(value.as_i64().ok_or(ParseAugResitRowError::InvalidID)?)
                }
                AugResitHeader::Id => {
                    output.student_info.id =
                        value.as_i64().ok_or(ParseAugResitRowError::InvalidID)?
                }
                AugResitHeader::LastName => {
                    output.student_info.last_name = value
                        .as_string()
                        .ok_or(ParseAugResitRowError::InvalidLastName)?
                }
                AugResitHeader::FirstName => {
                    output.student_info.first_name = value
                        .as_string()
                        .ok_or(ParseAugResitRowError::InvalidFirstName)?
                }
                AugResitHeader::Plan => {
                    output.student_info.plan = value
                        .as_string()
                        .ok_or(ParseAugResitRowError::InvalidFirstName)?
                }
                AugResitHeader::YearOfProgram => {
                    output.year_of_program = value
                        .as_string()
                        .ok_or(ParseAugResitRowError::InvalidYearOfProgram)?
                }
                AugResitHeader::AutumnCredit => {
                    output.autumn_credit = if value.is_empty() {
                        None
                    } else {
                        Some(
                            value
                                .as_f64()
                                .ok_or(ParseAugResitRowError::InvalidAutumnCredit)?,
                        )
                    }
                }
                AugResitHeader::AutumnMean => {
                    output.autumn_mean = if value.is_empty() {
                        None
                    } else {
                        Some(
                            value
                                .as_f64()
                                .ok_or(ParseAugResitRowError::InvalidAutumnMean)?,
                        )
                    }
                }
                AugResitHeader::FullCredit => {
                    output.full_credit = if value.is_empty() {
                        None
                    } else {
                        Some(
                            value
                                .as_f64()
                                .ok_or(ParseAugResitRowError::InvalidFullCredit)?,
                        )
                    }
                }
                AugResitHeader::FullMean => {
                    output.full_mean = if value.is_empty() {
                        None
                    } else {
                        Some(
                            value
                                .as_f64()
                                .ok_or(ParseAugResitRowError::InvalidFullMean)?,
                        )
                    }
                }
                AugResitHeader::SpringCredit => {
                    output.spring_credit = if value.is_empty() {
                        None
                    } else {
                        Some(
                            value
                                .as_f64()
                                .ok_or(ParseAugResitRowError::InvalidSpringCredit)?,
                        )
                    }
                }
                AugResitHeader::SpringMean => {
                    output.spring_mean = if value.is_empty() {
                        None
                    } else {
                        Some(
                            value
                                .as_f64()
                                .ok_or(ParseAugResitRowError::InvalidSpringMean)?,
                        )
                    }
                }
                AugResitHeader::Empty => continue,
                AugResitHeader::ResultHeader => continue,
                AugResitHeader::YearCredit => {
                    output.year_credit = if value.is_empty() {
                        None
                    } else {
                        // Taking newest (last) value
                        let value: Vec<f64> = value
                            .as_string()
                            .ok_or(ParseAugResitRowError::InvalidYearCredit)?
                            .split("\r\n")
                            .filter(|s| !s.is_empty())
                            .map(|s| {
                                s.parse()
                                    .map_err(|_| ParseAugResitRowError::InvalidYearCredit)
                            })
                            .collect::<Result<_, ParseAugResitRowError>>()?;
                        Some(
                            *value
                                .last()
                                .ok_or(ParseAugResitRowError::InvalidYearCredit)?,
                        )
                    }
                }
                AugResitHeader::YearProgAverage => {
                    output.year_prog_average = if value.is_empty() {
                        None
                    } else {
                        // Taking newest (last) value
                        let value: Vec<f64> = value
                            .as_string()
                            .ok_or(ParseAugResitRowError::InvalidYearProgAverage)?
                            .split("\r\n")
                            .filter(|s| !s.is_empty())
                            .map(|s| {
                                s.parse()
                                    .map_err(|_| ParseAugResitRowError::InvalidYearProgAverage)
                            })
                            .collect::<Result<_, ParseAugResitRowError>>()?;
                        Some(
                            *value
                                .last()
                                .ok_or(ParseAugResitRowError::InvalidYearProgAverage)?,
                        )
                    }
                }
                AugResitHeader::CreditsL3Lt30 => {
                    output.credits_l3_lt30 = if value.is_empty() {
                        None
                    } else {
                        // Taking newest (last) value
                        let value: Vec<f64> = value
                            .as_string()
                            .ok_or(ParseAugResitRowError::InvalidCreditsL3Lt30)?
                            .split("\r\n")
                            .filter(|s| !s.is_empty())
                            .map(|s| {
                                s.parse()
                                    .map_err(|_| ParseAugResitRowError::InvalidCreditsL3Lt30)
                            })
                            .collect::<Result<_, ParseAugResitRowError>>()?;
                        Some(
                            *value
                                .last()
                                .ok_or(ParseAugResitRowError::InvalidCreditsL3Lt30)?,
                        )
                    }
                }
                AugResitHeader::CreditsL33039 => {
                    output.credits_l3_30_39 = if value.is_empty() {
                        None
                    } else {
                        let value: Vec<f64> = value
                            .as_string()
                            .ok_or(ParseAugResitRowError::InvalidCreditsL33039)?
                            .split("\r\n")
                            .filter(|s| !s.is_empty())
                            .map(|s| {
                                s.parse()
                                    .map_err(|_| ParseAugResitRowError::InvalidCreditsL33039)
                            })
                            .collect::<Result<_, ParseAugResitRowError>>()?;
                        // Taking newest (last) value
                        Some(
                            *value
                                .last()
                                .ok_or(ParseAugResitRowError::InvalidCreditsL33039)?,
                        )
                    }
                }
                AugResitHeader::Progression => {
                    output.progression = value
                        .as_string()
                        .ok_or(ParseAugResitRowError::InvalidProgression)?;
                }
                AugResitHeader::Course => {
                    // Skipping Empty course
                    if value.is_empty() {
                        continue;
                    }

                    // Initialize Mark
                    let mut mark = Mark::default();
                    let value = value
                        .as_string()
                        .ok_or(ParseAugResitRowError::InvalidCourse)?;

                    if value.contains("\x03") {
                        // Multi-row data
                        let mut value: VecDeque<&str> = value.split("\x03").collect();
                        let mut rest = value.split_off(1);
                        let mut module_info: Vec<&str> =
                            value[0].split("\r\n").filter(|s| !s.is_empty()).collect();

                        // Extract module code and credits
                        if module_info.len() == 3 {
                            let credits = module_info.split_off(2)[0];
                            mark.code = module_info.join("").trim().to_owned();
                            mark.credit = credits
                                .trim()
                                .parse()
                                .map_err(|_| ParseAugResitRowError::InvalidCourse)?;
                        } else if module_info.len() == 2 {
                            mark.code = module_info.join("");
                            mark.credit = 10;
                        } else {
                            return Err(ParseAugResitRowError::InvalidCourse);
                        }

                        // Extracting marks
                        mark.mark = rest
                            .pop_front()
                            .ok_or(ParseAugResitRowError::InvalidCourse)?
                            .trim()
                            .parse()
                            .map_err(|_| ParseAugResitRowError::InvalidCourse)?;

                        // Extracting retakes
                        if !rest.is_empty() {
                            mark.retake1 = rest
                                .pop_front()
                                .expect("There should be one more elements")
                                .trim()
                                .parse()
                                .ok();
                        }
                    } else {
                        // Single row data
                        let mut value: Vec<&str> = value.split("\r\n").collect();
                        let mut rest = value.split_off(2);

                        // Extracting module code and credits
                        mark.code = value.join("").trim().to_owned();
                        mark.credit = if rest.len() == 1 {
                            10
                        } else {
                            let tmp = rest.split_off(2);
                            let credits = rest[1].trim();
                            rest = tmp;
                            if credits.is_empty() {
                                10
                            } else {
                                credits
                                    .parse()
                                    .map_err(|_| ParseAugResitRowError::InvalidCourse)?
                            }
                        };

                        // Extracting marks
                        rest.retain(|s| !s.is_empty());
                        let mut rest: VecDeque<_> = rest.into();
                        mark.mark = rest
                            .pop_front()
                            .ok_or(ParseAugResitRowError::InvalidCourse)?
                            .trim()
                            .parse()
                            .map_err(|_| ParseAugResitRowError::InvalidCourse)?;

                        // Extracting retakes
                        if !rest.is_empty() {
                            mark.retake1 = rest
                                .pop_front()
                                .expect("There should be one more elements")
                                .trim()
                                .parse()
                                .ok();
                        }
                    }
                    output.modules.push(mark);
                }
                AugResitHeader::Remarks => {
                    output.remarks = if value.is_empty() {
                        None
                    } else {
                        Some(
                            value
                                .as_string()
                                .ok_or(ParseAugResitRowError::InvalidRemarks)?,
                        )
                    }
                }
            }
        }

        Ok(output)
    }

    /// Parse [`StudentResult`] from a August resit report (0D) raw data.
    pub fn from_resit_aug<P: AsRef<Path>>(
        data: P,
    ) -> Result<Vec<StudentResult>, ParseAugResitError> {
        let mut output = vec![];

        // Checking workbook
        let mut excel: Xlsx<_> = open_workbook(data).map_err(ParseAugResitError::WorkbookError)?;
        let mut worksheets = excel.worksheets();

        // Getting worksheet
        let (_, range) = worksheets
            .pop()
            .expect("There should be one worksheet in resit workbook.");

        // Getting Headers
        let headers = range.headers().ok_or(ParseAugResitError::InvalidHeaders)?;
        let sub_headers = range
            .range((1, 0), range.end().ok_or(ParseAugResitError::NoSubheader)?)
            .headers()
            .ok_or(ParseAugResitError::NoSubheader)?;
        let headers = AugResitHeader::from_sheet_headers(&headers, &sub_headers)?;

        // Merging multi-row data
        let mut current = vec![];
        let mut new_data = vec![];
        for (row, data) in range.rows().enumerate().skip(2) {
            if !data
                .get(1)
                .ok_or(ParseAugResitError::InvalidDataRow(
                    row + 1,
                    ParseAugResitRowError::InvalidID,
                ))?
                .is_empty()
            {
                // Adding merged row to list
                if !current.is_empty() {
                    new_data.push(current.clone());
                }
                current = data.to_vec();
            } else {
                // Combining data if the ID row is empty
                current = current
                    .into_iter()
                    .zip(data)
                    .map(|(current, data)| match (&current, data) {
                        // Merging empty
                        (Data::Empty, d) => d.clone(),
                        // Merging with string
                        (Data::String(s1), Data::String(s2)) => {
                            Data::String(s1.to_owned() + "\x03" + s2)
                        }
                        // Merging with float
                        (Data::String(s1), Data::Float(s2)) => {
                            Data::String(s1.to_owned() + "\x03" + &s2.to_string())
                        }
                        _ => current,
                    })
                    .collect();
            }
        }
        new_data.push(current);

        // Parsing data
        for (row, data) in new_data.iter().enumerate() {
            let row_data = Self::from_resit_aug_row(&headers, data)
                .map_err(|e| ParseAugResitError::InvalidDataRow(row + 1, e))?;
            output.push(row_data);
        }

        Ok(output)
    }
}
