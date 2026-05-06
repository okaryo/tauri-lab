use serde::Serialize;
use std::sync::Mutex;
use tauri::State;

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

#[tauri::command]
fn complete_todo(id: u32, state: State<'_, AppState>) -> Result<Todo, String> {
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

    Ok(todo.clone())
}

#[cfg_attr(mobile, tauri::mobile_entry_point)]
pub fn run() {
    tauri::Builder::default()
        .manage(AppState::default())
        .invoke_handler(tauri::generate_handler![
            create_todo,
            list_todos,
            complete_todo
        ])
        .run(tauri::generate_context!())
        .expect("error while running tauri application");
}
