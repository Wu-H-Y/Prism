// Prevents additional console window on Windows in release, DO NOT REMOVE!!
#![cfg_attr(not(debug_assertions), windows_subsystem = "windows")]

use sha2::Digest;
#[cfg(debug_assertions)]
use specta_typescript::BigIntExportBehavior;
use specta_typescript::Typescript;
use tauri_specta::{Builder, collect_commands};

// Import clean architecture modules
mod commands;
mod events;
mod state;
mod domain;

#[tokio::main]
async fn main() {
    // 启动 Tauri v2
    let builder = Builder::<tauri::Wry>::new().commands(collect_commands![]);

    #[cfg(debug_assertions)]
    builder
        .export(
            Typescript::default().bigint(BigIntExportBehavior::BigInt),
            "../src/lib/bindings.ts",
        )
        .expect("Failed to export typescript bindings");

    tauri::Builder::default()
        .plugin(tauri_plugin_shell::init())
        .plugin(tauri_plugin_fs::init())
        .plugin(tauri_plugin_log::Builder::new().build())
        .invoke_handler(builder.invoke_handler())
        .setup(move |app| {
            builder.mount_events(app);
            Ok(())
        })
        .run(tauri::generate_context!())
        .expect("error while running tauri application");
}
