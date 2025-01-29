use std::path::PathBuf;

use serde::Deserialize;
use sqlx::{migrate, sqlite::SqliteConnectOptions, SqlitePool};
use tauri::{Manager, State};
use tokio::sync::Mutex;

use nott_a_database_core::{
    database::{insert_student_info_async, insert_student_result_async},
    AcademicYear, StudentInfo, StudentResult,
};

#[derive(Debug, Deserialize)]
#[serde(rename_all = "kebab-case")]
enum DataType {
    Result,
    Award,
    ResitMay,
    ResitAug,
}

// Learn more about Tauri commands at https://tauri.app/develop/calling-rust/
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
        .invoke_handler(tauri::generate_handler![insert_data])
        .run(tauri::generate_context!())
        .expect("error while running tauri application");
}
