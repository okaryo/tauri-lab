use serde::Serialize;
use std::sync::Mutex;
use tauri::State;

#[derive(Serialize)]
#[serde(rename_all = "camelCase")]
struct Greeting {
    message: String,
    name_length: usize,
}

#[derive(Clone, Serialize)]
#[serde(rename_all = "camelCase")]
struct Todo {
    id: u32,
    title: String,
    completed: bool,
}

#[derive(Default)]
struct TodoStore {
    todos: Vec<Todo>,
    next_id: u32,
}

#[derive(Default)]
struct AppState {
    todo_store: Mutex<TodoStore>,
}

#[tauri::command]
fn greet(name: &str) -> Result<Greeting, String> {
    let name = name.trim();

    if name.is_empty() {
        return Err("Name is required.".to_string());
    }

    Ok(Greeting {
        message: format!("Hello, {name}! You've been greeted from Rust."),
        name_length: name.chars().count(),
    })
}

#[tauri::command]
fn create_todo(title: &str, state: State<'_, AppState>) -> Result<Todo, String> {
    let title = title.trim();

    if title.is_empty() {
        return Err("Todo title is required.".to_string());
    }

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

#[cfg_attr(mobile, tauri::mobile_entry_point)]
pub fn run() {
    tauri::Builder::default()
        .manage(AppState::default())
        .invoke_handler(tauri::generate_handler![greet, create_todo, list_todos])
        .run(tauri::generate_context!())
        .expect("error while running tauri application");
}
