use serde::{Deserialize, Serialize};
use uuid::Uuid;

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub struct Item {
    pub id: Uuid,  // Añadimos ID único para identificar items
    pub producto: String,
    pub cantidad_mensual: f64,
    pub precio_unitario: u64,
    pub costo_mensual: u64,
    pub tienda: String,
    pub categoria: String,  // Añadimos categoría por item
}

impl Default for Item {
    fn default() -> Self {
        Self {
            id: Uuid::new_v4(),
            producto: String::new(),
            cantidad_mensual: 0.0,
            precio_unitario: 0,
            costo_mensual: 0,
            tienda: String::new(),
            categoria: "general".to_string(),
        }
    }
}

impl Item {
    pub fn new(producto: String, cantidad_mensual: f64, precio_unitario: u64, tienda: String) -> Self {
        let costo_mensual = (cantidad_mensual * precio_unitario as f64).round() as u64;

        Self {
            id: Uuid::new_v4(),
            producto,
            cantidad_mensual,
            precio_unitario,
            costo_mensual,
            tienda,
            categoria: "general".to_string(),
        }
    }

    pub fn with_categoria(mut self, categoria: String) -> Self {
        self.categoria = categoria;
        self
    }

    pub fn recalcular_costo(&mut self) {
        self.costo_mensual = (self.cantidad_mensual * self.precio_unitario as f64).round() as u64;
    }

    pub fn set_cantidad_mensual(&mut self, nueva_cantidad: f64) {
        self.cantidad_mensual = nueva_cantidad;
        self.recalcular_costo();
    }

    pub fn set_precio_unitario(&mut self, nuevo_precio: u64) {
        self.precio_unitario = nuevo_precio;
        self.recalcular_costo();
    }
}