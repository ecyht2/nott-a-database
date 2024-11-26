//! Simple CLI to parse the raw data and store it into the database.
use refinery::embed_migrations;
use rusqlite::{params, Connection};

use nott_a_database::{database::insert_student_info, StudentInfo};

embed_migrations!("./migrations");

fn main() -> Result<(), Box<dyn std::error::Error>> {
    let mut args: Vec<String> = std::env::args().collect();
    if args.len() != 2 {
        return Err("Invalid Command Line Arguments".into());
    }

    let file = args
        .pop()
        .expect("There should be atleast 2 elements in args");

    let mut conn = Connection::open("./test.db")?;
    conn.pragma(None, "foreign_keys", 1, |_| Ok(()))?;
    migrations::runner().run(&mut conn)?;
    conn.execute(
        "INSERT OR IGNORE INTO AcademicYear
         (AcademicYear) VALUES (?1)",
        params!["2024/2025"],
    )?;

    let data = StudentInfo::from_award(file)?;
    insert_student_info(&data, &mut conn)?;

    Ok(())
}
