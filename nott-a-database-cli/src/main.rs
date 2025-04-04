//! Simple CLI to parse the raw data and store it into the database.
use std::{path::PathBuf, str::FromStr};

use clap::Parser;
use refinery::embed_migrations;
use rusqlite::Connection;

use nott_a_database_core::{
    database::{insert_student_info_transaction, insert_student_result_transaction},
    AcademicYear, StudentInfo, StudentResult,
};

embed_migrations!("../nott-a-database-core/migrations");

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

    /// Prints nothing to the standard output.
    #[arg(short, long, group = "print")]
    quiet: bool,
    /// Prints debug outputs to the standard output.
    #[arg(short, long, group = "print")]
    verbose: bool,
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

fn main() -> Result<(), anyhow::Error> {
    let args = Arg::parse();

    if !args.quiet {
        println!("Saving data to: {}", &args.datbase.to_string_lossy());
    }
    let mut conn = Connection::open(args.datbase)?;
    conn.pragma(None, "foreign_keys", 1, |_| Ok(()))?;
    migrations::runner().run(&mut conn)?;
    args.academic_year.insert_db_sync(&mut conn)?;
    let trans = conn.transaction()?;

    // Parse result raw data
    for file in args.data.result {
        if !args.quiet {
            println!("Parsing data from {}..", &file.to_string_lossy());
        }
        let data = StudentResult::from_result(&file)?;

        if args.verbose {
            println!("{:#?}", data);
        }
        if !args.quiet {
            println!("Found {} rows in {}", data.len(), file.to_string_lossy());
        }
        insert_student_result_transaction(&trans, &data, &args.academic_year)?;
    }

    // Parse award report raw data
    for file in args.data.award {
        if !args.quiet {
            println!("Parsing data from {}..", &file.to_string_lossy());
        }
        let data = StudentInfo::from_award(&file)?;

        if args.verbose {
            println!("{:#?}", data);
        }
        if !args.quiet {
            println!("Found {} rows in {}", data.len(), file.to_string_lossy());
        }
        insert_student_info_transaction(&data, &trans, &args.academic_year, true)?;
    }

    // Parse May resit raw data
    for file in args.data.resit_may {
        if !args.quiet {
            println!("Parsing data from {}..", &file.to_string_lossy());
        }
        let data = StudentResult::from_resit_may(&file)?;

        if args.verbose {
            println!("{:#?}", data);
        }
        if !args.quiet {
            println!("Found {} rows in {}", data.len(), file.to_string_lossy());
        }
        insert_student_result_transaction(&trans, &data, &args.academic_year)?;
    }

    // Parse August resit raw data
    for file in args.data.resit_aug {
        if !args.quiet {
            println!("Parsing data from {}..", &file.to_string_lossy());
        }
        let data = StudentResult::from_resit_aug(&file)?;

        if args.verbose {
            println!("{:#?}", data);
        }
        if !args.quiet {
            println!("Found {} rows in {}", data.len(), file.to_string_lossy());
        }
        insert_student_result_transaction(&trans, &data, &args.academic_year)?;
    }

    trans.commit()?;

    if !args.quiet {
        println!("Done");
    }

    Ok(())
}
