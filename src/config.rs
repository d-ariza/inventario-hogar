use serde::{Deserialize, Serialize};
use crate::hogar::Hogar;
use crate::item::Item;
use std::collections::HashMap;
use fuzzy_matcher::FuzzyMatcher;
use fuzzy_matcher::skim::SkimMatcherV2;

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub struct ConfiguracionInventario {
    pub hogar: Hogar,
    pub items: Vec<Item>,
    pub version: String,
    pub ultima_actualizacion: Option<String>,
}

impl ConfiguracionInventario {
    pub fn new(hogar: Hogar, items: Vec<Item>) -> Self {
        Self {
            hogar,
            items,
            version: env!("CARGO_PKG_VERSION").to_string(),
            ultima_actualizacion: Some(chrono::Utc::now().to_rfc3339()),
        }
    }

    // ========== OPERACIONES CRUD ==========

    pub fn agregar_item(&mut self, item: Item) -> Result<(), String> {
        if item.producto.is_empty() {
            return Err("El producto no puede estar vacío".to_string());
        }
        if item.cantidad_mensual <= 0.0 {
            return Err("La cantidad mensual debe ser positiva".to_string());
        }
        if item.precio_unitario == 0 {
            return Err("El precio unitario no puede ser 0".to_string());
        }

        self.items.push(item);
        self.recalcular_totales();
        self.actualizar_timestamp();
        Ok(())
    }

    pub fn eliminar_item(&mut self, id: uuid::Uuid) -> Result<Item, String> {
        let pos = self.items.iter().position(|item| item.id == id)
            .ok_or("Item no encontrado")?;
        let item_eliminado = self.items.remove(pos);
        self.recalcular_totales();
        self.actualizar_timestamp();
        Ok(item_eliminado)
    }

    pub fn actualizar_item(&mut self, id: uuid::Uuid, actualizacion: impl FnOnce(&mut Item)) -> Result<(), String> {
        let item = self.items.iter_mut().find(|item| item.id == id)
            .ok_or("Item no encontrado")?;
        actualizacion(item);
        item.recalcular_costo();
        self.recalcular_totales();
        self.actualizar_timestamp();
        Ok(())
    }

    pub fn obtener_item(&self, id: uuid::Uuid) -> Option<&Item> {
        self.items.iter().find(|item| item.id == id)
    }

    pub fn obtener_item_mut(&mut self, id: uuid::Uuid) -> Option<&mut Item> {
        self.items.iter_mut().find(|item| item.id == id)
    }

    // ========== BÚSQUEDA Y FILTRADO ==========

    pub fn buscar_por_nombre(&self, query: &str) -> Vec<&Item> {
        let matcher = SkimMatcherV2::default();
        let mut resultados: Vec<(&Item, i64)> = self.items
            .iter()
            .filter_map(|item| {
                matcher.fuzzy_match(&item.producto, query)
                    .map(|score| (item, score))
            })
            .collect();

        resultados.sort_by(|a, b| b.1.cmp(&a.1));
        resultados.into_iter().map(|(item, _)| item).collect()
    }

    pub fn filtrar_por_tienda(&self, tienda: &str) -> Vec<&Item> {
        self.items.iter()
            .filter(|item| item.tienda.eq_ignore_ascii_case(tienda))
            .collect()
    }

    pub fn filtrar_por_categoria(&self, categoria: &str) -> Vec<&Item> {
        self.items.iter()
            .filter(|item| item.categoria.eq_ignore_ascii_case(categoria))
            .collect()
    }

    pub fn filtrar_por_precio(&self, min: u64, max: u64) -> Vec<&Item> {
        self.items.iter()
            .filter(|item| item.precio_unitario >= min && item.precio_unitario <= max)
            .collect()
    }

    // ========== ANÁLISIS Y REPORTES ==========

    /// Agrupar items por tienda
    pub fn items_por_tienda(&self) -> HashMap<String, Vec<&Item>> {
        let mut grupos = HashMap::new();
        for item in &self.items {
            grupos.entry(item.tienda.clone()).or_insert_with(Vec::new).push(item);
        }
        grupos
    }

    /// Obtener total por tienda
    pub fn total_por_tienda(&self) -> HashMap<String, u64> {
        let mut totales = HashMap::new();
        for item in &self.items {
            *totales.entry(item.tienda.clone()).or_insert(0) += item.costo_mensual;
        }
        totales
    }

    pub fn gasto_por_categoria(&self) -> HashMap<String, u64> {
        let mut gastos = HashMap::new();
        for item in &self.items {
            *gastos.entry(item.categoria.clone()).or_insert(0) += item.costo_mensual;
        }
        gastos
    }

    pub fn mejor_tienda_para_producto(&self, nombre_producto: &str) -> Option<(&Item, &Item)> {
        let productos: Vec<&Item> = self.items.iter()
            .filter(|item| item.producto.to_lowercase().contains(&nombre_producto.to_lowercase()))
            .collect();

        if productos.is_empty() {
            return None;
        }

        let mas_barato = productos.iter()
            .min_by_key(|item| item.precio_unitario)?;

        let mas_caro = productos.iter()
            .max_by_key(|item| item.precio_unitario)?;

        Some((*mas_barato, *mas_caro))
    }

    pub fn dias_restantes(&self, item_id: uuid::Uuid) -> Option<f64> {
        let item = self.obtener_item(item_id)?;
        if item.cantidad_mensual <= 0.0 {
            return Some(0.0);
        }
        let consumo_diario = item.cantidad_mensual / self.hogar.periodo_dias as f64;
        if consumo_diario <= 0.0 {
            return Some(f64::INFINITY);
        }
        Some(item.cantidad_mensual / consumo_diario)
    }

    pub fn top_items_mas_caros(&self, n: usize) -> Vec<&Item> {
        let mut items: Vec<&Item> = self.items.iter().collect();
        items.sort_by(|a, b| b.costo_mensual.cmp(&a.costo_mensual));
        items.into_iter().take(n).collect()
    }

    pub fn resumen_completo(&self) -> String {
        let mut resumen = String::new();
        resumen.push_str(&format!("🏠 Hogar: {} personas, {} días\n", self.hogar.personas, self.hogar.periodo_dias));
        resumen.push_str(&format!("💰 Total mensual: ${}\n", self.hogar.total_mensual_aproximado));
        resumen.push_str(&format!("📊 Total items: {}\n", self.items.len()));

        resumen.push_str("\n📦 Gasto por categoría:\n");
        for (categoria, total) in self.gasto_por_categoria() {
            resumen.push_str(&format!("  • {}: ${}\n", categoria, total));
        }

        resumen
    }

    // ========== MÉTODOS PÚBLICOS PARA RECÁLCULO ==========

    /// Recalcular todos los totales del hogar
    pub fn recalcular_totales(&mut self) {
        let total: u64 = self.items.iter().map(|item| item.costo_mensual).sum();
        self.hogar.calcular_totales(total);
    }

    fn actualizar_timestamp(&mut self) {
        self.ultima_actualizacion = Some(chrono::Utc::now().to_rfc3339());
    }

    // ========== VALIDACIÓN ==========

    pub fn validar(&self) -> Result<(), String> {
        if self.hogar.personas == 0 {
            return Err("El número de personas debe ser mayor a 0".to_string());
        }

        if self.hogar.periodo_dias == 0 {
            return Err("El período de días debe ser mayor a 0".to_string());
        }

        for (i, item) in self.items.iter().enumerate() {
            if item.producto.is_empty() {
                return Err(format!("Item {}: El producto no puede estar vacío", i));
            }

            if item.cantidad_mensual <= 0.0 {
                return Err(format!("Item {}: La cantidad mensual debe ser positiva", i));
            }

            if item.precio_unitario == 0 {
                return Err(format!("Item {}: El precio unitario no puede ser 0", i));
            }

            if item.tienda.is_empty() {
                return Err(format!("Item {}: La tienda no puede estar vacía", i));
            }
        }

        Ok(())
    }
}