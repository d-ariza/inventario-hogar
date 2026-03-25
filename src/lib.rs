mod config;
mod hogar;
mod item;
mod rutas;
mod utils;

pub use config::ConfiguracionInventario;
pub use hogar::Hogar;
pub use item::Item;
pub use rutas::{RutasApp, Plataforma};
pub use utils::{formatear_numero, formatear_moneda, guardar_atomico, crear_backup, limpiar_backups_antiguos};

use std::fs;
use std::path::Path;
use uuid::Uuid;

pub fn cargar_desde_json(path: &Path) -> Result<ConfiguracionInventario, String> {
    let contenido = fs::read_to_string(path)
        .map_err(|e| format!("No se pudo leer el archivo {:?}: {}", path, e))?;

    let config: ConfiguracionInventario = serde_json::from_str(&contenido)
        .map_err(|e| format!("Error al parsear JSON: {}", e))?;

    config.validar()?;

    Ok(config)
}

pub fn guardar_a_json(config: &ConfiguracionInventario, path: &Path) -> Result<(), String> {
    config.validar()?;

    let contenido = serde_json::to_string_pretty(config)
        .map_err(|e| format!("Error al serializar a JSON: {}", e))?;

    if let Some(parent) = path.parent() {
        fs::create_dir_all(parent)
            .map_err(|e| format!("No se pudo crear el directorio {:?}: {}", parent, e))?;
    }

    guardar_atomico(&contenido, path)?;

    Ok(())
}

pub fn guardar_con_backup(config: &ConfiguracionInventario, path: &Path, backups_dir: &Path) -> Result<(), String> {
    // Crear backup antes de guardar
    if path.exists() {
        crear_backup(path, backups_dir)?;
    }

    // Guardar con escritura atómica
    guardar_a_json(config, path)?;

    // Limpiar backups antiguos (mantener últimos 5)
    limpiar_backups_antiguos(backups_dir, 5)?;

    Ok(())
}

pub fn crear_ejemplo_aseo() -> ConfiguracionInventario {
    let hogar = Hogar::new(2, 30, "aseo_y_bienestar".to_string(), "COP".to_string());

    let items = vec![
        Item::new("Shampoo Maria Salome sin sal (ella)".to_string(), 0.5, 26000, "Zapatoca".to_string()).with_categoria("higiene".to_string()),
        Item::new("Shampoo Maria Salome anticaida (el)".to_string(), 0.5, 25000, "Zapatoca".to_string()).with_categoria("higiene".to_string()),
        Item::new("Jabon corporal liquido Lactovit".to_string(), 1.0, 26000, "Ara".to_string()).with_categoria("higiene".to_string()),
        Item::new("Crema dental".to_string(), 1.0, 4000, "Ara".to_string()).with_categoria("higiene".to_string()),
        Item::new("Cepillos de dientes".to_string(), 0.67, 6000, "Ara".to_string()).with_categoria("higiene".to_string()),
        Item::new("Desodorante".to_string(), 2.0, 7000, "Ara".to_string()).with_categoria("higiene".to_string()),
        Item::new("Papel higienico".to_string(), 1.0, 15000, "Ara".to_string()).with_categoria("hogar".to_string()),
        Item::new("Servilletas".to_string(), 1.0, 3000, "D1".to_string()).with_categoria("hogar".to_string()),
        Item::new("Detergente liquido (3L)".to_string(), 1.0, 12600, "D1".to_string()).with_categoria("limpieza".to_string()),
        Item::new("Suavizante ropa".to_string(), 1.0, 10000, "D1".to_string()).with_categoria("limpieza".to_string()),
        Item::new("Arroz".to_string(), 2.0, 4000, "Ara".to_string()).with_categoria("granos".to_string()),
        Item::new("Frijoles".to_string(), 1.5, 5000, "D1".to_string()).with_categoria("granos".to_string()),
    ];

    let mut config = ConfiguracionInventario::new(hogar, items);
    config.recalcular_totales();
    config
}

pub fn cargar_o_crear_config(nombre_app: &str) -> Result<ConfiguracionInventario, String> {
    let rutas = RutasApp::new(nombre_app);

    println!("🔧 Inicializando app en: {}", rutas.plataforma().nombre());
    rutas.mostrar_info();

    rutas.crear_directorios()?;

    let json_path = rutas.ruta_json();

    if json_path.exists() {
        println!("📂 Cargando configuración existente desde: {:?}", json_path);
        cargar_desde_json(&json_path)
    } else {
        println!("🆕 Creando nueva configuración de ejemplo");
        let config = crear_ejemplo_aseo();
        guardar_a_json(&config, &json_path)?;
        println!("✅ Configuración guardada en: {:?}", json_path);
        Ok(config)
    }
}