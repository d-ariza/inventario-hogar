#![cfg_attr(not(debug_assertions), windows_subsystem = "windows")]

use inventario_hogar_tauri::AppState;
use tauri::Manager;

fn main() {
    tauri::Builder::default()
        .setup(|app| {
            let json_path = inventario_hogar_tauri::get_json_path();

            let config = if json_path.exists() {
                match inventario_hogar::cargar_desde_json(&json_path) {
                    Ok(c) => {
                        println!("✅ Configuración cargada: {} items", c.items.len());
                        Some(c)
                    }
                    Err(e) => {
                        eprintln!("❌ Error: {}", e);
                        None
                    }
                }
            } else {
                println!("🆕 Creando nueva configuración");
                let c = inventario_hogar::crear_ejemplo_aseo();
                if let Err(e) = inventario_hogar::guardar_a_json(&c, &json_path) {
                    eprintln!("❌ Error guardando: {}", e);
                }
                Some(c)
            };

            app.manage(AppState {
                config: std::sync::Mutex::new(config),
            });

            Ok(())
        })
        .invoke_handler(tauri::generate_handler![
            get_items,
            get_resumen,
            add_item,
            delete_item,
        ])
        .run(tauri::generate_context!())
        .expect("error while running tauri application");
}

#[tauri::command]
fn get_items(state: tauri::State<AppState>) -> Result<Vec<inventario_hogar_tauri::ItemResponse>, String> {
    let config = state.config.lock().unwrap();
    if let Some(ref cfg) = *config {
        let items: Vec<inventario_hogar_tauri::ItemResponse> = cfg.items.iter().map(|item| {
            inventario_hogar_tauri::ItemResponse {
                id: item.id.to_string(),
                producto: item.producto.clone(),
                cantidad_mensual: item.cantidad_mensual,
                precio_unitario: item.precio_unitario,
                costo_mensual: item.costo_mensual,
                tienda: item.tienda.clone(),
                categoria: item.categoria.clone(),
            }
        }).collect();
        Ok(items)
    } else {
        Ok(vec![])
    }
}

#[tauri::command]
fn get_resumen(state: tauri::State<AppState>) -> Result<inventario_hogar_tauri::ResumenResponse, String> {
    let config = state.config.lock().unwrap();
    if let Some(ref cfg) = *config {
        let gasto_por_categoria: Vec<(String, u64)> = cfg.gasto_por_categoria().into_iter().collect();

        let top_items: Vec<inventario_hogar_tauri::ItemResponse> = cfg.top_items_mas_caros(5)
            .iter()
            .map(|item| inventario_hogar_tauri::ItemResponse {
                id: item.id.to_string(),
                producto: item.producto.clone(),
                cantidad_mensual: item.cantidad_mensual,
                precio_unitario: item.precio_unitario,
                costo_mensual: item.costo_mensual,
                tienda: item.tienda.clone(),
                categoria: item.categoria.clone(),
            })
            .collect();

        Ok(inventario_hogar_tauri::ResumenResponse {
            total_mensual: cfg.hogar.total_mensual_aproximado,
            total_items: cfg.items.len(),
            gasto_por_categoria,
            top_items,
        })
    } else {
        Ok(inventario_hogar_tauri::ResumenResponse {
            total_mensual: 0,
            total_items: 0,
            gasto_por_categoria: vec![],
            top_items: vec![],
        })
    }
}

#[tauri::command]
fn add_item(
    state: tauri::State<AppState>,
    producto: String,
    cantidadMensual: f64,      // ← camelCase
    precioUnitario: u64,        // ← camelCase
    tienda: String,
    categoria: String,
) -> Result<inventario_hogar_tauri::ItemResponse, String> {
    let mut config = state.config.lock().unwrap();
    let cfg = config.as_mut().ok_or("Configuración no inicializada")?;

    let item = inventario_hogar::Item::new(producto, cantidadMensual, precioUnitario, tienda)
        .with_categoria(categoria);

    let item_clone = item.clone();
    cfg.agregar_item(item)?;

    let json_path = inventario_hogar_tauri::get_json_path();
    inventario_hogar::guardar_a_json(cfg, &json_path)?;

    Ok(inventario_hogar_tauri::ItemResponse {
        id: item_clone.id.to_string(),
        producto: item_clone.producto,
        cantidad_mensual: item_clone.cantidad_mensual,
        precio_unitario: item_clone.precio_unitario,
        costo_mensual: item_clone.costo_mensual,
        tienda: item_clone.tienda,
        categoria: item_clone.categoria,
    })
}

#[tauri::command]
fn delete_item(
    state: tauri::State<AppState>,
    id: String,
) -> Result<(), String> {
    let uuid = uuid::Uuid::parse_str(&id).map_err(|e| format!("ID inválido: {}", e))?;

    let mut config = state.config.lock().unwrap();
    let cfg = config.as_mut().ok_or("Configuración no inicializada")?;

    cfg.eliminar_item(uuid)?;

    let json_path = inventario_hogar_tauri::get_json_path();
    inventario_hogar::guardar_a_json(cfg, &json_path)?;

    Ok(())
}