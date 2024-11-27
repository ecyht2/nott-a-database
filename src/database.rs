//! Implementation for inserting data into the database.
use rusqlite::{params, types::ToSqlOutput, Connection, ToSql, Transaction};

use crate::{ModuleStatus, StudentInfo, StudentResult};

impl ToSql for ModuleStatus {
    fn to_sql(&self) -> rusqlite::Result<rusqlite::types::ToSqlOutput<'_>> {
        Ok(match self {
            ModuleStatus::Pass => ToSqlOutput::Borrowed("Pass".into()),
            ModuleStatus::SoftFail => ToSqlOutput::Borrowed("SF".into()),
            ModuleStatus::HardFail => ToSqlOutput::Borrowed("HF".into()),
            ModuleStatus::ComponentFail => ToSqlOutput::Borrowed("CF".into()),
        })
    }
}

/// Insert [`StudentResult`] into a database using a database connection.
pub fn insert_student_result(
    conn: &mut Connection,
    data: &[StudentResult],
) -> Result<(), rusqlite::Error> {
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
         (ID, AcademicYear, YearOfStudy, AutumnCredits, AutumnMean,
          SpringCredits, SpringMean, YearCredits, YearMean, Progression,
          Remarks)
         VALUES 
         (?1, ?2, ?3, ?4, ?5, ?6, ?7, ?8, ?9, ?10, ?11)",
    )?;
    let mut insert_student = trans.prepare(
        "INSERT OR IGNORE INTO StudentInfo
         (ID, FirstName, LastName, Plan) VALUES (?1, ?2, ?3, ?4)",
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
        insert_student.execute(params![
            result.student_info.id,
            result.student_info.first_name,
            result.student_info.last_name,
            result.student_info.plan,
        ])?;

        insert_result.insert(params![
            result.student_info.id,
            "2024/2025",
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
                result.student_info.id,
                module.code,
                module.mark,
                module.status,
                colour_id
            ])?;
        }
    }

    Ok(())
}

impl StudentInfo {
    pub const INSERT_STATEMENT: &'static str = "
        INSERT INTO StudentInfo
        (
            ID,
            FirstName,
            LastName,
            Plan,
            PlanDesc,
            Program,
            ProgramDesc,
            INTAKE,
            CareerNo,
            QAA,
            CalcModel,
            RawMark,
            TruncatedMark,
            FinalMark,
            Borderline,
            Calculation,
            DegreeAward,
            Selected,
            ExceptionData,
            Recommendation
        )
        VALUES (
            ?1,
            ?2,
            ?3,
            ?4,
            ?5,
            ?6,
            ?7,
            ?8,
            ?9,
            ?10,
            ?11,
            ?12,
            ?13,
            ?14,
            ?15,
            ?16,
            ?17,
            ?18,
            ?19,
            ?20
        )
        ON CONFLICT DO UPDATE SET
        FirstName=?2,
        LastName=?3,
        Plan=?4,
        PlanDesc=?5,
        Program=?6,
        ProgramDesc=?7,
        INTAKE=?8,
        CareerNo=?9,
        QAA=?10,
        CalcModel=?11,
        RawMark=?12,
        TruncatedMark=?13,
        FinalMark=?14,
        Borderline=?15,
        Calculation=?16,
        DegreeAward=?17,
        Selected=?18,
        ExceptionData=?19,
        Recommendation=?20
        ";

    /// Insert [`StudentInfo`] into a database using a database connection.
    pub fn insert_db_sync(&self, conn: &mut Connection) -> Result<(), rusqlite::Error> {
        let trans = conn.transaction()?;
        self.insert_db_transaction_sync(&trans)?;
        trans.commit()?;
        Ok(())
    }

    /// Insert [`StudentInfo`] into database using a database transaction.
    /// *Note*: This function does not commit the changes to the database.
    pub fn insert_db_transaction_sync(&self, trans: &Transaction) -> Result<(), rusqlite::Error> {
        trans.execute(
            Self::INSERT_STATEMENT,
            params![
                self.id,
                self.first_name,
                self.last_name,
                self.plan,
                self.plan_description,
                self.academic_program,
                self.program_description,
                self.intake,
                self.carrer_number,
                self.qaa_effective_date
                    .map(|v| v.format("%D%M%Y").to_string()),
                self.calculation_model,
                self.raw_mark,
                self.truncated_mark,
                self.final_mark,
                self.borderline,
                self.calculation,
                self.degree_award,
                self.selected,
                self.exception_data,
                self.recommendation,
            ],
        )?;

        Ok(())
    }
}

/// Insert [`StudentInfo`] into a database using a database connection.
pub fn insert_student_info(
    data: &[StudentInfo],
    conn: &mut Connection,
) -> Result<(), rusqlite::Error> {
    let trans = conn.transaction()?;
    insert_student_info_transaction(data, &trans)?;
    trans.commit()?;
    Ok(())
}

/// Insert [`StudentInfo`] into database using a database transaction.
/// *Note*: This function does not commit the changes to the database.
pub fn insert_student_info_transaction(
    data: &[StudentInfo],
    trans: &Transaction,
) -> Result<(), rusqlite::Error> {
    for info in data {
        info.insert_db_transaction_sync(trans)?;
    }

    Ok(())
}
