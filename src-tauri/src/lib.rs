use serde::Serialize;
use std::path::PathBuf;
use tauri::{
    menu::{Menu, MenuItem},
    tray::TrayIconBuilder,
};
use tauri::{AppHandle, Manager};
use tauri_plugin_global_shortcut::{Code, GlobalShortcutExt, Modifiers, Shortcut, ShortcutState};
use walkdir::WalkDir;

#[derive(Serialize, Clone)]
struct AppInfo {
    name: String,
    icon: String, // Placeholder for now
    path: String,
}

#[tauri::command]
fn search_apps(query: &str) -> Vec<AppInfo> {
    let mut apps = Vec::new();
    let query = query.to_lowercase();

    // Common Start Menu paths
    let paths = vec![
        r"C:\ProgramData\Microsoft\Windows\Start Menu\Programs",
        r"C:\Users\kusha\AppData\Roaming\Microsoft\Windows\Start Menu\Programs", // TODO: dynamic user path?
    ];

    for path in paths {
        for entry in WalkDir::new(path).into_iter().filter_map(|e| e.ok()) {
            if entry.path().extension().and_then(|s| s.to_str()) == Some("lnk") {
                let file_name = entry.file_name().to_string_lossy().to_string();
                if file_name.to_lowercase().contains(&query) {
                    let name = file_name.replace(".lnk", "");
                    // For now, we just invoke the lnk itself
                    apps.push(AppInfo {
                        name,
                        icon: "application".to_string(),
                        path: entry.path().to_string_lossy().to_string(),
                    });
                }
            }
        }
    }
    apps.truncate(5); // Limit results
    apps
}

#[cfg_attr(mobile, tauri::mobile_entry_point)]
pub fn run() {
    tauri::Builder::default()
        .plugin(tauri_plugin_opener::init())
        .invoke_handler(tauri::generate_handler![search_apps])
        .setup(|app| {
            #[cfg(desktop)]
            {
                let ctrl_n_shortcut = Shortcut::new(Some(Modifiers::ALT), Code::Space);
                app.handle().plugin(
                    tauri_plugin_global_shortcut::Builder::new()
                        .with_handler(move |app, shortcut, event| {
                            println!("{:?}", shortcut);
                            if shortcut == &ctrl_n_shortcut {
                                match event.state() {
                                    ShortcutState::Pressed => {
                                        if let Some(window) = app.get_webview_window("main") {
                                            if window.is_visible().unwrap_or(false) {
                                                window.hide().unwrap();
                                            } else {
                                                window.show().unwrap();
                                                window.set_focus().unwrap();
                                            }
                                        }
                                    }
                                    _ => {}
                                }
                            }
                        })
                        .build(),
                )?;

                app.global_shortcut().register(ctrl_n_shortcut)?;
            }

            let quit_i = MenuItem::with_id(app, "quit", "Quit", true, None::<&str>)?;
            let toggle_i = MenuItem::with_id(app, "toggle", "Toggle", true, None::<&str>)?;
            let menu = Menu::with_items(app, &[&toggle_i, &quit_i])?;

            let _tray = TrayIconBuilder::with_id("tray")
                .menu(&menu)
                .icon(app.default_window_icon().unwrap().clone())
                .on_menu_event(|app, event| match event.id().as_ref() {
                    "quit" => {
                        for window in app.webview_windows().values() {
                            let _ = window.close();
                        }
                        app.exit(0);
                    }
                    "toggle" => {
                        if let Some(window) = app.get_webview_window("main") {
                            if window.is_visible().unwrap_or(false) {
                                window.hide().unwrap();
                            } else {
                                window.show().unwrap();
                                window.set_focus().unwrap();
                            }
                        }
                    }
                    _ => {}
                })
                .build(app)?;

            Ok(())
        })
        .run(tauri::generate_context!())
        .expect("error while running tauri application");
}
