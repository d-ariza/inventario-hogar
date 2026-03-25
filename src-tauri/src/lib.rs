use tauri::Manager;
use inventario_hogar::{ConfiguracionInventario, Item, RutasApp};
use std::sync::Mutex;
use std::path::PathBuf;

// Estado global de la aplicación
pub struct AppState {
    pub config: Mutex<Option<ConfiguracionInventario>>,
    pub app_name: String,
}

#[derive(Clone, serde::Serialize)]
pub struct ItemResponse {
    pub id: String,
    pub producto: String,
    pub cantidad_mensual: f64,
    pub precio_unitario: u64,
    pub costo_mensual: u64,
    pub tienda: String,
    pub categoria: String,
}

#[derive(Clone, serde::Serialize)]
pub struct ResumenResponse {
    pub total_mensual: u64,
    pub total_items: usize,
    pub gasto_por_categoria: Vec<(String, u64)>,
    pub top_items: Vec<ItemResponse>,
}

impl From<&inventario_hogar::Item> for ItemResponse {
    fn from(item: &inventario_hogar::Item) -> Self {
        Self {
            id: item.id.to_string(),
            producto: item.producto.clone(),
            cantidad_mensual: item.cantidad_mensual,
            precio_unitario: item.precio_unitario,
            costo_mensual: item.costo_mensual,
            tienda: item.tienda.clone(),
            categoria: item.categoria.clone(),
        }
    }
}

// Obtener la ruta de datos de la aplicación usando Tauri
fn get_data_path(app_handle: &tauri::AppHandle) -> PathBuf {
    app_handle.path().app_data_dir().unwrap_or_else(|_| {
        // Fallback a nuestro RutasApp si Tauri no puede
        let rutas = RutasApp::new("inventario_hogar");
        rutas.ruta_json().parent().unwrap().to_path_buf()
    })
}

// Cargar o crear configuración
fn load_config(app_handle: &tauri::AppHandle) -> Result<ConfiguracionInventario, String> {
    let data_dir = get_data_path(app_handle);
    let json_path = data_dir.join("inventario.json");

    println!("📂 Cargando desde: {:?}", json_path);

    if json_path.exists() {
        inventario_hogar::cargar_desde_json(&json_path)
    } else {
        let mut config = inventario_hogar::crear_ejemplo_aseo();
        // Guardar en la ruta correcta
        if let Err(e) = inventario_hogar::guardar_a_json(&config, &json_path) {
            eprintln!("Error al guardar configuración inicial: {}", e);
        }
        Ok(config)
    }
}

// Comandos Tauri

#[tauri::command]
async fn get_items(state: tauri::State<'_, AppState>) -> Result<Vec<ItemResponse>, String> {
    let config = state.config.lock().unwrap();
    if let Some(ref cfg) = *config {
        Ok(cfg.items.iter().map(ItemResponse::from).collect())
    } else {
        Ok(vec![])
    }
}

#[tauri::command]
async fn get_resumen(state: tauri::State<'_, AppState>) -> Result<ResumenResponse, String> {
    let config = state.config.lock().unwrap();
    if let Some(ref cfg) = *config {
        let gasto_por_categoria: Vec<(String, u64)> = cfg.gasto_por_categoria().into_iter().collect();
        let top_items: Vec<ItemResponse> = cfg.top_items_mas_caros(5)
            .iter()
            .map(|item| item.into())
            .collect();

        Ok(ResumenResponse {
            total_mensual: cfg.hogar.total_mensual_aproximado,
            total_items: cfg.items.len(),
            gasto_por_categoria,
            top_items,
        })
    } else {
        Ok(ResumenResponse {
            total_mensual: 0,
            total_items: 0,
            gasto_por_categoria: vec![],
            top_items: vec![],
        })
    }
}

#[tauri::command]
async fn add_item(
    state: tauri::State<'_, AppState>,
    app_handle: tauri::AppHandle,
    producto: String,
    cantidad_mensual: f64,
    precio_unitario: u64,
    tienda: String,
    categoria: String,
) -> Result<ItemResponse, String> {
    let mut config = state.config.lock().unwrap();
    let cfg = config.as_mut().ok_or("Configuración no inicializada")?;

    let item = inventario_hogar::Item::new(producto, cantidad_mensual, precio_unitario, tienda)
        .with_categoria(categoria);

    let item_clone = item.clone();
    cfg.agregar_item(item)?;

    // Guardar cambios
    let data_dir = get_data_path(&app_handle);
    let json_path = data_dir.join("inventario.json");
    inventario_hogar::guardar_a_json(cfg, &json_path)?;

    Ok((&item_clone).into())
}

#[tauri::command]
async fn delete_item(
    state: tauri::State<'_, AppState>,
    app_handle: tauri::AppHandle,
    id: String,
) -> Result<(), String> {
    let uuid = uuid::Uuid::parse_str(&id).map_err(|e| format!("ID inválido: {}", e))?;

    let mut config = state.config.lock().unwrap();
    let cfg = config.as_mut().ok_or("Configuración no inicializada")?;

    cfg.eliminar_item(uuid)?;

    // Guardar cambios
    let data_dir = get_data_path(&app_handle);
    let json_path = data_dir.join("inventario.json");
    inventario_hogar::guardar_a_json(cfg, &json_path)?;

    Ok(())
}

#[tauri::command]
async fn update_item(
    state: tauri::State<'_, AppState>,
    app_handle: tauri::AppHandle,
    id: String,
    cantidad_mensual: Option<f64>,
    precio_unitario: Option<u64>,
    tienda: Option<String>,
) -> Result<(), String> {
    let uuid = uuid::Uuid::parse_str(&id).map_err(|e| format!("ID inválido: {}", e))?;

    let mut config = state.config.lock().unwrap();
    let cfg = config.as_mut().ok_or("Configuración no inicializada")?;

    cfg.actualizar_item(uuid, |item| {
        if let Some(cantidad) = cantidad_mensual {
            item.set_cantidad_mensual(cantidad);
        }
        if let Some(precio) = precio_unitario {
            item.set_precio_unitario(precio);
        }
        if let Some(tienda_nueva) = tienda {
            item.tienda = tienda_nueva;
        }
    })?;

    // Guardar cambios
    let data_dir = get_data_path(&app_handle);
    let json_path = data_dir.join("inventario.json");
    inventario_hogar::guardar_a_json(cfg, &json_path)?;

    Ok(())
}

#[tauri::command]
async fn buscar_items(
    state: tauri::State<'_, AppState>,
    query: String,
) -> Result<Vec<ItemResponse>, String> {
    let config = state.config.lock().unwrap();
    if let Some(ref cfg) = *config {
        Ok(cfg.buscar_por_nombre(&query).iter().map(|item| item.into()).collect())
    } else {
        Ok(vec![])
    }
}