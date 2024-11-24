//! Implementation for inserting data into the database.
use rusqlite::{params, Connection, Transaction};

use crate::StudentResult;

/// Insert [`StudentResult`] into a database using a database connection.
pub fn insert_student_result(conn: &mut Connection, data: &[StudentResult]) -> Result<(), rusqlite::Error> {
    let trans = conn.transaction()?;
    insert_student_result_transaction(&trans, data)?;
    trans.commit()?;
    Ok(())
}

/// Insert [`StudentResult`] into database using a database transaction.
/// *Note*: This function does not commit the changes to the database.
pub fn insert_student_result_transaction(
    trans: &Transaction,
    data: &[StudentResult],
) -> Result<(), rusqlite::Error> {
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

        for module in &result.modules {
            insert_module.execute(params![module.code, module.credit])?;
            let colour_id: Option<i64> = match &module.fill {
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

    Ok(())
}
