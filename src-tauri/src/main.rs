#![cfg_attr(
    all(not(debug_assertions), target_os = "windows"),
    windows_subsystem = "windows"
)]

fn main() {
    tauri::Builder::default()
        .invoke_handler(tauri::generate_handler![double])
        .run(tauri::generate_context!())
        .expect("error while running tauri application");
}

#[tauri::command]
fn double(num: u8) -> u8 {
    return num * 2;
}
