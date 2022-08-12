#![cfg_attr(
    all(not(debug_assertions), target_os = "windows"),
    windows_subsystem = "windows"
)]
use tauri::Manager;

#[tauri::command]
fn hello_test(word: String) -> String {
    format!("Hello, {}", word)
}
fn main() {
    tauri::Builder::default()
        .setup(|app| {
            app.listen_global("backend", |event| {
                println!(
                    "get backend messgae from fronrend {}",
                    event.payload().unwrap()
                );
            });
            Ok(())
        })
        .invoke_handler(tauri::generate_handler![hello_test])
        .run(tauri::generate_context!())
        .expect("error while running tauri application");
}
