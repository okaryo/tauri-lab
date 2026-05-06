use rusqlite::{params, Connection};
use serde::Serialize;
use std::fs;
use std::path::PathBuf;
use std::time::{SystemTime, UNIX_EPOCH};
use tauri::{AppHandle, Manager};

#[derive(Clone, Serialize)]
#[serde(rename_all = "camelCase")]
struct Todo {
    id: u32,
    title: String,
    completed: bool,
}

#[derive(Clone, Serialize)]
#[serde(rename_all = "camelCase")]
struct WorkLog {
    id: u32,
    body: String,
    created_at_ms: u64,
}

fn app_data_dir(app: &AppHandle) -> Result<PathBuf, String> {
    let app_data_dir = app
        .path()
        .app_data_dir()
        .map_err(|error| format!("Failed to resolve app data directory: {error}"))?;

    fs::create_dir_all(&app_data_dir)
        .map_err(|error| format!("Failed to create app data directory: {error}"))?;

    Ok(app_data_dir)
}

fn database_path(app: &AppHandle) -> Result<PathBuf, String> {
    Ok(app_data_dir(app)?.join("data.sqlite"))
}

fn open_database(app: &AppHandle) -> Result<Connection, String> {
    let path = database_path(app)?;

    Connection::open(path).map_err(|error| format!("Failed to open database: {error}"))
}

fn init_database(app: &AppHandle) -> Result<(), String> {
    let mut connection = open_database(app)?;
    let version = current_schema_version(&connection)?;

    if version < 1 {
        migrate_to_v1(&mut connection)?;
    }

    Ok(())
}

fn current_schema_version(connection: &Connection) -> Result<u32, String> {
    connection
        .query_row("PRAGMA user_version", [], |row| row.get(0))
        .map_err(|error| format!("Failed to read schema version: {error}"))
}

fn migrate_to_v1(connection: &mut Connection) -> Result<(), String> {
    let transaction = connection
        .transaction()
        .map_err(|error| format!("Failed to start migration v1: {error}"))?;

    transaction
        .execute_batch(
            "
            CREATE TABLE IF NOT EXISTS todos (
                id INTEGER PRIMARY KEY AUTOINCREMENT,
                title TEXT NOT NULL,
                completed INTEGER NOT NULL DEFAULT 0
            );

            CREATE TABLE IF NOT EXISTS work_logs (
                id INTEGER PRIMARY KEY AUTOINCREMENT,
                body TEXT NOT NULL,
                created_at_ms INTEGER NOT NULL
            );

            PRAGMA user_version = 1;
            ",
        )
        .map_err(|error| format!("Failed to apply migration v1: {error}"))?;

    transaction
        .commit()
        .map_err(|error| format!("Failed to commit migration v1: {error}"))?;

    Ok(())
}

fn row_id_to_u32(row_id: i64) -> Result<u32, String> {
    u32::try_from(row_id).map_err(|_| format!("Database row id {row_id} is out of range."))
}

#[tauri::command]
fn create_todo(title: &str, app: AppHandle) -> Result<Todo, String> {
    let title = title.trim();

    if title.is_empty() {
        return Err("Todo title is required.".to_string());
    }

    let connection = open_database(&app)?;

    connection
        .execute(
            "INSERT INTO todos (title, completed) VALUES (?1, ?2)",
            params![title, false],
        )
        .map_err(|error| format!("Failed to create todo: {error}"))?;

    Ok(Todo {
        id: row_id_to_u32(connection.last_insert_rowid())?,
        title: title.to_string(),
        completed: false,
    })
}

#[tauri::command]
fn list_todos(app: AppHandle) -> Result<Vec<Todo>, String> {
    let connection = open_database(&app)?;
    let mut statement = connection
        .prepare("SELECT id, title, completed FROM todos ORDER BY id ASC")
        .map_err(|error| format!("Failed to prepare todo list query: {error}"))?;
    let todos = statement
        .query_map([], |row| {
            Ok(Todo {
                id: row.get(0)?,
                title: row.get(1)?,
                completed: row.get(2)?,
            })
        })
        .map_err(|error| format!("Failed to query todos: {error}"))?
        .collect::<Result<Vec<_>, _>>()
        .map_err(|error| format!("Failed to read todos: {error}"))?;

    Ok(todos)
}

#[tauri::command]
fn complete_todo(id: u32, app: AppHandle) -> Result<Todo, String> {
    let connection = open_database(&app)?;
    let updated_rows = connection
        .execute(
            "UPDATE todos SET completed = ?1 WHERE id = ?2",
            params![true, id],
        )
        .map_err(|error| format!("Failed to complete todo: {error}"))?;

    if updated_rows == 0 {
        return Err(format!("Todo #{id} was not found."));
    }

    connection
        .query_row(
            "SELECT id, title, completed FROM todos WHERE id = ?1",
            params![id],
            |row| {
                Ok(Todo {
                    id: row.get(0)?,
                    title: row.get(1)?,
                    completed: row.get(2)?,
                })
            },
        )
        .map_err(|error| format!("Failed to read completed todo: {error}"))
}

#[tauri::command]
fn create_work_log(body: &str, app: AppHandle) -> Result<WorkLog, String> {
    let body = body.trim();

    if body.is_empty() {
        return Err("Work log body is required.".to_string());
    }

    let created_at_ms = SystemTime::now()
        .duration_since(UNIX_EPOCH)
        .map_err(|_| "System time is before Unix epoch.".to_string())?
        .as_millis() as u64;

    let connection = open_database(&app)?;

    connection
        .execute(
            "INSERT INTO work_logs (body, created_at_ms) VALUES (?1, ?2)",
            params![body, created_at_ms],
        )
        .map_err(|error| format!("Failed to create work log: {error}"))?;

    Ok(WorkLog {
        id: row_id_to_u32(connection.last_insert_rowid())?,
        body: body.to_string(),
        created_at_ms,
    })
}

#[tauri::command]
fn list_work_logs(app: AppHandle) -> Result<Vec<WorkLog>, String> {
    let connection = open_database(&app)?;
    let mut statement = connection
        .prepare("SELECT id, body, created_at_ms FROM work_logs ORDER BY id DESC")
        .map_err(|error| format!("Failed to prepare work log list query: {error}"))?;
    let work_logs = statement
        .query_map([], |row| {
            Ok(WorkLog {
                id: row.get(0)?,
                body: row.get(1)?,
                created_at_ms: row.get(2)?,
            })
        })
        .map_err(|error| format!("Failed to query work logs: {error}"))?
        .collect::<Result<Vec<_>, _>>()
        .map_err(|error| format!("Failed to read work logs: {error}"))?;

    Ok(work_logs)
}

#[cfg_attr(mobile, tauri::mobile_entry_point)]
pub fn run() {
    tauri::Builder::default()
        .plugin(tauri_plugin_notification::init())
        .setup(|app| {
            #[cfg(desktop)]
            app.handle()
                .plugin(tauri_plugin_global_shortcut::Builder::new().build())?;

            if let Err(error) = init_database(app.handle()) {
                eprintln!("Failed to initialize database: {error}");
            }

            Ok(())
        })
        .invoke_handler(tauri::generate_handler![
            create_todo,
            list_todos,
            complete_todo,
            create_work_log,
            list_work_logs
        ])
        .run(tauri::generate_context!())
        .expect("error while running tauri application");
}
