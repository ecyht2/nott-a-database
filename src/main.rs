//! Simple CLI to parse the raw data and store it into the database.

use nott_a_database::StudentResult;
use refinery::embed_migrations;
use rusqlite::{params, Connection};

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
    let trans = conn.transaction()?;

    let data = StudentResult::from_workbook(file)?;
    let mut insert_result = trans.prepare(
        "INSERT INTO Result
         (ID, AcademicYear, Plan, YearOfStudy, AutumnCredits, AutumnMean,
          SpringCredits, SpringMean, YearCredits, YearMean, Progression,
          Remarks)
         VALUES 
         (?1, ?2, ?3, ?4, ?5, ?6, ?7, ?8, ?9, ?10, ?11, ?12)",
    )?;
    let mut insert_student = trans.prepare(
        "INSERT OR IGNORE INTO StudentInfo
         (ID, FirstName, LastName) VALUES (?1, ?2, ?3)",
    )?;
    let mut insert_module = trans.prepare(
        "INSERT OR IGNORE INTO Module
         (Code, Credit) VALUES (?1, ?2)",
    )?;
    let mut insert_mark = trans.prepare(
        "INSERT INTO Mark
         (ID, Module, Mark, Status, Fill) VALUES (?1, ?2, ?3, ?4, ?5)",
    )?;
    let mut colour_insert = trans.prepare(
        "
        INSERT INTO FillColour (Alpha, Red, Green, Blue)
        SELECT ?1, ?2, ?3, ?4
        WHERE NOT EXISTS (
            SELECT Alpha, Red, Green, Blue
            FROM FillColour
            WHERE Alpha=?1 AND Red=?2 AND Green=?3 AND Blue=?4
        )
        ",
    )?;
    let mut colour_get = trans.prepare(
        "
        SELECT *
        FROM FillColour
        WHERE Alpha=?1 AND Red=?2 AND Green=?3 AND Blue=?4
        ",
    )?;

    for result in data {
        insert_student.execute(params![result.id, result.first_name, result.last_name])?;

        insert_result.insert(params![
            result.id,
            "2024/2025",
            result.plan,
            result.year_of_program,
            result.autumn_credit,
            result.autumn_mean,
            result.spring_credit,
            result.spring_mean,
            result.year_credit,
            result.year_prog_average,
            result.progression,
            result.remarks,
        ])?;

        for module in result.modules {
            insert_module.execute(params![module.code, module.credit])?;
            let colour_id: Option<i64> = match module.fill {
                Some(fill) => {
                    colour_insert.execute(params![fill.alpha, fill.red, fill.green, fill.blue,])?;
                    Some(colour_get.query_row(
                        params![fill.alpha, fill.red, fill.green, fill.blue,],
                        |row| row.get(0),
                    )?)
                }
                None => None,
            };

            insert_mark.insert(params![
                result.id,
                module.code,
                module.mark,
                module.status,
                colour_id
            ])?;
        }
    }

    drop(insert_result);
    drop(insert_student);
    drop(insert_module);
    drop(insert_mark);
    drop(colour_insert);
    drop(colour_get);
    trans.commit()?;

    Ok(())
}
