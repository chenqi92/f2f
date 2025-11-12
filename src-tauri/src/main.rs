// Prevents additional console window on Windows in release builds
#![cfg_attr(not(debug_assertions), windows_subsystem = "windows")]

mod commands;
mod types;
mod core;
mod error;

// use tauri::Manager;
use tracing_subscriber::{fmt, prelude::*, EnvFilter};

fn main() {
    // Initialize logging
    tracing_subscriber::registry()
        .with(fmt::layer())
        .with(EnvFilter::from_default_env()
            .add_directive(tracing::Level::INFO.into()))
        .init();

    tracing::info!("Starting F2F Converter");

    tauri::Builder::default()
        .plugin(tauri_plugin_shell::init())
        .setup(|app| {
            // Initialize core systems
            let app_handle = app.handle().clone();

            // Initialize capability registry
            if let Err(e) = core::registry::initialize(&app_handle) {
                tracing::error!("Failed to initialize capability registry: {}", e);
            }

            // Initialize database
            if let Err(e) = core::storage::initialize(&app_handle) {
                tracing::error!("Failed to initialize storage: {}", e);
            }

            tracing::info!("F2F Converter initialized successfully");
            Ok(())
        })
        .invoke_handler(tauri::generate_handler![
            commands::detect_capabilities,
            commands::detect_file,
            commands::plan_targets,
            commands::create_job,
            commands::list_jobs,
            commands::get_job,
            commands::control_job,
            commands::get_artifacts,
            commands::get_logs,
            commands::get_settings,
            commands::set_settings,
            commands::run_health_check,
        ])
        .run(tauri::generate_context!())
        .expect("error while running tauri application");
}
