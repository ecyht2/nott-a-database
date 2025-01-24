//! Implementation for inserting data into the database.
#[cfg(feature = "sync")]
use rusqlite::{params, types::ToSqlOutput, Connection, ToSql, Transaction};

#[cfg(feature = "async")]
use sqlx::{Sqlite, SqlitePool, Transaction as AsyncTransaction};

use crate::{AcademicYear, StudentInfo, StudentResult};
#[cfg(feature = "sync")]
use crate::ModuleStatus;

#[cfg(feature = "sync")]
impl ToSql for AcademicYear {
    fn to_sql(&self) -> rusqlite::Result<ToSqlOutput<'_>> {
        Ok(ToSqlOutput::Owned(self.to_string().into()))
    }
}

impl AcademicYear {
    pub const INSERT_STATEMENT: &str = "
        INSERT OR IGNORE INTO AcademicYear
        VALUES (?1)
        ";

    /// Add a new [`AcademicYear`] into database using a database connection.
    #[cfg(feature = "sync")]
    pub fn insert_db_sync(&self, conn: &mut Connection) -> Result<(), rusqlite::Error> {
        let trans = conn.transaction()?;
        self.insert_db_transaction_sync(&trans)?;
        trans.commit()?;
        Ok(())
    }

    /// Add a new [`AcademicYear`] into database using a database transaction.
    /// *Note*: This function does not commit the changes to the database.
    #[cfg(feature = "sync")]
    pub fn insert_db_transaction_sync(&self, trans: &Transaction) -> Result<(), rusqlite::Error> {
        trans.execute(Self::INSERT_STATEMENT, params![self])?;
        Ok(())
    }

    /// Add a new [`AcademicYear`] into database using a database connection.
    #[cfg(feature = "async")]
    pub async fn insert_db_async(&self, conn: &mut SqlitePool) -> Result<(), sqlx::Error> {
        let mut trans = conn.begin().await?;
        self.insert_db_transaction_async(&mut trans).await?;
        trans.commit().await?;
        Ok(())
    }

    /// Add a new [`AcademicYear`] into database using a database transaction.
    /// *Note*: This function does not commit the changes to the database.
    #[cfg(feature = "async")]
    pub async fn insert_db_transaction_async(
        &self,
        trans: &mut AsyncTransaction<'_, Sqlite>,
    ) -> Result<(), sqlx::Error> {
        sqlx::query(
            "INSERT OR IGNORE INTO AcademicYear
             VALUES (?1)",
        )
        .bind(self.to_string())
        .execute(&mut **trans)
        .await?;
        Ok(())
    }
}

#[cfg(feature = "sync")]
impl ToSql for ModuleStatus {
    fn to_sql(&self) -> rusqlite::Result<rusqlite::types::ToSqlOutput<'_>> {
        Ok(ToSqlOutput::Owned(self.to_string().into()))
    }
}

/// Insert [`StudentResult`] into a database using a database connection.
#[cfg(feature = "sync")]
pub fn insert_student_result(
    conn: &mut Connection,
    data: &[StudentResult],
    intake: &AcademicYear,
) -> Result<(), rusqlite::Error> {
    let trans = conn.transaction()?;
    insert_student_result_transaction(&trans, data, intake)?;
    trans.commit()?;
    Ok(())
}

/// Insert [`StudentResult`] into database using a database transaction.
/// *Note*: This function does not commit the changes to the database.
#[cfg(feature = "sync")]
pub fn insert_student_result_transaction(
    trans: &Transaction,
    data: &[StudentResult],
    intake: &AcademicYear,
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
         (ID, FirstName, LastName, Plan, IntakeYear) VALUES (?1, ?2, ?3, ?4, ?5)",
    )?;
    let mut insert_module = trans.prepare(
        "INSERT OR IGNORE INTO Module
         (Code, Credit) VALUES (?1, ?2)",
    )?;
    let mut insert_mark = trans.prepare(
        "INSERT INTO Mark
         (ID, Module, Mark, Retake1, Retake2, Status, Fill)
         VALUES (?1, ?2, ?3, ?4, ?5, ?6, ?7)",
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
            intake,
        ])?;

        insert_result.insert(params![
            result.student_info.id,
            intake,
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
                module.retake1,
                module.retake2,
                module.status,
                colour_id
            ])?;
        }
    }

    Ok(())
}

/// Insert [`StudentResult`] into a database using a database connection.
#[cfg(feature = "async")]
pub async fn insert_student_result_async(
    conn: &mut SqlitePool,
    data: &[StudentResult],
    intake: &AcademicYear,
) -> Result<(), sqlx::Error> {
    let mut trans = conn.begin().await?;
    insert_student_result_transaction_async(&mut trans, data, intake).await?;
    trans.commit().await?;
    Ok(())
}

/// Insert [`StudentResult`] into database using a database transaction.
/// *Note*: This function does not commit the changes to the database.
#[cfg(feature = "async")]
pub async fn insert_student_result_transaction_async(
    trans: &mut AsyncTransaction<'_, Sqlite>,
    data: &[StudentResult],
    intake: &AcademicYear,
) -> Result<(), sqlx::Error> {
    for result in data {
        sqlx::query(
            "INSERT INTO Result
              (ID, AcademicYear, YearOfStudy, AutumnCredits, AutumnMean,
               SpringCredits, SpringMean, YearCredits, YearMean, Progression,
               Remarks)
              VALUES
              (?1, ?2, ?3, ?4, ?5, ?6, ?7, ?8, ?9, ?10, ?11)",
        )
        .bind(result.student_info.id)
        .bind(&result.student_info.first_name)
        .bind(&result.student_info.last_name)
        .bind(&result.student_info.plan)
        .bind(intake.to_string())
        .execute(&mut **trans)
        .await?;

        sqlx::query(
            "INSERT INTO Result
             (ID, AcademicYear, YearOfStudy, AutumnCredits, AutumnMean,
              SpringCredits, SpringMean, YearCredits, YearMean, Progression,
              Remarks)
             VALUES 
             (?1, ?2, ?3, ?4, ?5, ?6, ?7, ?8, ?9, ?10, ?11)",
        )
        .bind(result.student_info.id)
        .bind(intake.to_string())
        .bind(&result.year_of_program)
        .bind(result.autumn_credit)
        .bind(result.autumn_mean)
        .bind(result.spring_credit)
        .bind(result.spring_mean)
        .bind(result.year_credit)
        .bind(result.year_prog_average)
        .bind(&result.progression)
        .bind(&result.remarks)
        .execute(&mut **trans)
        .await?;

        for module in &result.modules {
            sqlx::query(
                "INSERT OR IGNORE INTO Module
                 (Code, Credit) VALUES (?1, ?2)",
            )
            .bind(&module.code)
            .bind(module.credit)
            .execute(&mut **trans)
            .await?;
            let colour_id: Option<i64> = match &module.fill {
                Some(fill) => {
                    sqlx::query(
                        "INSERT INTO FillColour (Alpha, Red, Green, Blue)
                         SELECT ?1, ?2, ?3, ?4
                         WHERE NOT EXISTS (
                             SELECT Alpha, Red, Green, Blue
                             FROM FillColour
                             WHERE Alpha=?1 AND Red=?2 AND Green=?3 AND Blue=?4
                         )",
                    )
                    .bind(fill.alpha)
                    .bind(fill.red)
                    .bind(fill.green)
                    .bind(fill.blue)
                    .execute(&mut **trans)
                    .await?;
                    Some(
                        sqlx::query_as::<_, (i64,)>(
                            "SELECT * FROM FillColour
                             WHERE Alpha=?1 AND Red=?2 AND Green=?3 AND Blue=?4",
                        )
                        .bind(fill.alpha)
                        .bind(fill.red)
                        .bind(fill.green)
                        .bind(fill.blue)
                        .fetch_one(&mut **trans)
                        .await?
                        .0,
                    )
                }
                None => None,
            };

            sqlx::query(
                "INSERT INTO Mark
              (ID, Module, Mark, Retake1, Retake2, Status, Fill)
              VALUES (?1, ?2, ?3, ?4, ?5, ?6, ?7)",
            )
            .bind(result.student_info.id)
            .bind(&module.code)
            .bind(module.mark)
            .bind(module.retake1)
            .bind(module.retake2)
            .bind(module.status.to_string())
            .bind(colour_id)
            .execute(&mut **trans)
            .await?;
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
            Recommendation,
            IntakeYear,
            GraduationYear
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
            ?20,
            ?21,
            ?22
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
        Recommendation=?20,
        GraduationYear=?22
        ";

    /// Insert [`StudentInfo`] into a database using a database connection.
    #[cfg(feature = "sync")]
    pub fn insert_db_sync(
        &self,
        conn: &mut Connection,
        intake: &AcademicYear,
        award: bool,
    ) -> Result<(), rusqlite::Error> {
        let trans = conn.transaction()?;
        self.insert_db_transaction_sync(&trans, intake, award)?;
        trans.commit()?;
        Ok(())
    }

    /// Insert [`StudentInfo`] into database using a database transaction.
    /// *Note*: This function does not commit the changes to the database.
    #[cfg(feature = "sync")]
    pub fn insert_db_transaction_sync(
        &self,
        trans: &Transaction,
        intake: &AcademicYear,
        award: bool,
    ) -> Result<(), rusqlite::Error> {
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
                intake,
                if award { Some(intake) } else { None }
            ],
        )?;

        Ok(())
    }

    /// Insert [`StudentInfo`] into a database using a database connection.
    #[cfg(feature = "async")]
    pub async fn insert_db_async(
        &self,
        conn: &mut SqlitePool,
        intake: &AcademicYear,
        award: bool,
    ) -> Result<(), sqlx::Error> {
        let mut trans = conn.begin().await?;
        self.insert_db_transaction_async(&mut trans, intake, award)
            .await?;
        trans.commit().await?;
        Ok(())
    }

    /// Insert [`StudentInfo`] into database using a database transaction.
    /// *Note*: This function does not commit the changes to the database.
    #[cfg(feature = "async")]
    pub async fn insert_db_transaction_async(
        &self,
        trans: &mut AsyncTransaction<'_, Sqlite>,
        intake: &AcademicYear,
        award: bool,
    ) -> Result<(), sqlx::Error> {
        sqlx::query(Self::INSERT_STATEMENT)
            .bind(self.id)
            .bind(&self.first_name)
            .bind(&self.last_name)
            .bind(&self.plan)
            .bind(&self.plan_description)
            .bind(&self.academic_program)
            .bind(&self.program_description)
            .bind(&self.intake)
            .bind(self.carrer_number)
            .bind(
                self.qaa_effective_date
                    .map(|v| v.format("%D%M%Y").to_string()),
            )
            .bind(&self.calculation_model)
            .bind(self.raw_mark)
            .bind(self.truncated_mark)
            .bind(self.final_mark)
            .bind(&self.borderline)
            .bind(self.calculation)
            .bind(&self.degree_award)
            .bind(self.selected)
            .bind(&self.exception_data)
            .bind(&self.recommendation)
            .bind(intake.to_string())
            .bind(if award {
                Some(intake.to_string())
            } else {
                None
            })
            .execute(&mut **trans)
            .await?;

        Ok(())
    }
}

/// Insert [`StudentInfo`] into a database using a database connection.
#[cfg(feature = "sync")]
pub fn insert_student_info(
    data: &[StudentInfo],
    conn: &mut Connection,
    intake: &AcademicYear,
    award: bool,
) -> Result<(), rusqlite::Error> {
    let trans = conn.transaction()?;
    insert_student_info_transaction(data, &trans, intake, award)?;
    trans.commit()?;
    Ok(())
}

/// Insert [`StudentInfo`] into database using a database transaction.
/// *Note*: This function does not commit the changes to the database.
#[cfg(feature = "sync")]
pub fn insert_student_info_transaction(
    data: &[StudentInfo],
    trans: &Transaction,
    intake: &AcademicYear,
    award: bool,
) -> Result<(), rusqlite::Error> {
    for info in data {
        info.insert_db_transaction_sync(trans, intake, award)?;
    }

    Ok(())
}

/// Insert [`StudentInfo`] into a database using a database connection.
#[cfg(feature = "async")]
pub async fn insert_student_info_async(
    conn: &mut SqlitePool,
    data: &[StudentInfo],
    intake: &AcademicYear,
    award: bool,
) -> Result<(), sqlx::Error> {
    let mut trans = conn.begin().await?;
    insert_student_info_transaction_async(&mut trans, data, intake, award).await?;
    trans.commit().await?;
    Ok(())
}

/// Insert [`StudentInfo`] into database using a database transaction.
/// *Note*: This function does not commit the changes to the database.
#[cfg(feature = "async")]
pub async fn insert_student_info_transaction_async(
    trans: &mut AsyncTransaction<'_, Sqlite>,
    data: &[StudentInfo],
    intake: &AcademicYear,
    award: bool,
) -> Result<(), sqlx::Error> {
    for info in data {
        info.insert_db_transaction_async(trans, intake, award)
            .await?;
    }

    Ok(())
}
