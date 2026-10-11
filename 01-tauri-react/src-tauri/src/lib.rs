use serde::Serialize;
use std::sync::Mutex;
use tauri::State;

#[derive(Serialize, Clone)]
struct Todo {
    id: u32,
    title: String,
    done: bool,
}

#[derive(Default)]
struct Store {
    todos: Vec<Todo>,
    next_id: u32,
}

type AppState = Mutex<Store>;

#[tauri::command]
fn list(state: State<AppState>) -> Vec<Todo> {
    state.lock().unwrap().todos.clone()
}

#[tauri::command]
fn add(state: State<AppState>, title: String) -> Vec<Todo> {
    let mut s = state.lock().unwrap();
    let title = title.trim().to_string();
    if !title.is_empty() {
        s.next_id += 1;
        let id = s.next_id;
        s.todos.push(Todo { id, title, done: false });
    }
    s.todos.clone()
}

#[tauri::command]
fn toggle(state: State<AppState>, id: u32) -> Vec<Todo> {
    let mut s = state.lock().unwrap();
    if let Some(t) = s.todos.iter_mut().find(|t| t.id == id) {
        t.done = !t.done;
    }
    s.todos.clone()
}

#[tauri::command]
fn remove(state: State<AppState>, id: u32) -> Vec<Todo> {
    let mut s = state.lock().unwrap();
    s.todos.retain(|t| t.id != id);
    s.todos.clone()
}

#[cfg_attr(mobile, tauri::mobile_entry_point)]
pub fn run() {
    tauri::Builder::default()
        .manage(AppState::default())
        .invoke_handler(tauri::generate_handler![list, add, toggle, remove])
        .run(tauri::generate_context!())
        .expect("error while running tauri application");
}
