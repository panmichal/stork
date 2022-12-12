#![cfg_attr(
    all(not(debug_assertions), target_os = "windows"),
    windows_subsystem = "windows"
)]

use std::fs::OpenOptions;
use std::io::{Read, Write};
use std::ops::Deref;
use std::sync::Mutex;
use tauri::{ActivationPolicy, GlobalShortcutManager, Manager};
use tauri::{CustomMenuItem, SystemTray, SystemTrayEvent, SystemTrayMenu};

mod links;

const FILENAME: &str = "links.txt";

struct State {
    data_path: Mutex<String>,
}

#[tauri::command]
fn save_link(state: tauri::State<State>, url: &str, name: &str, desc: &str) -> bool {
    let guard = state.data_path.lock().unwrap();
    let data_dir = guard.deref();
    let file_path = format!("{}/{}", data_dir, FILENAME);
    let string = format!("{};;{};;{}\n", url, name, desc);

    if !std::path::Path::new(data_dir).is_dir() {
        std::fs::create_dir_all(data_dir).unwrap();
    }

    let mut file = OpenOptions::new()
        .create(true)
        .append(true)
        .open(file_path)
        .unwrap();
    file.write_all(string.as_bytes()).is_ok()
}
#[tauri::command]
fn get_links(state: tauri::State<State>) -> Vec<links::Link> {
    let guard = state.data_path.lock().unwrap();
    let data_dir = guard.deref();
    let file_path = format!("{}/{}", data_dir, FILENAME);

    let links = if std::path::Path::new(&file_path).is_file() {
        let mut file = OpenOptions::new().read(true).open(file_path).unwrap();
        let mut contents = String::new();
        file.read_to_string(&mut contents).unwrap();

        contents
    } else {
        String::new()
    };
    links::parse_links(&links).unwrap()
}

fn toggle_main_window_visibility(app: &tauri::AppHandle) {
    let window = match app.get_window("main") {
        Some(window) => window,
        None => return,
    };
    if let Ok(true) = window.is_visible() {
        window.hide().unwrap();
    } else {
        window.show().unwrap();
        window.set_focus().unwrap();
    }
}

fn main() {
    let quit = CustomMenuItem::new("quit".to_string(), "Quit");
    let tray_menu = SystemTrayMenu::new().add_item(quit);
    let tray = SystemTray::new().with_menu(tray_menu);
    let context = tauri::generate_context!();

    let app_dir = tauri::api::path::app_data_dir(context.config())
        .unwrap()
        .to_str()
        .unwrap()
        .to_string();

    let mut app = tauri::Builder::default()
        .manage(State {
            data_path: Mutex::new(app_dir),
        })
        .invoke_handler(tauri::generate_handler![save_link, get_links])
        .system_tray(tray)
        .on_system_tray_event(|app, event| match event {
            SystemTrayEvent::LeftClick { .. } => {
                let window = match app.get_window("main") {
                    Some(window) => window,
                    None => return,
                };
                if let Ok(true) = window.is_visible() {
                    window.hide().unwrap();
                } else {
                    window.show().unwrap();
                    window.set_focus().unwrap();
                }
            }
            SystemTrayEvent::MenuItemClick { id, .. } => match id.as_str() {
                "quit" => {
                    std::process::exit(0);
                }
                _ => {}
            },
            _ => {
                println!("Other event");
            }
        })
        .on_window_event(|event| match event.event() {
            tauri::WindowEvent::CloseRequested { api, .. } => {
                event.window().hide().unwrap();
                api.prevent_close();
            }
            _ => {}
        })
        .build(context)
        .expect("error while building tauri application");

    let app_handle = app.handle();

    app.set_activation_policy(ActivationPolicy::Accessory);
    app.global_shortcut_manager()
        .register("Cmd+Shift+T", move || {
            toggle_main_window_visibility(&app_handle);
        })
        .unwrap();
    app.run(|_app_handle, _event| {});
}
