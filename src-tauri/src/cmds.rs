use tauri::{AppHandle, Manager};

type CmdResult<T = ()> = Result<T, String>;

// Learn more about Tauri commands at https://tauri.app/develop/calling-rust/
#[tauri::command]
pub async fn greet(name: &str) -> CmdResult<String> {
    Ok(format!("Hello, {}! You've been greeted from Rust!", name))
}

#[tauri::command]
pub async fn close_splashscreen(app: AppHandle) {
    // Show main window
    app.get_webview_window("main").unwrap().show().unwrap();
    // Close splashscreen
    app.get_webview_window("splashscreen")
        .unwrap()
        .close()
        .unwrap();
}
