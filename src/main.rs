//! Simple CLI to parse the raw data and store it into the database.
use std::{path::PathBuf, str::FromStr};

use clap::Parser;
use refinery::embed_migrations;
use rusqlite::Connection;

use nott_a_database::{
    database::{insert_student_info_transaction, insert_student_result_transaction},
    AcademicYear, StudentInfo, StudentResult,
};

embed_migrations!("./migrations");

/// Simple CLI to parse the raw data and store it into the database.
#[derive(Debug, Parser)]
struct Arg {
    /// The acdemic year of the reports.
    #[clap(value_parser = AcademicYear::from_str)]
    academic_year: AcademicYear,
    /// The database file to save to.
    datbase: PathBuf,
    /// List of raw data file to parse.
    #[command(flatten)]
    data: RawData,
}

/// CLI arguments to supply raw data.
#[derive(Debug, Parser)]
#[group(required = true)]
struct RawData {
    /// Specify (can specify multiple) result report (0A) raw data to parse.
    #[arg(long)]
    result: Vec<PathBuf>,
    /// Specify (can specify multiple) award report (0B) raw data to parse.
    #[arg(long)]
    award: Vec<PathBuf>,
    /// Specify (can specify multiple) May resit report (0C) raw data to parse.
    #[arg(long)]
    resit_may: Vec<PathBuf>,
    /// Specify (can specify multiple) August resit report (0D) raw data to parse.
    #[arg(long)]
    resit_aug: Vec<PathBuf>,
}

fn main() -> Result<(), Box<dyn std::error::Error>> {
    let args = Arg::parse();

    let mut conn = Connection::open(args.datbase)?;
    conn.pragma(None, "foreign_keys", 1, |_| Ok(()))?;
    migrations::runner().run(&mut conn)?;
    args.academic_year.insert_db_sync(&mut conn)?;
    let trans = conn.transaction()?;

    // Parse result raw data
    for file in args.data.result {
        let data = StudentResult::from_result(file)?;
        insert_student_result_transaction(&trans, &data)?;
    }

    // Parse award report raw data
    for file in args.data.award {
        let data = StudentInfo::from_award(file)?;
        insert_student_info_transaction(&data, &trans)?;
    }

    // Parse May resit raw data
    for file in args.data.resit_may {
        let data = StudentResult::from_resit_may(file)?;
        insert_student_result_transaction(&trans, &data)?;
    }

    // Parse August resit raw data
    for file in args.data.resit_aug {
        let data = StudentResult::from_resit_aug(file)?;
        insert_student_result_transaction(&trans, &data)?;
    }

    trans.commit()?;

    Ok(())
}
