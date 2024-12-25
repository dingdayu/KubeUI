use tauri::{AppHandle, Manager};

// Learn more about Tauri commands at https://tauri.app/develop/calling-rust/
#[tauri::command]
fn greet(name: &str) -> String {
    format!("Hello, {}! You've been greeted from Rust!", name)
}

#[tauri::command]
fn close_splashscreen(app: AppHandle) {
    // Show main window
    app.get_webview_window("main").unwrap().show().unwrap();
    // Close splashscreen
    app.get_webview_window("splashscreen")
        .unwrap()
        .close()
        .unwrap();
}

#[cfg_attr(mobile, tauri::mobile_entry_point)]
pub fn run() {
    let mut builder = tauri::Builder::default();

    #[cfg(desktop)]
    {
        builder = builder.plugin(tauri_plugin_single_instance::init(|app, args, cwd| {
            println!(
                "Arguments: {:?}, Current working directory: {:?}",
                args, cwd
            );
            let _ = app
                .get_webview_window("main")
                .expect("no main window")
                .set_focus();
        }));
    }
    
    builder
        .plugin(tauri_plugin_process::init())
        .plugin(tauri_plugin_dialog::init())
        .plugin(tauri_plugin_deep_link::init())
        .plugin(tauri_plugin_updater::Builder::new().build())
        .plugin(tauri_plugin_opener::init())
        .setup(|app| {
            #[cfg(any(windows, target_os = "linux"))]
            {
                use tauri_plugin_deep_link::DeepLinkExt;
                app.deep_link().register_all()?;
            }
            Ok::<(), Box<dyn std::error::Error>>(())
        })
        .invoke_handler(tauri::generate_handler![greet, close_splashscreen])
        .run(tauri::generate_context!())
        .expect("error while running tauri application");
}
