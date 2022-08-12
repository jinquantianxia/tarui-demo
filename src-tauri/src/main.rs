#![cfg_attr(
    all(not(debug_assertions), target_os = "windows"),
    windows_subsystem = "windows"
)]
use std::env;
use tauri::Manager;
use tauri::{CustomMenuItem, SystemTray, SystemTrayEvent, SystemTrayMenu, SystemTrayMenuItem};

#[tauri::command]
fn hello_test(word: String) -> String {
    format!("Hello, {}", word)
}

#[tauri::command]
fn show_main_window(window: tauri::Window) {
    if let Some(window) = window.get_window("splashscreen") {
        window.close().unwrap();
    }
    window.get_window("main").unwrap().show().unwrap();
}
fn main() {
    env::set_var("RUST_BACKTRACE", "1");
    let quit = CustomMenuItem::new("quit".to_string(), "QUIT");
    let hide = CustomMenuItem::new("hide".to_string(), "HIDE");
    let tray_menu = SystemTrayMenu::new()
        .add_item(quit)
        .add_native_item(SystemTrayMenuItem::Separator)
        .add_item(hide);
    let system_tray = SystemTray::new().with_menu(tray_menu);
    tauri::Builder::default()
        .system_tray(system_tray)
        .on_system_tray_event(|app, event| match event {
            SystemTrayEvent::MenuItemClick { id, .. } => match id.as_str() {
                "hide" => {
                    let window = app.get_window("main").unwrap();
                    window.hide().unwrap();
                }
                "quit" => {
                    let window = app.get_window("main").unwrap();
                    window.close().unwrap();
                }
                _ => {}
            },
            _ => {}
        })
        .setup(|app| {
            app.listen_global("backend", |event| {
                println!(
                    "get backend messgae from fronrend {}",
                    event.payload().unwrap()
                );
            });
            Ok(())
        })
        .invoke_handler(tauri::generate_handler![hello_test, show_main_window])
        .run(tauri::generate_context!())
        .expect("error while running tauri application");
}
