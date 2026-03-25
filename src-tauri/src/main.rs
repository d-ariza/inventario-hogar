// Prevents additional console window on Windows in release
#![cfg_attr(not(debug_assertions), windows_subsystem = "windows")]

use inventario_hogar_tauri::{AppState, load_config};

fn main() {
  tauri::Builder::default()
      .setup(|app| {
        let app_handle = app.handle();

        // Cargar configuración
        match load_config(&app_handle) {
          Ok(config) => {
            println!("✅ Configuración cargada correctamente");
            app.manage(AppState {
              config: std::sync::Mutex::new(Some(config)),
              app_name: "inventario_hogar".to_string(),
            });
          }
          Err(e) => {
            eprintln!("❌ Error al cargar configuración: {}", e);
            app.manage(AppState {
              config: std::sync::Mutex::new(None),
              app_name: "inventario_hogar".to_string(),
            });
          }
        }

        Ok(())
      })
      .invoke_handler(tauri::generate_handler![
            inventario_hogar_tauri::get_items,
            inventario_hogar_tauri::get_resumen,
            inventario_hogar_tauri::add_item,
            inventario_hogar_tauri::delete_item,
            inventario_hogar_tauri::update_item,
            inventario_hogar_tauri::buscar_items,
        ])
      .run(tauri::generate_context!())
      .expect("error while running tauri application");
}