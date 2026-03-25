use std::fs;
use std::path::{Path, PathBuf};
use chrono::Local;

/// Formatear un número con separadores de miles
pub fn formatear_numero(num: u64) -> String {
    let num_str = num.to_string();
    let chars: Vec<char> = num_str.chars().rev().collect();
    let mut resultado = Vec::new();

    for (i, ch) in chars.iter().enumerate() {
        if i > 0 && i % 3 == 0 {
            resultado.push(',');
        }
        resultado.push(*ch);
    }

    resultado.into_iter().rev().collect()
}

/// Formatear un número con símbolo de moneda
pub fn formatear_moneda(num: u64, moneda: &str) -> String {
    match moneda {
        "COP" => format!("${} COP", formatear_numero(num)),
        "USD" => format!("${} USD", formatear_numero(num)),
        "EUR" => format!("€{} EUR", formatear_numero(num)),
        _ => format!("{} {}", formatear_numero(num), moneda),
    }
}

/// Guardar archivo con escritura atómica (tmp + rename)
pub fn guardar_atomico(contenido: &str, path: &Path) -> Result<(), String> {
    let tmp_path = path.with_extension("tmp");

    // Escribir en archivo temporal
    fs::write(&tmp_path, contenido)
        .map_err(|e| format!("Error al escribir archivo temporal: {}", e))?;

    // Renombrar (operación atómica en la mayoría de sistemas)
    fs::rename(&tmp_path, path)
        .map_err(|e| format!("Error al renombrar archivo: {}", e))?;

    Ok(())
}

/// Crear backup del archivo JSON
pub fn crear_backup(ruta_json: &Path, backups_dir: &Path) -> Result<PathBuf, String> {
    if !ruta_json.exists() {
        return Err("El archivo JSON no existe".to_string());
    }

    // Crear directorio de backups si no existe
    fs::create_dir_all(backups_dir)
        .map_err(|e| format!("Error al crear directorio de backups: {}", e))?;

    // Generar nombre de backup con timestamp
    let timestamp = Local::now().format("%Y%m%d_%H%M%S");
    let backup_name = format!("inventario_backup_{}.json", timestamp);
    let backup_path = backups_dir.join(backup_name);

    // Copiar archivo
    fs::copy(ruta_json, &backup_path)
        .map_err(|e| format!("Error al crear backup: {}", e))?;

    println!("✅ Backup creado: {:?}", backup_path);
    Ok(backup_path)
}

/// Limpiar backups antiguos (mantener solo los últimos N)
pub fn limpiar_backups_antiguos(backups_dir: &Path, mantener: usize) -> Result<(), String> {
    let mut backups: Vec<PathBuf> = fs::read_dir(backups_dir)
        .map_err(|e| format!("Error al leer directorio de backups: {}", e))?
        .filter_map(|entry| {
            let entry = entry.ok()?;
            let path = entry.path();
            if path.extension().and_then(|ext| ext.to_str()) == Some("json") {
                Some(path)
            } else {
                None
            }
        })
        .collect();

    // Ordenar por fecha de modificación (más antiguos primero)
    backups.sort_by_key(|path| fs::metadata(path).and_then(|m| m.modified()).ok());

    // Eliminar backups excedentes
    let eliminar = backups.len().saturating_sub(mantener);
    for backup in backups.iter().take(eliminar) {
        fs::remove_file(backup)
            .map_err(|e| format!("Error al eliminar backup antiguo {:?}: {}", backup, e))?;
        println!("🗑️  Backup antiguo eliminado: {:?}", backup);
    }

    Ok(())
}

#[cfg(test)]
mod tests {
    use super::*;
    use tempfile::tempdir;

    #[test]
    fn test_formatear_numero() {
        assert_eq!(formatear_numero(1000), "1,000");
        assert_eq!(formatear_numero(219320), "219,320");
        assert_eq!(formatear_numero(1000000), "1,000,000");
    }

    #[test]
    fn test_formatear_moneda() {
        assert_eq!(formatear_moneda(219320, "COP"), "$219,320 COP");
        assert_eq!(formatear_moneda(100, "USD"), "$100 USD");
    }

    #[test]
    fn test_guardado_atomico() {
        let dir = tempdir().unwrap();
        let path = dir.path().join("test.json");

        guardar_atomico("contenido de prueba", &path).unwrap();
        assert!(path.exists());
        assert!(!path.with_extension("tmp").exists());

        let contenido = fs::read_to_string(&path).unwrap();
        assert_eq!(contenido, "contenido de prueba");
    }
}