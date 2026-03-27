use inventario_hogar::{ConfiguracionInventario, RutasApp};
use std::sync::Mutex;
use std::path::PathBuf;

pub struct AppState {
    pub config: Mutex<Option<ConfiguracionInventario>>,
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

pub fn get_json_path() -> PathBuf {
    let rutas = RutasApp::new("inventario_hogar");
    rutas.ruta_json()
}