//! Parser for raw data from exam results.

pub use marks::StudentResult;

pub mod database;
pub mod errors;
mod marks;
pub mod spreadsheet_ml;
