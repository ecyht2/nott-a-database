//! Parser for student marks data.
use std::{path::Path, str::FromStr};

use calamine::{open_workbook, Data, DataType, Range, Reader, Xlsx};

use crate::{
    errors::{InvalidHeader, ParseResultError},
    spreadsheet_ml::{get_data, Relationships, SheetRow, Styles, Workbook, Worksheet, XlsxColumns},
    ColourValue, Mark, ModuleStatus, StudentResult,
};

/// All the possible header column possible for [`Mark`] data.
#[derive(Debug)]
pub enum ResultHeaders {
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
    /// The amount of credits taken by the student in the entire year.
    YearCredit,
    /// The average/mean marks of the student in the entire year.
    YearProgAverage,
    /// Credits (L3) <30
    CreditsL3Lt30,
    /// Credits (L3) 30-39
    CreditsL33039,
    /// Credits (L4) <40
    CreditsL4Lt40,
    /// Credits (L4) 40-49
    CreditsL44049,
    /// The progression status of the student, e.g. requires retake.
    Progression,
    /// All the marks of the modules taken by the student.
    Modules,
    /// Remarks regardding the students result.
    Remarks,
}

impl ResultHeaders {
    /// Gets the header vector from the headers and sub-headers.
    pub fn get_headers(
        headers: &[String],
        sub_headers: &[String],
    ) -> Result<Vec<ResultHeaders>, InvalidHeader> {
        /// All the possible status when parsing the headers of the raw data.
        ///
        /// The status determine what the next header should be based on the sub-headers.
        #[derive(Debug)]
        enum HeaderStatus {
            /// No special treatment needed for the next header.
            Continue,
            /// Concatenate the current and next header with the sub-header.
            ConcatSubheader,
            /// Treat the next header as the "Module" header.
            Module,
        }

        // Creating Headers
        let mut status = (HeaderStatus::Continue, String::new());
        let mut output = vec![];
        for (header, sub_header) in headers.iter().zip(sub_headers.iter()) {
            let sub_header = sub_header
                // Convert to Lowercase
                .to_lowercase()
                // Fix "<" Credits (L3) and Credits (L4)
                .replace("<", "lt")
                // Fix "-" Credits (L3) and Credits (L4)
                .replace("-", "_")
                // Replace Space with _
                .replace(" ", "_");

            // Convert to Lowercase
            let mut header = header
                .to_lowercase()
                // Fix Year of Program
                .replace("\r\n", "")
                // Fix Credits (L3) and Credits (L4)
                .replace("(", "")
                .replace(")", "")
                // Replace Space with _
                .replace(" ", "_");

            match status.0 {
                HeaderStatus::Continue => {
                    if header == "autumn"
                        || header == "full"
                        || header == "spring"
                        || header == "year"
                        || header == "credits_l3"
                        || header == "credits_l4"
                    {
                        status.0 = HeaderStatus::ConcatSubheader;
                        status.1 = header.to_owned();
                        header += "_";
                        header += &sub_header;
                    } else if header == "modules" {
                        status.0 = HeaderStatus::Module;
                        status.1 = header.to_owned();
                    }
                }
                HeaderStatus::ConcatSubheader => {
                    header = status.1.to_owned();
                    header += "_";
                    header += &sub_header;
                    status.0 = HeaderStatus::Continue;
                    status.1 = String::new();
                }
                HeaderStatus::Module => {
                    if !header.is_empty() {
                        status.0 = HeaderStatus::Continue;
                        status.1 = String::new();
                        continue;
                    }
                    header = status.1.to_owned();
                }
            }
            output.push(self::ResultHeaders::from_str(&header)?);
        }

        Ok(output)
    }
}

impl FromStr for ResultHeaders {
    type Err = InvalidHeader;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        match s {
            "no" => Ok(Self::No),
            "id" => Ok(Self::Id),
            "last_name" => Ok(Self::LastName),
            "first_name" => Ok(Self::FirstName),
            "plan" => Ok(Self::Plan),
            "year_of_program" => Ok(Self::YearOfProgram),
            "autumn_credit" => Ok(Self::AutumnCredit),
            "autumn_mean" => Ok(Self::AutumnMean),
            "full_credit" => Ok(Self::FullCredit),
            "full_mean" => Ok(Self::FullMean),
            "spring_credit" => Ok(Self::SpringCredit),
            "spring_mean" => Ok(Self::SpringMean),
            "year_credit" => Ok(Self::YearCredit),
            "year_prog_average" => Ok(Self::YearProgAverage),
            "credits_l3_lt30" => Ok(Self::CreditsL3Lt30),
            "credits_l3_30_39" => Ok(Self::CreditsL33039),
            "credits_l4_lt40" => Ok(Self::CreditsL4Lt40),
            "credits_l4_40_49" => Ok(Self::CreditsL44049),
            "progression" => Ok(Self::Progression),
            "modules" => Ok(Self::Modules),
            "remarks" => Ok(Self::Remarks),
            _ => Err(InvalidHeader(s.to_owned())),
        }
    }
}

impl StudentResult {
    /// Parse a row of data from result report (0A).
    pub fn from_result_row(
        headers: &[ResultHeaders],
        row: &[Data],
        row_no: usize,
        styles: &Styles,
        row_data: &SheetRow,
    ) -> Result<Self, ParseResultError> {
        let mut output = Self::new();

        for (col, (header, data)) in headers.iter().zip(row).enumerate() {
            match header {
                ResultHeaders::No => output.no = data.as_i64(),
                ResultHeaders::Id => {
                    output.student_info.id = data.as_i64().ok_or(ParseResultError::InvalidID)?
                }
                ResultHeaders::LastName => {
                    output.student_info.last_name =
                        data.as_string().ok_or(ParseResultError::InvalidLastName)?
                }
                ResultHeaders::FirstName => {
                    output.student_info.first_name =
                        data.as_string().ok_or(ParseResultError::InvalidFirstName)?
                }
                ResultHeaders::Plan => {
                    output.student_info.plan =
                        data.as_string().ok_or(ParseResultError::InvalidPlan)?
                }
                ResultHeaders::YearOfProgram => {
                    output.year_of_program = data
                        .as_string()
                        .ok_or(ParseResultError::InvalidYearOfProgram)?
                }
                ResultHeaders::AutumnCredit => output.autumn_credit = data.as_f64(),
                ResultHeaders::AutumnMean => output.autumn_mean = data.as_f64(),
                ResultHeaders::SpringCredit => output.spring_credit = data.as_f64(),
                ResultHeaders::SpringMean => output.spring_mean = data.as_f64(),
                ResultHeaders::FullCredit => output.full_credit = data.as_f64(),
                ResultHeaders::FullMean => output.full_mean = data.as_f64(),
                ResultHeaders::YearCredit => output.year_credit = data.as_f64(),
                ResultHeaders::YearProgAverage => output.year_prog_average = data.as_f64(),
                ResultHeaders::CreditsL3Lt30 => output.credits_l3_lt30 = data.as_f64(),
                ResultHeaders::CreditsL33039 => output.credits_l3_30_39 = data.as_f64(),
                ResultHeaders::CreditsL4Lt40 => output.credits_l4_lt40 = data.as_f64(),
                ResultHeaders::CreditsL44049 => output.credits_l4_40_49 = data.as_f64(),
                ResultHeaders::Progression => {
                    output.progression = data
                        .as_string()
                        .ok_or(ParseResultError::InvalidProgression)?
                }
                ResultHeaders::Modules => {
                    if data.is_empty() {
                        continue;
                    }
                    let tmp = data.as_string().ok_or(ParseResultError::InvalidModule)?;
                    let mut tmp = Mark::from_str(&tmp)?;

                    let col_name = XlsxColumns::new()
                        .nth(col)
                        .expect("There should be an infinite amount of XLSX columns.");
                    let cell = col_name + &row_no.to_string();
                    let cell = row_data
                        .cells
                        .iter()
                        .find(|c| c.cell == cell)
                        .expect("There should be a cell found in row data.");
                    let style_id: usize = cell
                        .style
                        .parse()
                        .map_err(|_| ParseResultError::InvalidModule)?;
                    let fill_id = styles.cell_xfs.xf[style_id].fill_id;
                    let fill = &styles.fills.fill[fill_id];

                    if let Some(colour) = &fill.pattern_fill.fg_color {
                        tmp.status = ModuleStatus::try_from(&colour.rgb)?;
                        tmp.fill = Some(colour.rgb.clone());
                    }
                    output.modules.push(tmp);
                }
                ResultHeaders::Remarks => output.remarks = data.as_string(),
            }
        }

        Ok(output)
    }

    /// Parse a worksheet in from result report (0A).
    pub fn from_result_worksheet<P: AsRef<Path>>(
        name: &str,
        range: Range<Data>,
        file: P,
        workbook: &Workbook,
        relationship: &Relationships,
        styles: &Styles,
    ) -> Result<Vec<StudentResult>, Box<dyn std::error::Error>> {
        // Extract worksheet and relationship metadata
        let worksheet = workbook
            .sheets
            .sheet
            .iter()
            .find(|x| x.name == name)
            .expect("The parsed workbook XML should have the sheet.");
        let sheet_file = &relationship
            .relationship
            .iter()
            .find(|x| x.id == worksheet.rid)
            .expect("The parsed relationship XML should have the relationship.")
            .path;

        // Extract raw worksheet data
        let worksheet_path = if sheet_file.starts_with("../") {
            Path::new(
                sheet_file
                    .strip_prefix("../")
                    .expect("Path should have \"../\" prefix"),
            )
        } else if sheet_file.starts_with("/") {
            Path::new(
                sheet_file
                    .strip_prefix("/")
                    .expect("Path should have \"/\" prefix"),
            )
        } else {
            &Path::new("xl/").join(sheet_file)
        };

        let sheet: Worksheet = get_data(
            &file,
            worksheet_path
                .to_str()
                .expect("Invalid path in Workbook archive."),
        )?;

        // Getting Headers and Subheaders
        let headers = range
            .headers()
            .ok_or("Invalid workbook given, the first row of data must be the headers")?;
        let sub_headers = range
            .range((1, 0), range.end().unwrap())
            .headers()
            .ok_or("Invalid workbook given, the second row of data must be the sub-headers")?;
        let headers = ResultHeaders::get_headers(&headers, &sub_headers)?;

        let data: Vec<StudentResult> = range
            .rows()
            .enumerate()
            .skip(2)
            .map(|(row_no, row)| {
                StudentResult::from_result_row(
                    &headers,
                    row,
                    row_no + 1,
                    styles,
                    &sheet.sheet_data.row[row_no],
                )
            })
            .collect::<Result<_, ParseResultError>>()?;

        Ok(data)
    }

    /// Extract all the student from a result report (0A) workbook.
    pub fn from_result<P: AsRef<Path>>(
        file: P,
    ) -> Result<Vec<StudentResult>, Box<dyn std::error::Error>> {
        let mut excel: Xlsx<_> = open_workbook(&file).map_err(|_| "Unable to find workbook")?;

        let styles: Styles = get_data(&file, "xl/styles.xml")?;
        let workbook: Workbook = get_data(&file, "xl/workbook.xml")?;
        let relationship: Relationships = get_data(&file, "xl/_rels/workbook.xml.rels")?;

        let mut data = vec![];
        for (name, sheet) in excel.worksheets() {
            let mut sheet_data = Self::from_result_worksheet(
                &name,
                sheet,
                &file,
                &workbook,
                &relationship,
                &styles,
            )?;
            data.append(&mut sheet_data);
        }

        Ok(data)
    }
}

impl FromStr for Mark {
    type Err = ParseResultError;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let data: Vec<&str> = s.split("\r\n").filter(|s| !s.is_empty()).collect();

        if data.len() == 3 {
            let code = data[0].to_owned() + data[1];
            let credit = 10;
            let mark = f64::from_str(data[2]).map_err(|_| ParseResultError::InvalidModule)?;
            Ok(Mark {
                code,
                credit,
                mark,
                ..Default::default()
            })
        } else if data.len() == 4 {
            let code = data[0].to_owned() + data[1];
            let credit = i64::from_str(data[2]).map_err(|_| ParseResultError::InvalidModule)?;
            let mark = f64::from_str(data[3]).map_err(|_| ParseResultError::InvalidModule)?;
            Ok(Mark {
                code,
                credit,
                mark,
                ..Default::default()
            })
        } else {
            return Err(ParseResultError::InvalidModule);
        }
    }
}

impl TryFrom<ColourValue> for ModuleStatus {
    type Error = ParseResultError;

    /// Get the [`ModuleStatus`] base on the fill colour of the cell.
    ///
    /// The [`ModuleStatus`] colour code is as folows:
    fn try_from(value: ColourValue) -> Result<Self, Self::Error> {
        Self::try_from(&value)
    }
}

impl TryFrom<&ColourValue> for ModuleStatus {
    type Error = ParseResultError;

    /// Get the [`ModuleStatus`] base on the fill colour of the cell.
    ///
    /// The [`ModuleStatus`] colour code is as folows:
    ///
    /// Orange (255, 255, 235, 156) => Component Fail (CF)
    ///
    /// Green (255, 198, 235, 156) or (255, 198, 239, 206) => Soft Fail
    /// (SF)
    ///
    /// Red (255, 255, 199, 206) => Hard Fail (HF)
    fn try_from(value: &ColourValue) -> Result<Self, Self::Error> {
        if value.alpha != 255 {
            return Err(ParseResultError::InvalidModule);
        }

        if value.red == 255 && value.green == 235 && value.blue == 156 {
            // Orange Cell => Component Fail (CF)
            Ok(Self::ComponentFail)
        } else if value.red == 198
            && (value.green == 235 || value.green == 239)
            && (value.blue == 156 || value.blue == 206)
        {
            // Green Cell => Soft Fail (SF)
            Ok(Self::SoftFail)
        } else if value.red == 255 && value.green == 199 && value.blue == 206 {
            // Red Cell => Hard Fail (HF)
            Ok(Self::HardFail)
        } else {
            Err(ParseResultError::InvalidModule)
        }
    }
}
