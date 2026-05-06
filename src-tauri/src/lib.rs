use serde::Serialize;

#[derive(Serialize)]
#[serde(rename_all = "camelCase")]
struct Greeting {
    message: String,
    name_length: usize,
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

#[cfg_attr(mobile, tauri::mobile_entry_point)]
pub fn run() {
    tauri::Builder::default()
        .invoke_handler(tauri::generate_handler![greet])
        .run(tauri::generate_context!())
        .expect("error while running tauri application");
}
