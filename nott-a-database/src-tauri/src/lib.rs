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
    use serde::Serialize;
    use sqlx::{prelude::FromRow, SqlitePool};
    use tauri::State;
    use tokio::sync::Mutex;

    /// Wrapper type containing all the columns of the `Module` table.
    #[derive(Debug, Serialize, FromRow)]
    #[sqlx(rename_all = "PascalCase")]
    pub struct Module {
        /// The module code of the module in the row.
        code: String,
        /// The number of credits of the module in the row.
        credit: u64,
        /// The name of the module in the row.
        name: Option<String>,
    }

    /// Fetches all the modules currently saved in the database.
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
        .invoke_handler(tauri::generate_handler![insert_data, modules::get_modules,])
        .run(tauri::generate_context!())
        .expect("error while running tauri application");
}
