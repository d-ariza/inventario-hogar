use inventario_hogar::{
    cargar_o_crear_config, guardar_con_backup, formatear_moneda,
    ConfiguracionInventario, Item, RutasApp
};
use std::io::{self, Write};
use uuid::Uuid;

fn main() -> Result<(), String> {
    println!("=== 🏠 Gestor Interactivo de Inventario Hogar ===\n");

    let rutas = RutasApp::new("inventario_hogar");
    let backups_dir = rutas.directorio_datos().join("backups");

    let mut config = cargar_o_crear_config("inventario_hogar")?;

    loop {
        mostrar_menu();

        let opcion = leer_input("Seleccione una opción: ");

        match opcion.trim() {
            "1" => listar_items(&config),
            "2" => agregar_item(&mut config),
            "3" => eliminar_item(&mut config),
            "4" => actualizar_item(&mut config),
            "5" => buscar_items(&config),
            "6" => mostrar_reportes(&config),
            "7" => mostrar_analisis(&config),
            "8" => {
                println!("\n💾 Guardando configuración...");
                guardar_con_backup(&config, &rutas.ruta_json(), &backups_dir)?;
                println!("✅ Configuración guardada con backup\n");
            }
            "9" => {
                println!("\n👋 ¡Hasta luego!\n");
                break;
            }
            _ => println!("\n❌ Opción inválida\n"),
        }
    }

    Ok(())
}

fn mostrar_menu() {
    println!("\n╔════════════════════════════════════╗");
    println!("║        MENÚ PRINCIPAL              ║");
    println!("╠════════════════════════════════════╣");
    println!("║ 1. 📋 Listar items                 ║");
    println!("║ 2. ➕ Agregar item                 ║");
    println!("║ 3. ❌ Eliminar item                ║");
    println!("║ 4. ✏️  Actualizar item              ║");
    println!("║ 5. 🔍 Buscar items                 ║");
    println!("║ 6. 📊 Reportes                     ║");
    println!("║ 7. 📈 Análisis                     ║");
    println!("║ 8. 💾 Guardar cambios              ║");
    println!("║ 9. 🚪 Salir                        ║");
    println!("╚════════════════════════════════════╝");
}

fn leer_input(prompt: &str) -> String {
    print!("{}", prompt);
    io::stdout().flush().unwrap();
    let mut input = String::new();
    io::stdin().read_line(&mut input).unwrap();
    input.trim().to_string()
}

fn listar_items(config: &ConfiguracionInventario) {
    println!("\n📋 LISTA DE ITEMS");
    println!("{}", "=".repeat(50));

    if config.items.is_empty() {
        println!("No hay items registrados\n");
        return;
    }

    for (i, item) in config.items.iter().enumerate() {
        println!("{}. {} [{}]", i + 1, item.producto, item.tienda);
        println!("   📦 Cantidad mensual: {:.2} unidades", item.cantidad_mensual);
        println!("   💰 Precio unitario: ${}", item.precio_unitario);
        println!("   💵 Costo mensual: ${}", item.costo_mensual);
        println!("   🏷️  Categoría: {}", item.categoria);
        println!("   🆔 ID: {}", item.id);
        println!("{}", "-".repeat(40));
    }

    println!("\n💰 TOTAL MENSUAL: ${}\n", config.hogar.total_mensual_aproximado);
}

fn agregar_item(config: &mut ConfiguracionInventario) {
    println!("\n➕ AGREGAR NUEVO ITEM");
    println!("{}", "=".repeat(50));

    let producto = leer_input("Nombre del producto: ");
    if producto.is_empty() {
        println!("❌ El nombre no puede estar vacío\n");
        return;
    }

    let cantidad: f64 = loop {
        let input = leer_input("Cantidad mensual (ej: 0.5, 1, 2): ");
        match input.parse() {
            Ok(n) if n > 0.0 => break n,
            _ => println!("❌ Cantidad inválida, debe ser un número positivo"),
        }
    };

    let precio: u64 = loop {
        let input = leer_input("Precio unitario (COP): ");
        match input.parse() {
            Ok(n) if n > 0 => break n,
            _ => println!("❌ Precio inválido, debe ser un número positivo"),
        }
    };

    let tienda = leer_input("Tienda: ");
    if tienda.is_empty() {
        println!("❌ La tienda no puede estar vacía\n");
        return;
    }

    let categoria = leer_input("Categoría (higiene/hogar/limpieza/granos): ");
    let categoria = if categoria.is_empty() { "general".to_string() } else { categoria };

    let item = Item::new(producto, cantidad, precio, tienda)
        .with_categoria(categoria);

    match config.agregar_item(item) {
        Ok(()) => println!("✅ Item agregado correctamente"),
        Err(e) => println!("❌ Error: {}", e),
    }
}

fn eliminar_item(config: &mut ConfiguracionInventario) {
    println!("\n❌ ELIMINAR ITEM");
    println!("{}", "=".repeat(50));

    if config.items.is_empty() {
        println!("No hay items para eliminar\n");
        return;
    }

    listar_items(config);

    let id_str = leer_input("ID del item a eliminar (o 'cancelar'): ");
    if id_str == "cancelar" {
        return;
    }

    match Uuid::parse_str(&id_str) {
        Ok(id) => {
            match config.eliminar_item(id) {
                Ok(item) => println!("✅ Item eliminado: {}", item.producto),
                Err(e) => println!("❌ Error: {}", e),
            }
        }
        Err(_) => println!("❌ ID inválido"),
    }
}

fn actualizar_item(config: &mut ConfiguracionInventario) {
    println!("\n✏️  ACTUALIZAR ITEM");
    println!("{}", "=".repeat(50));

    if config.items.is_empty() {
        println!("No hay items para actualizar\n");
        return;
    }

    listar_items(config);

    let id_str = leer_input("ID del item a actualizar (o 'cancelar'): ");
    if id_str == "cancelar" {
        return;
    }

    let id = match Uuid::parse_str(&id_str) {
        Ok(id) => id,
        Err(_) => {
            println!("❌ ID inválido");
            return;
        }
    };

    println!("\n📝 Deje en blanco para mantener el valor actual\n");

    let nueva_cantidad: Option<f64> = {
        let input = leer_input("Nueva cantidad mensual: ");
        if input.is_empty() { None } else { input.parse().ok() }
    };

    let nuevo_precio: Option<u64> = {
        let input = leer_input("Nuevo precio unitario: ");
        if input.is_empty() { None } else { input.parse().ok() }
    };

    let nueva_tienda = leer_input("Nueva tienda: ");
    let nueva_tienda = if nueva_tienda.is_empty() { None } else { Some(nueva_tienda) };

    match config.actualizar_item(id, |item| {
        if let Some(cantidad) = nueva_cantidad {
            item.set_cantidad_mensual(cantidad);
        }
        if let Some(precio) = nuevo_precio {
            item.set_precio_unitario(precio);
        }
        if let Some(tienda) = nueva_tienda {
            item.tienda = tienda;
        }
    }) {
        Ok(()) => println!("✅ Item actualizado correctamente"),
        Err(e) => println!("❌ Error: {}", e),
    }
}

fn buscar_items(config: &ConfiguracionInventario) {
    println!("\n🔍 BUSCAR ITEMS");
    println!("{}", "=".repeat(50));

    println!("1. Por nombre (búsqueda difusa)");
    println!("2. Por tienda");
    println!("3. Por categoría");
    println!("4. Por rango de precio");

    let opcion = leer_input("Opción: ");

    match opcion.as_str() {
        "1" => {
            let query = leer_input("Término de búsqueda: ");
            let resultados = config.buscar_por_nombre(&query);
            mostrar_resultados(&resultados);
        }
        "2" => {
            let tienda = leer_input("Nombre de la tienda: ");
            let resultados = config.filtrar_por_tienda(&tienda);
            mostrar_resultados(&resultados);
        }
        "3" => {
            let categoria = leer_input("Categoría: ");
            let resultados = config.filtrar_por_categoria(&categoria);
            mostrar_resultados(&resultados);
        }
        "4" => {
            let min: u64 = leer_input("Precio mínimo: ").parse().unwrap_or(0);
            let max: u64 = leer_input("Precio máximo: ").parse().unwrap_or(u64::MAX);
            let resultados = config.filtrar_por_precio(min, max);
            mostrar_resultados(&resultados);
        }
        _ => println!("❌ Opción inválida"),
    }
}

fn mostrar_resultados(resultados: &[&Item]) {
    println!("\n📋 RESULTADOS ({} items encontrados)", resultados.len());
    println!("{}", "=".repeat(50));

    for item in resultados {
        println!("• {} - {} - ${} - {}",
                 item.producto, item.tienda, item.precio_unitario, item.categoria);
    }
    println!();
}

fn mostrar_reportes(config: &ConfiguracionInventario) {
    println!("\n📊 REPORTES");
    println!("{}", "=".repeat(50));

    println!("💰 Gasto por categoría:");
    for (categoria, total) in config.gasto_por_categoria() {
        println!("  • {}: ${}", categoria, total);
    }

    println!("\n🏪 Top items más caros:");
    for (i, item) in config.top_items_mas_caros(5).iter().enumerate() {
        println!("  {}. {} - ${}", i + 1, item.producto, item.costo_mensual);
    }

    println!("\n{}", config.resumen_completo());
}

fn mostrar_analisis(config: &ConfiguracionInventario) {
    println!("\n📈 ANÁLISIS");
    println!("{}", "=".repeat(50));

    println!("🎯 Proyección de días restantes:");
    for item in &config.items {
        if let Some(dias) = config.dias_restantes(item.id) {
            if dias.is_finite() {
                println!("  • {}: {:.1} días restantes", item.producto, dias);
            } else {
                println!("  • {}: Consumo ilimitado", item.producto);
            }
        }
    }

    println!("\n🏷️  Análisis de mejores tiendas:");
    let productos_analizados: std::collections::HashSet<String> = config.items
        .iter()
        .map(|i| i.producto.clone())
        .collect();

    for producto in productos_analizados.iter().take(5) {
        if let Some((barato, caro)) = config.mejor_tienda_para_producto(producto) {
            println!("  • {}: Mejor en {} (${}) vs {} (${})",
                     producto, barato.tienda, barato.precio_unitario,
                     caro.tienda, caro.precio_unitario);
        }
    }

    println!("\n💡 Sugerencias:");
    let total = config.hogar.total_mensual_aproximado;
    let presupuesto_sugerido = (total as f64 * 1.1) as u64; // 10% de margen
    println!("  • Presupuesto sugerido: ${}", presupuesto_sugerido);

    if let Some(mas_caro) = config.top_items_mas_caros(1).first() {
        println!("  • Considera reducir consumo de: {}", mas_caro.producto);
    }
}