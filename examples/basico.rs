use inventario_hogar::{cargar_o_crear_config, guardar_a_json, RutasApp};

fn main() {
    println!("=== Gestor de Inventario Hogar v{} ===\n", env!("CARGO_PKG_VERSION"));

    // Quitamos "mut" porque no estamos modificando config después
    let config = match cargar_o_crear_config("inventario_hogar") {
        Ok(c) => c,
        Err(e) => {
            eprintln!("❌ Error al cargar configuración: {}", e);
            return;
        }
    };

    let rutas = RutasApp::new("inventario_hogar");
    rutas.mostrar_info();
    println!();

    match config.validar() {
        Ok(()) => println!("✅ Configuración válida"),
        Err(e) => println!("⚠️  Advertencia: {}", e),
    }

    println!("\n📊 Resumen del Hogar:");
    println!("  👥 Personas: {}", config.hogar.personas);
    println!("  📅 Periodo: {} días", config.hogar.periodo_dias);
    println!("  🏷️  Categoría: {}", config.hogar.categoria);
    println!("  💰 Moneda: {}", config.hogar.moneda);
    println!("  💵 Total mensual: ${}", config.hogar.total_mensual_aproximado);
    println!("  🎯 Total redondeado: ${}\n", config.hogar.total_redondeado_decena_mil);

    println!("📦 Items por tienda:");
    let por_tienda = config.items_por_tienda();
    for (tienda, items) in por_tienda {
        let total_tienda: u64 = items.iter().map(|i| i.costo_mensual).sum();
        println!("  🏪 {}: {} items - Total: ${}", tienda, items.len(), total_tienda);

        for item in items.iter().take(3) {
            println!("     • {} - {:.2} unidad(es) - ${}",
                     item.producto,
                     item.cantidad_mensual,
                     item.costo_mensual);
        }
        if items.len() > 3 {
            println!("     ... y {} items más", items.len() - 3);
        }
        println!();
    }

    if let Err(e) = guardar_a_json(&config, &rutas.ruta_json()) {
        eprintln!("❌ Error al guardar: {}", e);
    } else {
        println!("✅ Configuración guardada en: {:?}", rutas.ruta_json());
    }
}