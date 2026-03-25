use std::path::PathBuf;

#[derive(Debug, Clone, Copy, PartialEq)]
pub enum Plataforma {
    Linux,
    Android,
    Desconocida,
}

impl Plataforma {
    pub fn actual() -> Self {
        if cfg!(target_os = "android") {
            Plataforma::Android
        } else if cfg!(target_os = "linux") {
            Plataforma::Linux
        } else {
            Plataforma::Desconocida
        }
    }

    pub fn nombre(&self) -> &'static str {
        match self {
            Plataforma::Linux => "Linux (Manjaro)",
            Plataforma::Android => "Android",
            Plataforma::Desconocida => "Desconocida",
        }
    }
}

pub struct RutasApp {
    plataforma: Plataforma,
    directorio_datos: PathBuf,
    directorio_config: PathBuf,
}

impl RutasApp {
    pub fn new(nombre_app: &str) -> Self {
        let plataforma = Plataforma::actual();

        let (datos_dir, config_dir) = match plataforma {
            Plataforma::Linux => {
                let home = dirs::home_dir().expect("No se pudo encontrar el home");
                let datos = home.join(".local").join("share").join(nombre_app);
                let config = home.join(".config").join(nombre_app);
                (datos, config)
            },
            Plataforma::Android => {
                let data_local = dirs::data_local_dir().expect("No se pudo encontrar data local");
                let datos = data_local.join(nombre_app);
                let config = data_local.join(nombre_app).join("config");
                (datos, config)
            },
            Plataforma::Desconocida => {
                let datos = PathBuf::from(".").join(nombre_app);
                let config = PathBuf::from(".").join(nombre_app).join("config");
                (datos, config)
            }
        };

        Self {
            plataforma,
            directorio_datos: datos_dir,
            directorio_config: config_dir,
        }
    }

    pub fn crear_directorios(&self) -> Result<(), String> {
        std::fs::create_dir_all(&self.directorio_datos)
            .map_err(|e| format!("Error al crear directorio de datos: {}", e))?;
        std::fs::create_dir_all(&self.directorio_config)
            .map_err(|e| format!("Error al crear directorio de config: {}", e))?;
        Ok(())
    }

    pub fn ruta_json(&self) -> PathBuf {
        self.directorio_datos.join("inventario.json")
    }

    pub fn plataforma(&self) -> Plataforma {
        self.plataforma
    }

    /// Obtener el directorio de datos
    pub fn directorio_datos(&self) -> &PathBuf {
        &self.directorio_datos
    }

    /// Obtener el directorio de configuración
    pub fn directorio_config(&self) -> &PathBuf {
        &self.directorio_config
    }

    pub fn mostrar_info(&self) {
        println!("📱 Plataforma detectada: {}", self.plataforma.nombre());
        println!("📁 Directorio de datos: {:?}", self.directorio_datos);
        println!("⚙️  Directorio de configuración: {:?}", self.directorio_config);
        println!("📄 Archivo JSON: {:?}", self.ruta_json());
    }
}