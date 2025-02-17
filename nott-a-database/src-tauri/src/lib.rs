/// TODO: Move types into the core module.
/// TODO: Limit the amount of student per fetch.
/// TODO: Use React Suspense to prevent blocking.
/// TODO: Handle errors when calling invokes.
use std::path::PathBuf;

use serde::Deserialize;
use sqlx::{migrate, sqlite::SqliteConnectOptions, SqlitePool};
use tauri::{Manager, State};
use tokio::sync::Mutex;

use nott_a_database_core::{
    database::{insert_student_info_async, insert_student_result_async},
    AcademicYear, StudentInfo, StudentResult,
};

/// Enumerations of all the different possible data types.
#[derive(Debug, Deserialize)]
#[serde(rename_all = "kebab-case")]
enum DataType {
    /// The `Result (0A)` data type.
    Result,
    /// The `Award (0B)` data type.
    Award,
    /// The `May Resit (0C)` data type.
    ResitMay,
    /// The `August Resit (0D)` data type.
    ResitAug,
}

// Inserts new data into the database.
#[tauri::command]
async fn insert_data(
    data_type: DataType,
    academic_year: AcademicYear,
    path: PathBuf,
    db_pool: State<'_, Mutex<SqlitePool>>,
) -> Result<(), String> {
    log::debug!("Rust Data\nType: {data_type:?}\nYear: {academic_year}\nPath: {path:?}");

    let mut db_pool = db_pool.lock().await;

    // Inserting Academic Year
    academic_year
        .insert_db_async(&mut db_pool)
        .await
        .map_err(|e| e.to_string())?;

    // Inserting Data
    match data_type {
        DataType::Result => {
            let data = StudentResult::from_result(path).map_err(|e| e.to_string())?;
            insert_student_result_async(&mut db_pool, &data, &academic_year)
                .await
                .map_err(|e| e.to_string())?;
        }
        DataType::Award => {
            let data = StudentInfo::from_award(path).map_err(|e| e.to_string())?;
            insert_student_info_async(&mut db_pool, &data, &academic_year, true)
                .await
                .map_err(|e| e.to_string())?;
        }
        DataType::ResitMay => {
            let data = StudentResult::from_resit_may(path).map_err(|e| e.to_string())?;
            insert_student_result_async(&mut db_pool, &data, &academic_year)
                .await
                .map_err(|e| e.to_string())?;
        }
        DataType::ResitAug => {
            let data = StudentResult::from_resit_aug(path).map_err(|e| e.to_string())?;
            insert_student_result_async(&mut db_pool, &data, &academic_year)
                .await
                .map_err(|e| e.to_string())?;
        }
    };
    Ok(())
}

/// Commands, types and utilities for interacting with module data.
mod modules {
    use serde::{Deserialize, Serialize};
    use sqlx::{prelude::FromRow, SqlitePool};
    use tauri::State;
    use tokio::sync::Mutex;

    /// Wrapper type containing all the columns of the `Module` table.
    #[derive(Debug, Deserialize, Serialize, FromRow)]
    #[sqlx(rename_all = "PascalCase")]
    pub struct Module {
        /// The module code of the module in the row.
        code: String,
        /// The number of credits of the module in the row.
        credit: u32,
        /// The name of the module in the row.
        name: Option<String>,
    }

    /// Fetches all the modules currently saved in the database.
    #[tauri::command]
    pub async fn update_module(
        module: Module,
        db_pool: State<'_, Mutex<SqlitePool>>,
    ) -> Result<Module, String> {
        let db_pool = db_pool.lock().await;
        let data = sqlx::query("UPDATE Module SET CREDIT=?2,NAME=?3 WHERE CODE=?1")
            .bind(&module.code)
            .bind(module.credit)
            .bind(&module.name)
            .execute(&*db_pool)
            .await
            .map_err(|e| e.to_string());
        match data {
            Ok(_) => Ok(module),
            Err(e) => {
                log::error!("Error updating module {}: {e}", module.code);
                Err(e)
            }
        }
    }

    /// Updates a module in the database.
    #[tauri::command]
    pub async fn get_modules(db_pool: State<'_, Mutex<SqlitePool>>) -> Result<Vec<Module>, String> {
        let db_pool = db_pool.lock().await;
        let data = sqlx::query_as("SELECT * from Module")
            .fetch_all(&*db_pool)
            .await
            .map_err(|e| e.to_string());
        match data {
            Ok(data) => Ok(data),
            Err(e) => {
                log::error!("Error fecthing module data: {e}");
                Err(e)
            }
        }
    }
}

mod students {
    use serde::Serialize;
    use sqlx::{prelude::FromRow, SqlitePool};
    use tauri::State;
    use tokio::sync::Mutex;

    /// Wrapper type for a row of data in the StudentInfo table.
    #[derive(Debug, Serialize, FromRow)]
    #[sqlx(rename_all = "PascalCase")]
    #[serde(rename_all = "camelCase")]
    pub struct StudentInfo {
        #[sqlx(rename = "ID")]
        id: u64,
        last_name: String,
        first_name: String,
        career_no: Option<u64>,
        program: Option<String>,
        program_desc: Option<String>,
        plan: String,
        plan_desc: Option<String>,
        #[sqlx(rename = "INTAKE")]
        intake: Option<String>,
        #[sqlx(rename = "QAA")]
        qaa: Option<String>,
        calc_model: Option<String>,
        raw_mark: Option<f64>,
        truncated_mark: Option<f64>,
        final_mark: Option<u64>,
        borderline: Option<String>,
        calculation: Option<u64>,
        degree_award: Option<String>,
        selected: Option<u64>,
        exception_data: Option<String>,
        recommendation: Option<String>,
        intake_year: String,
        graduation_year: Option<String>,
    }

    /// Fetches all the students in the database.
    #[tauri::command]
    pub async fn get_student_info(
        db_pool: State<'_, Mutex<SqlitePool>>,
    ) -> Result<Vec<StudentInfo>, String> {
        let db_pool = db_pool.lock().await;
        let data = sqlx::query_as("SELECT * from StudentInfo")
            .fetch_all(&*db_pool)
            .await
            .map_err(|e| e.to_string());
        match data {
            Ok(data) => Ok(data),
            Err(e) => {
                log::error!("Error fecthing student info: {e}");
                Err(e)
            }
        }
    }

    /// Fetches information about a student in the database.
    #[tauri::command]
    pub async fn get_student(
        id: i64,
        db_pool: State<'_, Mutex<SqlitePool>>,
    ) -> Result<StudentInfo, String> {
        let db_pool = db_pool.lock().await;
        let data = sqlx::query_as("SELECT * from StudentInfo WHERE ID=?1")
            .bind(id)
            .fetch_one(&*db_pool)
            .await
            .map_err(|e| e.to_string());
        match data {
            Ok(data) => Ok(data),
            Err(e) => {
                log::error!("Error fecthing student info: {e}");
                Err(e)
            }
        }
    }

    /// Wrapper type for a row of data in the Result table.
    #[derive(Debug, Serialize, FromRow)]
    #[sqlx(rename_all = "PascalCase")]
    #[serde(rename_all = "camelCase")]
    pub struct StudentResult {
        academic_year: String,
        #[sqlx(rename = "ID")]
        id: u64,
        year_of_study: u64,
        autumn_credits: Option<u64>,
        autumn_mean: Option<f64>,
        spring_credits: Option<u64>,
        spring_mean: Option<f64>,
        year_credits: Option<u64>,
        year_mean: Option<f64>,
        progression: Option<String>,
        remarks: Option<String>,
    }

    /// Fetches all the student's results every year in the database.
    #[tauri::command]
    pub async fn get_results(
        id: i64,
        db_pool: State<'_, Mutex<SqlitePool>>,
    ) -> Result<Vec<StudentResult>, String> {
        let db_pool = db_pool.lock().await;
        let data = sqlx::query_as("SELECT * from Result WHERE ID=?1")
            .bind(id)
            .fetch_all(&*db_pool)
            .await
            .map_err(|e| e.to_string());
        match data {
            Ok(data) => Ok(data),
            Err(e) => {
                log::error!("Error fecthing results for {id}: {e}");
                Err(e)
            }
        }
    }

    /// Wrapper type for a row of data in the Mark table.
    #[derive(Debug, Serialize, FromRow)]
    #[sqlx(rename_all = "PascalCase")]
    #[serde(rename_all = "camelCase")]
    pub struct Mark {
        #[sqlx(rename = "ID")]
        id: u64,
        mark: f64,
        fill: Option<u64>,
        retake1: Option<f64>,
        retake2: Option<f64>,
        extra: Option<String>,
        module: String,
        status: String,
    }

    /// Fetches all the student's module marks in the database.
    #[tauri::command]
    pub async fn get_marks(
        id: i64,
        db_pool: State<'_, Mutex<SqlitePool>>,
    ) -> Result<Vec<Mark>, String> {
        let db_pool = db_pool.lock().await;
        let data = sqlx::query_as("SELECT * from Mark WHERE ID=?1")
            .bind(id)
            .fetch_all(&*db_pool)
            .await
            .map_err(|e| e.to_string());
        match data {
            Ok(data) => Ok(data),
            Err(e) => {
                log::error!("Error fecthing marks for {id}: {e}");
                Err(e)
            }
        }
    }
}

/// Allows blocking on async code without creating a nested runtime.
///
/// This function is taken from [SQL Tauri Plugin](https://github.com/tauri-apps/plugins-workspace/blob/v2/plugins/sql/src/lib.rs).
fn run_async_command<F: std::future::Future>(cmd: F) -> F::Output {
    if tokio::runtime::Handle::try_current().is_ok() {
        tokio::task::block_in_place(|| tokio::runtime::Handle::current().block_on(cmd))
    } else {
        tauri::async_runtime::block_on(cmd)
    }
}

#[cfg_attr(mobile, tauri::mobile_entry_point)]
pub fn run() {
    tauri::Builder::default()
        .plugin(tauri_plugin_dialog::init())
        .plugin(tauri_plugin_log::Builder::new().build())
        .setup(|app| {
            run_async_command(async move {
                let mut db_path = app.path().app_data_dir().expect("Unsupported OS detected.");
                std::fs::create_dir_all(&db_path).unwrap();
                db_path.push("data.db");

                let db_options = SqliteConnectOptions::new()
                    .filename(db_path)
                    .create_if_missing(true)
                    .pragma("foreign_keys", "1");

                let pool = SqlitePool::connect_with(db_options).await?;
                migrate!("../../nott-a-database-core/migrations-async")
                    .run(&pool)
                    .await?;
                app.manage(Mutex::new(pool));

                Ok(())
            })
        })
        .invoke_handler(tauri::generate_handler![
            insert_data,
            modules::get_modules,
            modules::update_module,
            students::get_student_info,
            students::get_student,
            students::get_results,
            students::get_marks
        ])
        .run(tauri::generate_context!())
        .expect("error while running tauri application");
}
