//! Custom data types for SpreadsheetML in spec.
//!
//! This module only implement a small part of the Office Open XML document for
//! SpreadsheetML. Only a small part of the [`Styles Part`](Styles),
//! [`Archive Relationships`](Relationships), [`Worksheet Part`](Sheets), and
//! [`Workbook Part`](Workbook). See the
//! [spec](https://www.iso.org/standard/71691.html) for more information.
use std::{fmt::Debug, fs::File, io::Read, iter::Cloned, path::Path, slice::Iter};

use quick_xml::de::from_str;
use serde::{de::Visitor, Deserialize};
use zip::ZipArchive;

use crate::ColourValue;

/// The `Styles Part` in the workbook.
#[derive(Debug, Deserialize)]
pub struct Styles {
    /// The styles for the fill of a cell.
    pub fills: Fills,
    /// The styles of each cell.
    #[serde(rename = "cellXfs")]
    pub cell_xfs: CellXf,
}

/// The styles for the fill of a cell.
#[derive(Debug, Deserialize)]
pub struct Fills {
    /// All the different type of fills in the workbook.
    pub fill: Vec<Fill>,
}

/// A fill entry describing a fill for a cell.
#[derive(Debug, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct Fill {
    /// The fill information of the fill entry.
    pub pattern_fill: PatternFill,
}

/// A pattern fill for a cell.
#[derive(Debug, Deserialize)]
pub struct PatternFill {
    /// The type of pattern fill.
    #[serde(rename = "@patternType")]
    pub pattern_type: PatternType,
    /// The foreground colour of the fill.
    #[serde(rename = "fgColor")]
    pub fg_color: Option<FgColor>,
}

/// The different type of pattern fill for a cell.
#[derive(Debug, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum PatternType {
    /// The solid fill type.
    Solid,
    /// Other fill types possible for a cell.
    #[serde(other)]
    Other,
}

/// The foreground colour for a given pattern fill.
#[derive(Debug, Deserialize)]
pub struct FgColor {
    /// The ARGB value of the pattern fill.
    ///
    /// The fill contains an alpha (transparency), red colour intensity, green
    /// intensity, and the blue intensity.
    ///
    /// The value is a 8 digit hexadecimal number encoded as a string.
    #[serde(rename = "@rgb", deserialize_with = "deserialize_colour_value")]
    pub rgb: ColourValue,
}

fn deserialize_colour_value<'de, D>(deserializer: D) -> Result<ColourValue, D::Error>
where
    D: serde::Deserializer<'de>,
{
    deserializer.deserialize_str(HexVisitor)
}

/// Custom visitor for parsing [`ColourValue`] from an 8 digit hexadecimal string value.
struct HexVisitor;

impl<'de> Visitor<'de> for HexVisitor {
    type Value = ColourValue;

    fn expecting(&self, formatter: &mut std::fmt::Formatter) -> std::fmt::Result {
        formatter.write_str("a 8 digit hexadecimal number")
    }

    fn visit_str<E>(self, v: &str) -> Result<Self::Value, E>
    where
        E: serde::de::Error,
    {
        if v.len() != 8 {
            return Err(E::custom(format!(
                "The &str is {} characters long and not 8.",
                v.len()
            )));
        }
        Ok(Self::Value {
            alpha: u8::from_str_radix(&v[0..=1], 16)
                .map_err(|_| E::custom(format!("Invalide hex number {}", &v[0..=1])))?,
            red: u8::from_str_radix(&v[2..=3], 16)
                .map_err(|_| E::custom(format!("Invalide hex number {}", &v[2..=3])))?,
            green: u8::from_str_radix(&v[4..=5], 16)
                .map_err(|_| E::custom(format!("Invalide hex number {}", &v[4..=5])))?,
            blue: u8::from_str_radix(&v[6..=7], 16)
                .map_err(|_| E::custom(format!("Invalide hex number {}", &v[6..=7])))?,
        })
    }
}

/// The formatting of all cell styles.
#[derive(Debug, Deserialize)]
pub struct CellXf {
    /// All the cell formatting in the workbook.
    pub xf: Vec<Xf>,
}

/// Representation of the formatting applied to a cell.
#[derive(Debug, Deserialize)]
pub struct Xf {
    /// The fill ID of the cell.
    #[serde(rename = "@fillId")]
    pub fill_id: usize,
}

/// The `Workbook Part` of the workbook.
#[derive(Debug, Deserialize)]
pub struct Workbook {
    /// The worksheet metadata in the workbook.
    pub sheets: Sheets,
}

/// The worksheet metadata in the workbook.
#[derive(Debug, Deserialize)]
pub struct Sheets {
    /// All the individual worksheet metadata entry in the workbook.
    pub sheet: Vec<Sheet>,
}

/// A worksheet in the Excel workbook.
#[derive(Debug, Deserialize)]
pub struct Sheet {
    /// The display name of the worksheet.
    #[serde(rename = "@name")]
    pub name: String,
    /// The unique ID of the worksheet.
    #[serde(rename = "@sheetId")]
    pub id: usize,
    /// The ID of the worksheet in the relationship file.
    #[serde(rename = "@id")]
    pub rid: String,
}

/// The relationship file in the workbook archive.
#[derive(Debug, Deserialize)]
pub struct Relationships {
    /// All the relationship entrys in the workbook.
    #[serde(rename = "Relationship")]
    pub relationship: Vec<Relationship>,
}

/// A relationship entry in the workbook.
#[derive(Debug, Deserialize)]
pub struct Relationship {
    /// The unique ID of the relationship.
    #[serde(rename = "@Id")]
    pub id: String,
    /// The file path to the relationship.
    #[serde(rename = "@Target")]
    pub path: String,
}

/// The data and information of a given worksheet.
#[derive(Debug, Deserialize)]
pub struct Worksheet {
    /// The data in the worksheet.
    #[serde(rename = "sheetData")]
    pub sheet_data: SheetData,
}

/// The data in the worksheet.
#[derive(Debug, Deserialize)]
pub struct SheetData {
    /// All the rows of data in the worksheet.
    pub row: Vec<SheetRow>,
}

/// A row of data in the worksheet.
#[derive(Debug, Deserialize)]
pub struct SheetRow {
    /// All the cell in the row of data.
    #[serde(rename = "c")]
    pub cells: Vec<SheetCell>,
}

/// A cell in the worksheet.
#[derive(Debug, Deserialize)]
pub struct SheetCell {
    /// The cell location in the worksheet.
    #[serde(rename = "@r")]
    pub cell: String,
    /// The ID of the style used.
    #[serde(rename = "@s")]
    pub style: String,
}

/// An iterator for all columns in an Excel worksheet.
///
/// This iterator is an infinite iterator. It starts with a single letter from
/// A-Z. After that it goes from AA-AZ, BA-BZ and so on. A new letter will be
/// added to the squence when it reaches to the end.
///
/// # Examples
///
/// ```rust
/// use nott_a_database::spreadsheet_ml::XlsxColumns;
///
/// let mut columns = XlsxColumns::new();
///
/// assert_eq!(columns.next(), Some(String::from("A")));
/// assert_eq!(columns.nth(26), Some(String::from("AB")));
/// assert_eq!(columns.next(), Some(String::from("AC")));
/// ```
#[derive(Debug)]
pub struct XlsxColumns {
    /// The current amount of letters to output.
    count: usize,
    /// The internal output iterator.
    state: ChainPermutation<'static>,
}

impl XlsxColumns {
    const LETTERS: [&str; 26] = [
        "A", "B", "C", "D", "E", "F", "G", "H", "I", "J", "K", "L", "M", "N", "O", "P", "Q", "R",
        "S", "T", "U", "V", "W", "X", "Y", "Z",
    ];

    /// Creates a new iterator through all columns in an Excel worksheet.
    pub fn new() -> Self {
        Self {
            count: 1,
            state: ChainPermutation::new(1, &Self::LETTERS)
                .expect("There should be more than one item in LETTERS."),
        }
    }
}

impl Default for XlsxColumns {
    fn default() -> Self {
        Self::new()
    }
}

impl Iterator for XlsxColumns {
    type Item = String;

    fn next(&mut self) -> Option<Self::Item> {
        if let Some(v) = self.state.next() {
            Some(v)
        } else {
            self.count += 1;
            self.state = ChainPermutation::new(self.count, &Self::LETTERS)
                .expect("There should be more than one item in LETTERS.");
            Some(
                self.state
                    .next()
                    .expect("XLSX should have infinite columns."),
            )
        }
    }
}

/// Generate permutations of `count` amount for a given array (`data`).
#[derive(Debug)]
struct ChainPermutation<'a> {
    /// The amount of letters in the output.
    count: usize,
    /// The internal iterator state.
    state: Cloned<Iter<'a, &'a str>>,
    /// The inner iterator for more than one letter.
    inner: Option<Box<ChainPermutation<'a>>>,
    /// The data array containing all the possible values.
    data: &'a [&'a str],
    /// The current letter in the iteration.
    ///
    /// **Note**: It is only used when the count is more than one.
    current: &'a str,
}

impl<'a> ChainPermutation<'a> {
    /// Create a new [`ChainPermutation`] instance.
    ///
    /// The [`ChainPermutation`] iterator will output string of length `count`.
    /// The different possible letters are taken from `data` array.
    fn new(count: usize, data: &'a [&str]) -> Option<Self> {
        if data.is_empty() {
            return None;
        }

        Some(if count > 1 {
            let mut state = data.iter().cloned();
            let current = state
                .next()
                .expect("There should be more than one item in data.");
            Self {
                count,
                state,
                inner: Some(Box::new(
                    Self::new(count - 1, data)
                        .expect("There should be more than one item in data."),
                )),
                current,
                data,
            }
        } else {
            Self {
                count,
                state: data.iter().cloned(),
                inner: None,
                current: "",
                data,
            }
        })
    }
}

impl Iterator for ChainPermutation<'_> {
    type Item = String;

    fn next(&mut self) -> Option<Self::Item> {
        Some(match self.inner.as_mut() {
            Some(inner) => {
                if let Some(e) = inner.next() {
                    self.current.to_owned() + &e
                } else {
                    self.current = self.state.next()?;
                    self.inner = Some(Box::new(
                        Self::new(self.count - 1, self.data)
                            .expect("There should be more than one item in data."),
                    ));
                    self.current.to_owned()
                        + &self
                            .inner
                            .as_mut()
                            .expect("There should be an inner.")
                            .next()
                            .expect("There should be more than one item in data.")
                }
            }
            None => self.state.next()?.to_string(),
        })
    }
}

/// Gets data from an XLSX file.
///
/// This function will extract the XML data from a file in the given XLSX
/// archive `archive_file`. The path of the XML data to extract is given by the
/// `file` parameter. This function will returned a [`serde`] deserialized object.
///
/// # Examples
///
/// This example extracts the styles in the XLSX archive.
///
/// ```rust
/// use nott_a_database::spreadsheet_ml::{get_data, Styles};
///
/// let styles: Styles = get_data("./sample_0A.xlsx", "xl/styles.xml").expect("Unable to find file");
/// println!("{:?}", styles);
/// ```
pub fn get_data<T: for<'a> Deserialize<'a> + Debug, P: AsRef<Path>>(
    file: P,
    archive_file: &str,
) -> Result<T, Box<dyn std::error::Error>> {
    let file = File::open(file)?;
    let mut archive = ZipArchive::new(file)?;

    let mut archive_file = archive.by_name(archive_file)?;
    let mut file_content = String::new();
    archive_file.read_to_string(&mut file_content)?;

    let output: T = from_str(&file_content)?;
    Ok(output)
}
