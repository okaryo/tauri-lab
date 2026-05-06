use serde::{Deserialize, Serialize};
use std::fs;
use std::path::PathBuf;
use std::sync::Mutex;
use std::time::{SystemTime, UNIX_EPOCH};
use tauri::{AppHandle, Manager, State};

#[derive(Clone, Deserialize, Serialize)]
#[serde(rename_all = "camelCase")]
struct Todo {
    id: u32,
    title: String,
    completed: bool,
}

#[derive(Clone, Deserialize, Serialize)]
#[serde(rename_all = "camelCase")]
struct WorkLog {
    id: u32,
    body: String,
    created_at_ms: u64,
}

#[derive(Clone, Default, Deserialize, Serialize)]
#[serde(rename_all = "camelCase")]
struct TodoStore {
    todos: Vec<Todo>,
    next_id: u32,
}

#[derive(Clone, Default, Deserialize, Serialize)]
#[serde(rename_all = "camelCase")]
struct WorkLogStore {
    work_logs: Vec<WorkLog>,
    next_id: u32,
}

#[derive(Default, Deserialize, Serialize)]
#[serde(rename_all = "camelCase")]
struct PersistedData {
    todo_store: TodoStore,
    work_log_store: WorkLogStore,
}

#[derive(Default)]
struct AppState {
    todo_store: Mutex<TodoStore>,
    work_log_store: Mutex<WorkLogStore>,
}

fn data_file_path(app: &AppHandle) -> Result<PathBuf, String> {
    let app_data_dir = app
        .path()
        .app_data_dir()
        .map_err(|error| format!("Failed to resolve app data directory: {error}"))?;

    fs::create_dir_all(&app_data_dir)
        .map_err(|error| format!("Failed to create app data directory: {error}"))?;

    Ok(app_data_dir.join("data.json"))
}

fn load_state(app: &AppHandle, state: &AppState) -> Result<(), String> {
    let path = data_file_path(app)?;

    if !path.exists() {
        return Ok(());
    }

    let json =
        fs::read_to_string(&path).map_err(|error| format!("Failed to read data file: {error}"))?;
    let data: PersistedData = serde_json::from_str(&json)
        .map_err(|error| format!("Failed to parse data file: {error}"))?;

    *state
        .todo_store
        .lock()
        .map_err(|_| "Todo store lock is poisoned.".to_string())? = data.todo_store;

    *state
        .work_log_store
        .lock()
        .map_err(|_| "Work log store lock is poisoned.".to_string())? = data.work_log_store;

    Ok(())
}

fn save_state(app: &AppHandle, state: &AppState) -> Result<(), String> {
    let todo_store = state
        .todo_store
        .lock()
        .map_err(|_| "Todo store lock is poisoned.".to_string())?
        .clone();
    let work_log_store = state
        .work_log_store
        .lock()
        .map_err(|_| "Work log store lock is poisoned.".to_string())?
        .clone();

    let data = PersistedData {
        todo_store,
        work_log_store,
    };
    let json = serde_json::to_string_pretty(&data)
        .map_err(|error| format!("Failed to serialize data: {error}"))?;
    let path = data_file_path(app)?;

    fs::write(path, json).map_err(|error| format!("Failed to write data file: {error}"))?;

    Ok(())
}

#[tauri::command]
fn create_todo(title: &str, state: State<'_, AppState>, app: AppHandle) -> Result<Todo, String> {
    let title = title.trim();

    if title.is_empty() {
        return Err("Todo title is required.".to_string());
    }

    let todo = {
        let mut store = state
            .todo_store
            .lock()
            .map_err(|_| "Todo store lock is poisoned.".to_string())?;

        store.next_id += 1;

        let todo = Todo {
            id: store.next_id,
            title: title.to_string(),
            completed: false,
        };

        store.todos.push(todo.clone());

        todo
    };

    save_state(&app, &state)?;

    Ok(todo)
}

#[tauri::command]
fn list_todos(state: State<'_, AppState>) -> Result<Vec<Todo>, String> {
    let store = state
        .todo_store
        .lock()
        .map_err(|_| "Todo store lock is poisoned.".to_string())?;

    Ok(store.todos.clone())
}

#[tauri::command]
fn complete_todo(id: u32, state: State<'_, AppState>, app: AppHandle) -> Result<Todo, String> {
    let completed_todo = {
        let mut store = state
            .todo_store
            .lock()
            .map_err(|_| "Todo store lock is poisoned.".to_string())?;

        let todo = store
            .todos
            .iter_mut()
            .find(|todo| todo.id == id)
            .ok_or_else(|| format!("Todo #{id} was not found."))?;

        todo.completed = true;

        todo.clone()
    };

    save_state(&app, &state)?;

    Ok(completed_todo)
}

#[tauri::command]
fn create_work_log(
    body: &str,
    state: State<'_, AppState>,
    app: AppHandle,
) -> Result<WorkLog, String> {
    let body = body.trim();

    if body.is_empty() {
        return Err("Work log body is required.".to_string());
    }

    let created_at_ms = SystemTime::now()
        .duration_since(UNIX_EPOCH)
        .map_err(|_| "System time is before Unix epoch.".to_string())?
        .as_millis() as u64;

    let work_log = {
        let mut store = state
            .work_log_store
            .lock()
            .map_err(|_| "Work log store lock is poisoned.".to_string())?;

        store.next_id += 1;

        let work_log = WorkLog {
            id: store.next_id,
            body: body.to_string(),
            created_at_ms,
        };

        store.work_logs.push(work_log.clone());

        work_log
    };

    save_state(&app, &state)?;

    Ok(work_log)
}

#[tauri::command]
fn list_work_logs(state: State<'_, AppState>) -> Result<Vec<WorkLog>, String> {
    let store = state
        .work_log_store
        .lock()
        .map_err(|_| "Work log store lock is poisoned.".to_string())?;

    Ok(store.work_logs.clone())
}

#[cfg_attr(mobile, tauri::mobile_entry_point)]
pub fn run() {
    tauri::Builder::default()
        .manage(AppState::default())
        .setup(|app| {
            let state = app.state::<AppState>();

            if let Err(error) = load_state(app.handle(), &state) {
                eprintln!("Failed to load persisted data: {error}");
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
