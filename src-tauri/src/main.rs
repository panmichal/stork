#![cfg_attr(
    all(not(debug_assertions), target_os = "windows"),
    windows_subsystem = "windows"
)]

use tauri::{ActivationPolicy, GlobalShortcutManager, Manager};
use tauri::{CustomMenuItem, SystemTray, SystemTrayEvent, SystemTrayMenu};

#[tauri::command]
fn save_link(url: &str, name: &str, desc: &str) -> String {
    println!("Saving link: {} - {} - {}", url, name, desc);
    format!("Saving link: {} {} {}", url, name, desc)
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

    let app_dir = tauri::api::path::app_data_dir(context.config()).unwrap();
    println!("App dir: {:?}", app_dir);

    let mut app = tauri::Builder::default()
        .invoke_handler(tauri::generate_handler![save_link])
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
                println!("Close requested");
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
            println!("Shortcut pressed");
            toggle_main_window_visibility(&app_handle);
        })
        .unwrap();
    app.run(|_app_handle, _event| {});
}
